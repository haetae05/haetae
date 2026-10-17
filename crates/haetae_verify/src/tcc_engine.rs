#[allow(unused_imports)]
use haetae_core::config::{
    ALT_SIGNAL_STACK_SIZE_BYTES, CLANG_ENV_VAR, CLANG_FLAG_COMPILE_ONLY, CLANG_FLAG_FPIC,
    CLANG_FLAG_SHARED, CLANG_OPT_O0, CLANG_OPT_O2, CLANG_TEMP_PREFIX, COFF_FILE_HEADER_BYTES,
    COFF_NUM_SECTIONS_OFFSET, COFF_OPTHDR_SIZE_OFFSET, COFF_SECTION_HEADER_BYTES,
    COFF_SECTION_NAME_BYTES, COFF_SECTION_RAW_PTR_OFFSET, COFF_SECTION_RAW_SIZE_OFFSET,
    ELF_CLASS_64, HEX_RADIX, MAX_STACK_FRAME_BYTES, MODEL_DOWNLOAD_CHUNK_SIZE, POSIX_SIGFPE,
    POSIX_SIGSEGV, WIN_EXCEPTION_ACCESS_VIOLATION, WIN_EXCEPTION_FLT_DIVIDE_BY_ZERO,
    WIN_EXCEPTION_INT_DIVIDE_BY_ZERO, WIN_EXCEPTION_STACK_OVERFLOW,
};
#[allow(unused_imports)]
use std::ffi::c_void;
#[cfg(unix)]
use std::ffi::{CStr, CString};

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const COMPILER_ENV_VARS: &[&str] = &[CLANG_ENV_VAR, "HAETAE_CC_PATH", "CC"];
#[cfg(target_os = "windows")]
const CANDIDATE_COMPILERS: &[&str] = &["clang.exe", "clang", "gcc.exe", "gcc", "cc.exe", "cc"];
#[cfg(not(target_os = "windows"))]
const CANDIDATE_COMPILERS: &[&str] = &["clang", "gcc", "cc"];

static JIT_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("TCC initialization failed")]
    InitFailed,
    #[error("TCC compilation failed: {0}")]
    CompileError(String),
    #[error("TCC relocation failed")]
    RelocateFailed,
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),
    #[error("Stack frame exceeded: {0} > {1}")]
    StackOverflow(usize, usize),
    #[error("CFI integrity check failed: {0}")]
    CfiViolation(String),
    #[error("Execution crashed with signal: {0}")]
    CrashSignal(i32),
    #[error("Unsupported syntax: {0}")]
    UnsupportedSyntax(String),
}

#[cfg(target_os = "windows")]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const std::ffi::c_char) -> *mut c_void;
    fn FreeLibrary(hModule: *mut c_void) -> i32;
}

pub struct CompiledCode {
    pub state: *mut c_void,
    pub entry_symbol: String,
    pub relocated_size: usize,
    pub source_code: String,
    pub bytecode: Vec<u8>,
    pub module_handle: *mut c_void,
    pub safe_call_fn: Option<unsafe extern "C" fn(i64, *mut i32, *mut i32) -> i32>,
    pub safe_call_multi_fn:
        Option<unsafe extern "C" fn(*const i64, i32, *mut i32, *mut i32) -> i32>,
    pub temp_paths: Vec<PathBuf>,
}

impl Drop for CompiledCode {
    fn drop(&mut self) {
        unsafe {
            if !self.module_handle.is_null() {
                #[cfg(target_os = "windows")]
                {
                    FreeLibrary(self.module_handle);
                }
                #[cfg(unix)]
                {
                    libc::dlclose(self.module_handle);
                }
                self.module_handle = std::ptr::null_mut();
            }
            for path in &self.temp_paths {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}

unsafe impl Send for CompiledCode {}
unsafe impl Sync for CompiledCode {}

pub struct TccEngine;

impl Default for TccEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TccEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn find_c_compiler() -> Option<PathBuf> {
        for env_var in COMPILER_ENV_VARS {
            if let Ok(p) = std::env::var(env_var) {
                let trimmed = p.trim();
                if !trimmed.is_empty() {
                    let pb = PathBuf::from(trimmed);
                    if pb.is_file() {
                        return Some(pb);
                    }
                    if let Some(path_var) = std::env::var_os("PATH") {
                        for dir in std::env::split_paths(&path_var) {
                            let cand_path = dir.join(trimmed);
                            if cand_path.is_file() {
                                return Some(cand_path);
                            }
                            #[cfg(target_os = "windows")]
                            {
                                let with_exe = dir.join(format!("{}.exe", trimmed));
                                if with_exe.is_file() {
                                    return Some(with_exe);
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(path_var) = std::env::var_os("PATH") {
            for candidate in CANDIDATE_COMPILERS {
                for dir in std::env::split_paths(&path_var) {
                    let cand_path = dir.join(candidate);
                    if cand_path.is_file() {
                        return Some(cand_path);
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        let locator = "where";
        #[cfg(not(target_os = "windows"))]
        let locator = "which";

        for candidate in CANDIDATE_COMPILERS {
            if let Ok(output) = std::process::Command::new(locator).arg(candidate).output() {
                if output.status.success() {
                    if let Ok(text) = std::str::from_utf8(&output.stdout) {
                        for line in text.lines() {
                            let trimmed = line.trim();
                            if !trimmed.is_empty() {
                                let pb = PathBuf::from(trimmed);
                                if pb.is_file() {
                                    return Some(pb);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn find_clang() -> Option<PathBuf> {
        Self::find_c_compiler()
    }

    pub fn extract_coff_text_section(obj_bytes: &[u8]) -> Option<Vec<u8>> {
        if obj_bytes.len() < COFF_FILE_HEADER_BYTES {
            return None;
        }

        let num_sections_slice = obj_bytes
            .get(COFF_NUM_SECTIONS_OFFSET..COFF_NUM_SECTIONS_OFFSET + std::mem::size_of::<u16>())?;
        let num_sections = u16::from_le_bytes(num_sections_slice.try_into().ok()?) as usize;

        let opt_hdr_slice = obj_bytes
            .get(COFF_OPTHDR_SIZE_OFFSET..COFF_OPTHDR_SIZE_OFFSET + std::mem::size_of::<u16>())?;
        let opt_hdr_size = u16::from_le_bytes(opt_hdr_slice.try_into().ok()?) as usize;

        let mut offset = COFF_FILE_HEADER_BYTES + opt_hdr_size;
        for _ in 0..num_sections {
            if offset + COFF_SECTION_HEADER_BYTES > obj_bytes.len() {
                break;
            }
            let name_slice = obj_bytes.get(offset..offset + COFF_SECTION_NAME_BYTES)?;
            let raw_size_slice = obj_bytes.get(
                offset + COFF_SECTION_RAW_SIZE_OFFSET
                    ..offset + COFF_SECTION_RAW_SIZE_OFFSET + std::mem::size_of::<u32>(),
            )?;
            let raw_size = u32::from_le_bytes(raw_size_slice.try_into().ok()?) as usize;

            let raw_ptr_slice = obj_bytes.get(
                offset + COFF_SECTION_RAW_PTR_OFFSET
                    ..offset + COFF_SECTION_RAW_PTR_OFFSET + std::mem::size_of::<u32>(),
            )?;
            let raw_ptr = u32::from_le_bytes(raw_ptr_slice.try_into().ok()?) as usize;

            if name_slice.starts_with(b".text")
                && raw_size > 0
                && raw_ptr.checked_add(raw_size)? <= obj_bytes.len()
            {
                return Some(obj_bytes[raw_ptr..raw_ptr + raw_size].to_vec());
            }
            offset += COFF_SECTION_HEADER_BYTES;
        }
        None
    }

    pub fn extract_elf_text_section(obj_bytes: &[u8]) -> Option<Vec<u8>> {
        #[repr(C)]
        struct Elf64Hdr {
            e_ident: [u8; HEX_RADIX as usize],
            e_type: u16,
            e_machine: u16,
            e_version: u32,
            e_entry: u64,
            e_phoff: u64,
            e_shoff: u64,
            e_flags: u32,
            e_ehsize: u16,
            e_phentsize: u16,
            e_phnum: u16,
            e_shentsize: u16,
            e_shnum: u16,
            e_shstrndx: u16,
        }

        #[repr(C)]
        struct Elf64Shdr {
            sh_name: u32,
            sh_type: u32,
            sh_flags: u64,
            sh_addr: u64,
            sh_offset: u64,
            sh_size: u64,
            sh_link: u32,
            sh_info: u32,
            sh_addralign: u64,
            sh_entsize: u64,
        }

        if obj_bytes.len() < std::mem::size_of::<Elf64Hdr>() || !obj_bytes.starts_with(b"\x7fELF") {
            return None;
        }
        if obj_bytes.get(std::mem::size_of::<u32>()).copied() != Some(ELF_CLASS_64) {
            return None;
        }

        let shoff_offset = std::mem::offset_of!(Elf64Hdr, e_shoff);
        let shoff = u64::from_le_bytes(
            obj_bytes
                .get(shoff_offset..shoff_offset + std::mem::size_of::<u64>())?
                .try_into()
                .ok()?,
        ) as usize;

        let shentsize_offset = std::mem::offset_of!(Elf64Hdr, e_shentsize);
        let shentsize = u16::from_le_bytes(
            obj_bytes
                .get(shentsize_offset..shentsize_offset + std::mem::size_of::<u16>())?
                .try_into()
                .ok()?,
        ) as usize;

        let shnum_offset = std::mem::offset_of!(Elf64Hdr, e_shnum);
        let shnum = u16::from_le_bytes(
            obj_bytes
                .get(shnum_offset..shnum_offset + std::mem::size_of::<u16>())?
                .try_into()
                .ok()?,
        ) as usize;

        let shstrndx_offset = std::mem::offset_of!(Elf64Hdr, e_shstrndx);
        let shstrndx = u16::from_le_bytes(
            obj_bytes
                .get(shstrndx_offset..shstrndx_offset + std::mem::size_of::<u16>())?
                .try_into()
                .ok()?,
        ) as usize;

        if shstrndx >= shnum || shentsize == 0 {
            return None;
        }

        let strtab_hdr_offset = shoff.checked_add(shstrndx.checked_mul(shentsize)?)?;
        let sec_offset_offset = std::mem::offset_of!(Elf64Shdr, sh_offset);
        let sec_size_offset = std::mem::offset_of!(Elf64Shdr, sh_size);

        let strtab_offset = u64::from_le_bytes(
            obj_bytes
                .get(
                    strtab_hdr_offset + sec_offset_offset
                        ..strtab_hdr_offset + sec_offset_offset + std::mem::size_of::<u64>(),
                )?
                .try_into()
                .ok()?,
        ) as usize;
        let strtab_size = u64::from_le_bytes(
            obj_bytes
                .get(
                    strtab_hdr_offset + sec_size_offset
                        ..strtab_hdr_offset + sec_size_offset + std::mem::size_of::<u64>(),
                )?
                .try_into()
                .ok()?,
        ) as usize;

        let strtab = obj_bytes.get(strtab_offset..strtab_offset.checked_add(strtab_size)?)?;

        let sec_name_offset = std::mem::offset_of!(Elf64Shdr, sh_name);
        for i in 0..shnum {
            let sec_hdr_offset = shoff.checked_add(i.checked_mul(shentsize)?)?;
            let name_idx = u32::from_le_bytes(
                obj_bytes
                    .get(
                        sec_hdr_offset + sec_name_offset
                            ..sec_hdr_offset + sec_name_offset + std::mem::size_of::<u32>(),
                    )?
                    .try_into()
                    .ok()?,
            ) as usize;
            let sec_offset = u64::from_le_bytes(
                obj_bytes
                    .get(
                        sec_hdr_offset + sec_offset_offset
                            ..sec_hdr_offset + sec_offset_offset + std::mem::size_of::<u64>(),
                    )?
                    .try_into()
                    .ok()?,
            ) as usize;
            let sec_size = u64::from_le_bytes(
                obj_bytes
                    .get(
                        sec_hdr_offset + sec_size_offset
                            ..sec_hdr_offset + sec_size_offset + std::mem::size_of::<u64>(),
                    )?
                    .try_into()
                    .ok()?,
            ) as usize;

            if name_idx < strtab.len() {
                let name_bytes = &strtab[name_idx..];
                if (name_bytes.starts_with(b".text\0") || name_bytes.starts_with(b".text"))
                    && sec_size > 0
                    && sec_offset.checked_add(sec_size)? <= obj_bytes.len()
                {
                    return Some(obj_bytes[sec_offset..sec_offset + sec_size].to_vec());
                }
            }
        }
        None
    }

    pub fn extract_text_section(obj_bytes: &[u8]) -> Option<Vec<u8>> {
        if let Some(sec) = Self::extract_coff_text_section(obj_bytes) {
            return Some(sec);
        }
        if let Some(sec) = Self::extract_elf_text_section(obj_bytes) {
            return Some(sec);
        }
        None
    }

    pub fn compile_in_memory(
        &self,
        c_code: &str,
        entry_name: &str,
    ) -> Result<CompiledCode, VerifyError> {
        if c_code.is_empty() {
            return Err(VerifyError::CompileError("Empty C code".to_string()));
        }

        let trimmed = c_code.trim();
        if !trimmed.contains(entry_name) && !trimmed.contains("haetae_fuzz_entry") {
            return Err(VerifyError::SymbolNotFound(entry_name.to_string()));
        }

        #[cfg(any(target_os = "windows", unix))]
        {
            let compiler_path = Self::find_c_compiler().ok_or_else(|| {
                VerifyError::CompileError(
                    "C compiler not available or compilation failed: no C compiler (clang/gcc) found in PATH or environment".to_string(),
                )
            })?;
            self.compile_with_compiler(c_code, entry_name, &compiler_path)
                .map_err(|e| {
                    VerifyError::CompileError(format!(
                        "C compiler not available or compilation failed: {}",
                        e
                    ))
                })
        }

        #[cfg(not(any(target_os = "windows", unix)))]
        {
            Err(VerifyError::CompileError(
                "C compiler not available or compilation failed: platform not supported"
                    .to_string(),
            ))
        }
    }

    pub fn compile_with_clang(
        &self,
        c_code: &str,
        entry_name: &str,
        clang_path: &Path,
    ) -> Result<CompiledCode, VerifyError> {
        self.compile_with_compiler(c_code, entry_name, clang_path)
    }

    pub fn compile_with_compiler(
        &self,
        c_code: &str,
        entry_name: &str,
        compiler_path: &Path,
    ) -> Result<CompiledCode, VerifyError> {
        let seq = JIT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let temp_dir = std::env::temp_dir();
        let base_name = format!("{}_{}_{}", CLANG_TEMP_PREFIX, pid, seq);

        let c_path = temp_dir.join(format!("{}.c", base_name));
        let obj_path = temp_dir.join(format!("{}.o", base_name));

        #[cfg(target_os = "windows")]
        let lib_ext = "dll";
        #[cfg(target_os = "macos")]
        let lib_ext = "dylib";
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let lib_ext = "so";

        let lib_path = temp_dir.join(format!("{}.{}", base_name, lib_ext));
        #[cfg(target_os = "windows")]
        let exp_path = temp_dir.join(format!("{}.exp", base_name));
        #[cfg(target_os = "windows")]
        let win_lib_path = temp_dir.join(format!("{}.lib", base_name));

        let mut instrumented_c = String::new();
        instrumented_c.push_str("#ifndef _GNU_SOURCE\n#define _GNU_SOURCE\n#endif\n");
        instrumented_c.push_str(c_code);
        instrumented_c.push('\n');

        let has_fuzz_entry = c_code.contains("haetae_fuzz_entry");
        let has_fuzz_multi_entry = c_code.contains("haetae_fuzz_multi_entry");

        if !has_fuzz_entry {
            instrumented_c.push_str(&format!(
                "\nint {}(long long p);\nint haetae_fuzz_entry(long long payload) {{\n    return {}(payload);\n}}\n",
                entry_name, entry_name
            ));
        }

        if !has_fuzz_multi_entry {
            instrumented_c.push_str(
                "\nint haetae_fuzz_multi_entry(const long long* payloads, int count) {\n    if (count > 0 && payloads != 0) {\n        return haetae_fuzz_entry(payloads[0]);\n    }\n    return haetae_fuzz_entry(0);\n}\n"
            );
        }

        // Cross-platform native hardware trap execution harness
        instrumented_c.push_str(&format!(
            r#"
#ifdef _WIN32
#include <windows.h>
#include <malloc.h>

#ifndef HAETAE_SEH_DEFS_DECLARED
#define HAETAE_SEH_DEFS_DECLARED
int haetae_fuzz_entry(long long payload);
int haetae_fuzz_multi_entry(const long long* payloads, int count);

#define HAETAE_LIMIT_STACK(ref_sp) do {{ \
    ULONG_PTR low = 0, high = 0; \
    GetCurrentThreadStackLimits(&low, &high); \
    volatile char* cur_sp = (volatile char*)(ref_sp); \
    if ((ULONG_PTR)cur_sp > low) {{ \
        ULONG_PTR remaining = (ULONG_PTR)cur_sp - low; \
        ULONG_PTR max_allowed = {max_stack_bytes}; \
        ULONG_PTR margin = {stack_margin}; \
        if (remaining > max_allowed + margin) {{ \
            ULONG_PTR pad_size = remaining - max_allowed; \
            volatile char* pad = (volatile char*)_alloca(pad_size); \
            pad[0] = 0; \
        }} \
    }} \
}} while(0)

__declspec(dllexport) int haetae_safe_call(long long payload, int* out_res, int* out_signal) {{
    __try {{
        HAETAE_LIMIT_STACK(&payload);
        *out_res = haetae_fuzz_entry(payload);
        *out_signal = 0;
        return 0;
    }} __except (1) {{
        unsigned long code = GetExceptionCode();
        if (code == {stack_overflow:#X}) {{
            _resetstkoflw();
        }}
        if (code == {int_div_zero:#X} || code == {flt_div_zero:#X}) {{
            *out_signal = {sigfpe};
        }} else {{
            *out_signal = {sigsegv};
        }}
        return 1;
    }}
}}

__declspec(dllexport) int haetae_safe_call_multi(const long long* payloads, int count, int* out_res, int* out_signal) {{
    __try {{
        HAETAE_LIMIT_STACK(&payloads);
        if (count > 0 && payloads != 0) {{
            *out_res = haetae_fuzz_multi_entry(payloads, count);
        }} else {{
            *out_res = haetae_fuzz_entry(0);
        }}
        *out_signal = 0;
        return 0;
    }} __except (1) {{
        unsigned long code = GetExceptionCode();
        if (code == {stack_overflow:#X}) {{
            _resetstkoflw();
        }}
        if (code == {int_div_zero:#X} || code == {flt_div_zero:#X}) {{
            *out_signal = {sigfpe};
        }} else {{
            *out_signal = {sigsegv};
        }}
        return 1;
    }}
}}
#endif
#endif

#if defined(__unix__) || defined(__APPLE__) || (defined(__POSIX__) && !defined(_WIN32))
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <signal.h>
#include <setjmp.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <alloca.h>
#include <pthread.h>

#ifndef HAETAE_POSIX_SEH_DEFS_DECLARED
#define HAETAE_POSIX_SEH_DEFS_DECLARED
int haetae_fuzz_entry(long long payload);
int haetae_fuzz_multi_entry(const long long* payloads, int count);

static __thread sigjmp_buf haetae_sig_jmp_buf;
static __thread volatile sig_atomic_t haetae_in_safe_call = 0;
static __thread volatile sig_atomic_t haetae_caught_signal = 0;
static __thread stack_t haetae_alt_stack;
static __thread int haetae_alt_stack_initialized = 0;
static __thread char haetae_alt_stack_storage[{alt_stack_size}] __attribute__((aligned({hex_radix})));

static void haetae_posix_signal_handler(int sig, siginfo_t* info, void* ucontext) {{
    (void)info;
    (void)ucontext;
    if (haetae_in_safe_call) {{
        haetae_caught_signal = sig;
        haetae_in_safe_call = 0;
        siglongjmp(haetae_sig_jmp_buf, 1);
    }}
    struct sigaction dfl;
    memset(&dfl, 0, sizeof(dfl));
    dfl.sa_handler = SIG_DFL;
    sigaction(sig, &dfl, NULL);
    raise(sig);
}}

static inline void haetae_limit_stack_unix(volatile void* ref_sp, uintptr_t max_allowed, uintptr_t margin) {{
    uintptr_t low = 0;
    uintptr_t high = 0;
#if defined(__APPLE__)
    void* stackaddr = pthread_get_stackaddr_np(pthread_self());
    size_t stacksize = pthread_get_stacksize_np(pthread_self());
    if (stackaddr != NULL && stacksize > 0) {{
        high = (uintptr_t)stackaddr;
        low = high - (uintptr_t)stacksize;
    }}
#elif defined(__GLIBC__)
    pthread_attr_t attr;
    if (pthread_getattr_np(pthread_self(), &attr) == 0) {{
        void* stackaddr = NULL;
        size_t stacksize = 0;
        if (pthread_attr_getstack(&attr, &stackaddr, &stacksize) == 0 && stackaddr != NULL) {{
            low = (uintptr_t)stackaddr;
            high = low + (uintptr_t)stacksize;
        }}
        pthread_attr_destroy(&attr);
    }}
#endif
    if (low != 0) {{
        uintptr_t cur_sp = (uintptr_t)ref_sp;
        if (cur_sp > low) {{
            uintptr_t remaining = cur_sp - low;
            if (remaining > max_allowed + margin) {{
                uintptr_t pad_size = remaining - max_allowed;
                volatile char* pad = (volatile char*)alloca(pad_size);
                pad[0] = 0;
            }}
        }}
    }}
}}

static inline void haetae_setup_alt_stack(void) {{
    if (!haetae_alt_stack_initialized) {{
        memset(&haetae_alt_stack, 0, sizeof(haetae_alt_stack));
        haetae_alt_stack.ss_sp = haetae_alt_stack_storage;
        haetae_alt_stack.ss_size = sizeof(haetae_alt_stack_storage);
        haetae_alt_stack.ss_flags = 0;
        sigaltstack(&haetae_alt_stack, NULL);
        haetae_alt_stack_initialized = 1;
    }}
}}

__attribute__((visibility("default"))) int haetae_safe_call(long long payload, int* out_res, int* out_signal) {{
    haetae_setup_alt_stack();

    struct sigaction sa, old_fpe, old_segv, old_bus, old_ill;
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = haetae_posix_signal_handler;
    sa.sa_flags = SA_SIGINFO | SA_ONSTACK | SA_NODEFER;
    sigemptyset(&sa.sa_mask);

    sigaction(SIGFPE, &sa, &old_fpe);
    sigaction(SIGSEGV, &sa, &old_segv);
    sigaction(SIGBUS, &sa, &old_bus);
    sigaction(SIGILL, &sa, &old_ill);

    haetae_caught_signal = 0;
    haetae_in_safe_call = 1;

    int ret_status = 0;
    if (sigsetjmp(haetae_sig_jmp_buf, 1) == 0) {{
        haetae_limit_stack_unix(&payload, {max_stack_bytes}, {stack_margin});
        *out_res = haetae_fuzz_entry(payload);
        *out_signal = 0;
        ret_status = 0;
    }} else {{
        int sig = (int)haetae_caught_signal;
        if (sig == SIGBUS) {{
            sig = {sigsegv};
        }}
        *out_signal = sig;
        ret_status = 1;
    }}

    haetae_in_safe_call = 0;
    sigaction(SIGFPE, &old_fpe, NULL);
    sigaction(SIGSEGV, &old_segv, NULL);
    sigaction(SIGBUS, &old_bus, NULL);
    sigaction(SIGILL, &old_ill, NULL);

    return ret_status;
}}

__attribute__((visibility("default"))) int haetae_safe_call_multi(const long long* payloads, int count, int* out_res, int* out_signal) {{
    haetae_setup_alt_stack();

    struct sigaction sa, old_fpe, old_segv, old_bus, old_ill;
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = haetae_posix_signal_handler;
    sa.sa_flags = SA_SIGINFO | SA_ONSTACK | SA_NODEFER;
    sigemptyset(&sa.sa_mask);

    sigaction(SIGFPE, &sa, &old_fpe);
    sigaction(SIGSEGV, &old_segv, NULL);
    sigaction(SIGBUS, &old_bus, NULL);
    sigaction(SIGILL, &old_ill, NULL);

    haetae_caught_signal = 0;
    haetae_in_safe_call = 1;

    int ret_status = 0;
    if (sigsetjmp(haetae_sig_jmp_buf, 1) == 0) {{
        haetae_limit_stack_unix(&payloads, {max_stack_bytes}, {stack_margin});
        if (count > 0 && payloads != NULL) {{
            *out_res = haetae_fuzz_multi_entry(payloads, count);
        }} else {{
            *out_res = haetae_fuzz_entry(0);
        }}
        *out_signal = 0;
        ret_status = 0;
    }} else {{
        int sig = (int)haetae_caught_signal;
        if (sig == SIGBUS) {{
            sig = {sigsegv};
        }}
        *out_signal = sig;
        ret_status = 1;
    }}

    haetae_in_safe_call = 0;
    sigaction(SIGFPE, &old_fpe, NULL);
    sigaction(SIGSEGV, &old_segv, NULL);
    sigaction(SIGBUS, &old_bus, NULL);
    sigaction(SIGILL, &old_ill, NULL);

    return ret_status;
}}
#endif
#endif
"#,
            max_stack_bytes = MAX_STACK_FRAME_BYTES,
            stack_margin = MODEL_DOWNLOAD_CHUNK_SIZE,
            alt_stack_size = ALT_SIGNAL_STACK_SIZE_BYTES,
            hex_radix = HEX_RADIX,
            stack_overflow = WIN_EXCEPTION_STACK_OVERFLOW,
            int_div_zero = WIN_EXCEPTION_INT_DIVIDE_BY_ZERO,
            flt_div_zero = WIN_EXCEPTION_FLT_DIVIDE_BY_ZERO,
            sigfpe = POSIX_SIGFPE,
            sigsegv = POSIX_SIGSEGV,
        ));

        std::fs::write(&c_path, instrumented_c.as_bytes()).map_err(|e| {
            VerifyError::CompileError(format!("Failed to write temp C file: {}", e))
        })?;

        #[cfg(target_os = "windows")]
        let mut temp_paths = vec![c_path.clone(), lib_path.clone(), exp_path, win_lib_path];
        #[cfg(not(target_os = "windows"))]
        let mut temp_paths = vec![c_path.clone(), lib_path.clone()];

        // Compile into native object file to extract genuine bytecode
        let obj_output = std::process::Command::new(compiler_path)
            .arg(CLANG_FLAG_COMPILE_ONLY)
            .arg(CLANG_OPT_O0)
            .arg("-o")
            .arg(&obj_path)
            .arg(&c_path)
            .output()
            .map_err(|e| {
                for p in &temp_paths {
                    let _ = std::fs::remove_file(p);
                }
                VerifyError::CompileError(format!("Failed to invoke C compiler: {}", e))
            })?;

        if !obj_output.status.success() {
            let stderr_msg = String::from_utf8_lossy(&obj_output.stderr).to_string();
            for p in &temp_paths {
                let _ = std::fs::remove_file(p);
            }
            return Err(VerifyError::CompileError(stderr_msg));
        }
        temp_paths.push(obj_path.clone());

        let bytes = std::fs::read(&obj_path).map_err(|e| {
            for p in &temp_paths {
                let _ = std::fs::remove_file(p);
            }
            VerifyError::CompileError(format!("Failed to read object file: {}", e))
        })?;

        let bytecode = Self::extract_text_section(&bytes).ok_or_else(|| {
            for p in &temp_paths {
                let _ = std::fs::remove_file(p);
            }
            VerifyError::CompileError(
                "Failed to extract .text section from object file".to_string(),
            )
        })?;

        if bytecode.is_empty() {
            for p in &temp_paths {
                let _ = std::fs::remove_file(p);
            }
            return Err(VerifyError::CompileError(
                "Extracted .text section is empty".to_string(),
            ));
        }

        // Compile into native shared library
        let mut lib_cmd = std::process::Command::new(compiler_path);

        #[cfg(target_os = "windows")]
        {
            lib_cmd
                .arg(CLANG_FLAG_SHARED)
                .arg(CLANG_OPT_O0)
                .arg("-Wl,/export:haetae_safe_call")
                .arg("-Wl,/export:haetae_safe_call_multi")
                .arg("-Wl,/export:haetae_fuzz_entry")
                .arg("-Wl,/export:haetae_fuzz_multi_entry");

            if entry_name != "haetae_fuzz_entry" && !entry_name.is_empty() {
                lib_cmd.arg(format!("-Wl,/export:{}", entry_name));
            }
        }

        #[cfg(unix)]
        {
            lib_cmd
                .arg(CLANG_FLAG_SHARED)
                .arg(CLANG_FLAG_FPIC)
                .arg(CLANG_OPT_O0)
                .arg("-pthread");
        }

        lib_cmd.arg("-o").arg(&lib_path).arg(&c_path);

        let output = lib_cmd.output().map_err(|e| {
            VerifyError::CompileError(format!("Failed to invoke C compiler: {}", e))
        })?;

        if !output.status.success() {
            let stderr_msg = String::from_utf8_lossy(&output.stderr).to_string();
            for p in &temp_paths {
                let _ = std::fs::remove_file(p);
            }
            return Err(VerifyError::CompileError(stderr_msg));
        }

        // Load native shared library dynamically
        #[cfg(target_os = "windows")]
        let (handle, safe_call_fn, safe_call_multi_fn) = {
            let wide_path: Vec<u16> = lib_path
                .to_string_lossy()
                .encode_utf16()
                .chain(std::iter::once(0u16))
                .collect();

            let h = unsafe { LoadLibraryW(wide_path.as_ptr()) };
            if h.is_null() {
                for p in &temp_paths {
                    let _ = std::fs::remove_file(p);
                }
                return Err(VerifyError::CompileError(
                    "LoadLibrary failed on generated JIT DLL".to_string(),
                ));
            }

            let sc = unsafe {
                let sym = GetProcAddress(h, c"haetae_safe_call".as_ptr());
                if !sym.is_null() {
                    Some(std::mem::transmute::<
                        *mut c_void,
                        unsafe extern "C" fn(i64, *mut i32, *mut i32) -> i32,
                    >(sym))
                } else {
                    None
                }
            };

            let sc_multi = unsafe {
                let sym = GetProcAddress(h, c"haetae_safe_call_multi".as_ptr());
                if !sym.is_null() {
                    Some(std::mem::transmute::<
                        *mut c_void,
                        unsafe extern "C" fn(*const i64, i32, *mut i32, *mut i32) -> i32,
                    >(sym))
                } else {
                    None
                }
            };

            (h, sc, sc_multi)
        };

        #[cfg(unix)]
        let (handle, safe_call_fn, safe_call_multi_fn) = {
            let c_lib_path = CString::new(lib_path.to_string_lossy().as_bytes()).map_err(|e| {
                VerifyError::CompileError(format!("CString conversion failed for lib path: {}", e))
            })?;

            let h = unsafe { libc::dlopen(c_lib_path.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
            if h.is_null() {
                let err_msg = unsafe {
                    let err_ptr = libc::dlerror();
                    if !err_ptr.is_null() {
                        CStr::from_ptr(err_ptr).to_string_lossy().to_string()
                    } else {
                        "dlopen failed on generated JIT library".to_string()
                    }
                };
                for p in &temp_paths {
                    let _ = std::fs::remove_file(p);
                }
                return Err(VerifyError::CompileError(err_msg));
            }

            let sc = unsafe {
                let sym = libc::dlsym(h, c"haetae_safe_call".as_ptr());
                if !sym.is_null() {
                    Some(std::mem::transmute::<
                        *mut c_void,
                        unsafe extern "C" fn(i64, *mut i32, *mut i32) -> i32,
                    >(sym))
                } else {
                    None
                }
            };

            let sc_multi = unsafe {
                let sym = libc::dlsym(h, c"haetae_safe_call_multi".as_ptr());
                if !sym.is_null() {
                    Some(std::mem::transmute::<
                        *mut c_void,
                        unsafe extern "C" fn(*const i64, i32, *mut i32, *mut i32) -> i32,
                    >(sym))
                } else {
                    None
                }
            };

            (h, sc, sc_multi)
        };

        #[cfg(not(any(target_os = "windows", unix)))]
        let (handle, safe_call_fn, safe_call_multi_fn) = (std::ptr::null_mut(), None, None);

        let relocated_size = bytecode.len();

        Ok(CompiledCode {
            state: std::ptr::null_mut(),
            entry_symbol: entry_name.to_string(),
            relocated_size,
            source_code: c_code.to_string(),
            bytecode,
            module_handle: handle,
            safe_call_fn,
            safe_call_multi_fn,
            temp_paths,
        })
    }

    pub fn execute_entry(&self, code: &CompiledCode) -> Result<i32, VerifyError> {
        self.execute_entry_with_payload(code, 0i64)
    }

    pub fn execute_entry_with_payload(
        &self,
        code: &CompiledCode,
        payload: i64,
    ) -> Result<i32, VerifyError> {
        if let Some(safe_call) = code.safe_call_fn {
            let mut out_res = 0i32;
            let mut out_signal = 0i32;
            let crashed = unsafe { safe_call(payload, &mut out_res, &mut out_signal) };
            if crashed != 0 {
                return Err(VerifyError::CrashSignal(out_signal));
            }
            return Ok(out_res);
        }

        Err(VerifyError::SymbolNotFound(
            "Native JIT safe call entrypoint not loaded".to_string(),
        ))
    }

    pub fn execute_entry_with_payloads(
        &self,
        code: &CompiledCode,
        payloads: &[i64],
    ) -> Result<i32, VerifyError> {
        if payloads.is_empty() {
            return self.execute_entry_with_payload(code, 0i64);
        }

        if let Some(safe_call_multi) = code.safe_call_multi_fn {
            let mut out_res = 0i32;
            let mut out_signal = 0i32;
            let count = payloads.len() as i32;
            let ptr = payloads.as_ptr();
            let crashed = unsafe { safe_call_multi(ptr, count, &mut out_res, &mut out_signal) };
            if crashed != 0 {
                return Err(VerifyError::CrashSignal(out_signal));
            }
            return Ok(out_res);
        }

        Err(VerifyError::SymbolNotFound(
            "Native JIT safe call multi entrypoint not loaded".to_string(),
        ))
    }

    pub fn get_bytecode<'a>(&self, code: &'a CompiledCode) -> &'a [u8] {
        &code.bytecode
    }
}
