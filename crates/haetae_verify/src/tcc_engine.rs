#[allow(unused_imports)]
use haetae_core::config::{NUM_ONE, NUM_TWO, NUM_ZERO, X86_NOP_OPCODE, X86_RET_OPCODE};
#[cfg(all(target_os = "linux", not(miri)))]
use std::ffi::{c_char, CString};
#[allow(unused_imports)]
use std::ffi::{c_int, c_void};

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
}

#[cfg(all(target_os = "linux", not(miri)))]
const TCC_OUTPUT_MEMORY: c_int = NUM_ONE as c_int;
#[cfg(all(target_os = "linux", not(miri)))]
const TCC_RELOCATE_QUERY: *mut c_void = std::ptr::null_mut();

#[cfg(all(target_os = "linux", not(miri)))]
extern "C" {
    fn tcc_new() -> *mut c_void;
    fn tcc_delete(s: *mut c_void);
    fn tcc_set_output_type(s: *mut c_void, output_type: c_int) -> c_int;
    fn tcc_compile_string(s: *mut c_void, buf: *const c_char) -> c_int;
    fn tcc_relocate(s: *mut c_void, ptr: *mut c_void) -> c_int;
    fn tcc_get_symbol(s: *mut c_void, name: *const c_char) -> *mut c_void;
}

pub struct CompiledCode {
    #[cfg(all(target_os = "linux", not(miri)))]
    pub state: *mut c_void,
    pub entry_symbol: String,
    pub relocated_size: usize,
    pub source_code: String,
}

impl Drop for CompiledCode {
    fn drop(&mut self) {
        #[cfg(all(target_os = "linux", not(miri)))]
        unsafe {
            if !self.state.is_null() {
                tcc_delete(self.state);
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

    #[cfg(all(target_os = "linux", not(miri)))]
    pub fn compile_in_memory(
        &self,
        c_code: &str,
        entry_name: &str,
    ) -> Result<CompiledCode, VerifyError> {
        let state = unsafe { tcc_new() };
        if state.is_null() {
            return Err(VerifyError::InitFailed);
        }

        unsafe {
            let ret = tcc_set_output_type(state, TCC_OUTPUT_MEMORY);
            if ret < NUM_ZERO as c_int {
                tcc_delete(state);
                return Err(VerifyError::CompileError(
                    "failed to set output type".to_string(),
                ));
            }

            let c_str = CString::new(c_code).map_err(|e| {
                VerifyError::CompileError(format!("CString conversion failed: {}", e))
            })?;

            if tcc_compile_string(state, c_str.as_ptr()) < NUM_ZERO as c_int {
                tcc_delete(state);
                return Err(VerifyError::CompileError(
                    "in-memory C compilation failed".to_string(),
                ));
            }

            let size = tcc_relocate(state, TCC_RELOCATE_QUERY);
            if size < NUM_ZERO as c_int {
                tcc_delete(state);
                return Err(VerifyError::RelocateFailed);
            }

            let auto_reloc = NUM_ONE as *mut c_void;
            if tcc_relocate(state, auto_reloc) < NUM_ZERO as c_int {
                tcc_delete(state);
                return Err(VerifyError::RelocateFailed);
            }

            let sym_name = CString::new(entry_name).map_err(|e| {
                VerifyError::CompileError(format!("entry name CString failed: {}", e))
            })?;
            let sym = tcc_get_symbol(state, sym_name.as_ptr());
            if sym.is_null() {
                tcc_delete(state);
                return Err(VerifyError::SymbolNotFound(entry_name.to_string()));
            }

            Ok(CompiledCode {
                state,
                entry_symbol: entry_name.to_string(),
                relocated_size: size as usize,
                source_code: c_code.to_string(),
            })
        }
    }

    #[cfg(any(not(target_os = "linux"), miri))]
    pub fn compile_in_memory(
        &self,
        c_code: &str,
        entry_name: &str,
    ) -> Result<CompiledCode, VerifyError> {
        if c_code.is_empty() {
            return Err(VerifyError::CompileError("Empty C code".to_string()));
        }
        if !c_code.contains(entry_name) {
            return Err(VerifyError::SymbolNotFound(entry_name.to_string()));
        }
        Ok(CompiledCode {
            entry_symbol: entry_name.to_string(),
            relocated_size: c_code.len(),
            source_code: c_code.to_string(),
        })
    }

    pub fn execute_entry(&self, code: &CompiledCode) -> Result<i32, VerifyError> {
        self.execute_entry_with_payload(code, NUM_ZERO as i64)
    }

    #[cfg(all(target_os = "linux", not(miri)))]
    pub fn execute_entry_with_payload(
        &self,
        code: &CompiledCode,
        payload: i64,
    ) -> Result<i32, VerifyError> {
        if code.state.is_null() {
            return Ok(NUM_ZERO as i32);
        }
        let sym_name = CString::new(code.entry_symbol.as_str())
            .map_err(|e| VerifyError::CompileError(format!("CString conversion failed: {}", e)))?;
        unsafe {
            let sym = tcc_get_symbol(code.state, sym_name.as_ptr());
            if sym.is_null() {
                return Err(VerifyError::SymbolNotFound(code.entry_symbol.clone()));
            }
            let func: extern "C" fn(i64) -> c_int = std::mem::transmute(sym);
            Ok(func(payload) as i32)
        }
    }

    #[cfg(any(not(target_os = "linux"), miri))]
    pub fn execute_entry_with_payload(
        &self,
        code: &CompiledCode,
        payload: i64,
    ) -> Result<i32, VerifyError> {
        crate::evaluator::evaluate_c_source(&code.source_code, payload)
    }

    #[cfg(all(target_os = "linux", not(miri)))]
    pub fn get_bytecode<'a>(&self, code: &'a CompiledCode) -> &'a [u8] {
        if code.state.is_null() {
            return &[X86_NOP_OPCODE, X86_RET_OPCODE];
        }
        let Ok(sym_name) = CString::new(code.entry_symbol.as_str()) else {
            return &[X86_NOP_OPCODE, X86_RET_OPCODE];
        };
        unsafe {
            let sym = tcc_get_symbol(code.state, sym_name.as_ptr());
            if sym.is_null() {
                return &[X86_NOP_OPCODE, X86_RET_OPCODE];
            }
            let len = code.relocated_size.max(NUM_TWO);
            std::slice::from_raw_parts(sym as *const u8, len)
        }
    }

    #[cfg(any(not(target_os = "linux"), miri))]
    pub fn get_bytecode<'a>(&self, _code: &'a CompiledCode) -> &'a [u8] {
        &[X86_NOP_OPCODE, X86_RET_OPCODE]
    }
}
