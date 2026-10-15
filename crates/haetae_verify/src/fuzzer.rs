use crate::tcc_engine::{CompiledCode, TccEngine, VerifyError};
use haetae_core::config::{
    DEFAULT_FUZZ_ITERATIONS, MILLIS_PER_SEC, NUM_NEG_ONE, NUM_ONE,
    NUM_SIXTY_FIVE_THOUSAND_FIVE_HUNDRED_THIRTY_FIVE,
    NUM_SIXTY_FIVE_THOUSAND_FIVE_HUNDRED_THIRTY_SIX, NUM_TWO_HUNDRED_FIFTY_FIVE,
    NUM_TWO_HUNDRED_FIFTY_SIX, NUM_ZERO,
};
use std::sync::atomic::AtomicBool;

pub static CRASH_DETECTED: AtomicBool = AtomicBool::new(false);
pub static LAST_SIGNAL: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

#[cfg(all(target_os = "linux", not(miri)))]
#[repr(C)]
struct SharedFuzzState {
    passed: usize,
    failed: usize,
    crash_signal: i32,
    failed_payload: i64,
    current_payload: i64,
    completed: u8,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FuzzResult {
    pub total_iterations: usize,
    pub passed: usize,
    pub failed: usize,
    pub duration_ms: f64,
    pub crash_signal: Option<i32>,
    pub failed_payload: Option<i64>,
}

pub struct FuzzerEngine {
    pub max_iterations: usize,
}

impl Default for FuzzerEngine {
    fn default() -> Self {
        Self {
            max_iterations: DEFAULT_FUZZ_ITERATIONS,
        }
    }
}

impl FuzzerEngine {
    pub fn new(max_iterations: usize) -> Self {
        Self { max_iterations }
    }

    pub const PAYLOAD_BYTE_MAX: i64 = NUM_TWO_HUNDRED_FIFTY_FIVE;
    pub const PAYLOAD_BYTE_OVERFLOW: i64 = NUM_TWO_HUNDRED_FIFTY_SIX;
    pub const PAYLOAD_U16_MAX: i64 = NUM_SIXTY_FIVE_THOUSAND_FIVE_HUNDRED_THIRTY_FIVE;
    pub const PAYLOAD_U16_OVERFLOW: i64 = NUM_SIXTY_FIVE_THOUSAND_FIVE_HUNDRED_THIRTY_SIX;
    pub const PAYLOAD_NEG_ONE: i64 = NUM_NEG_ONE;

    pub fn edge_payloads() -> &'static [i64] {
        &[
            0,
            1,
            Self::PAYLOAD_NEG_ONE,
            i64::MAX,
            i64::MIN,
            i32::MAX as i64,
            i32::MIN as i64,
            u32::MAX as i64,
            Self::PAYLOAD_BYTE_MAX,
            Self::PAYLOAD_BYTE_OVERFLOW,
            Self::PAYLOAD_U16_MAX,
            Self::PAYLOAD_U16_OVERFLOW,
        ]
    }

    #[cfg(all(target_os = "linux", not(miri)))]
    pub fn run_fuzz_loop(
        &self,
        tcc: &TccEngine,
        code: &CompiledCode,
    ) -> Result<FuzzResult, VerifyError> {
        let start = std::time::Instant::now();
        let shm_size = std::mem::size_of::<SharedFuzzState>();
        let shm_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                shm_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_ANONYMOUS,
                NUM_NEG_ONE as libc::c_int,
                NUM_ZERO as libc::off_t,
            ) as *mut SharedFuzzState
        };
        if shm_ptr == libc::MAP_FAILED as *mut SharedFuzzState {
            return Err(VerifyError::InitFailed);
        }

        unsafe {
            (*shm_ptr).passed = NUM_ZERO;
            (*shm_ptr).failed = NUM_ZERO;
            (*shm_ptr).crash_signal = NUM_ZERO as i32;
            (*shm_ptr).failed_payload = NUM_ZERO as i64;
            (*shm_ptr).current_payload = NUM_ZERO as i64;
            (*shm_ptr).completed = NUM_ZERO as u8;
        }

        let pid = unsafe { libc::fork() };
        if pid < NUM_ZERO as libc::pid_t {
            unsafe {
                libc::munmap(shm_ptr as *mut libc::c_void, shm_size);
            }
            return Err(VerifyError::InitFailed);
        }

        if pid == NUM_ZERO as libc::pid_t {
            let payloads = Self::edge_payloads();
            let payload_count = payloads.len();
            for i in NUM_ZERO..self.max_iterations {
                let payload = payloads[i % payload_count];
                unsafe {
                    (*shm_ptr).current_payload = payload;
                }
                let exec_res = tcc.execute_entry_with_payload(code, payload);
                match exec_res {
                    Ok(_) => unsafe {
                        (*shm_ptr).passed += NUM_ONE;
                    },
                    Err(e) => {
                        unsafe {
                            (*shm_ptr).failed += NUM_ONE;
                            (*shm_ptr).failed_payload = payload;
                            match e {
                                VerifyError::CrashSignal(sig) => {
                                    (*shm_ptr).crash_signal = sig;
                                }
                                _ => {
                                    (*shm_ptr).crash_signal = haetae_core::config::POSIX_SIGFPE;
                                }
                            }
                        }
                        unsafe {
                            libc::_exit(NUM_ONE as i32);
                        }
                    }
                }
            }
            unsafe {
                (*shm_ptr).completed = NUM_ONE as u8;
                libc::_exit(NUM_ZERO as i32);
            }
        }

        let mut status: libc::c_int = NUM_ZERO as libc::c_int;
        unsafe {
            libc::waitpid(pid, &mut status, NUM_ZERO as libc::c_int);
        }

        let passed = unsafe { (*shm_ptr).passed };
        let mut failed = unsafe { (*shm_ptr).failed };
        let mut crash_signal = None;
        let mut failed_payload = None;

        if libc::WIFSIGNALED(status) {
            let sig = libc::WTERMSIG(status);
            failed += NUM_ONE;
            crash_signal = Some(sig);
            failed_payload = Some(unsafe { (*shm_ptr).current_payload });
        } else if libc::WIFEXITED(status) {
            let exit_code = libc::WEXITSTATUS(status);
            if exit_code != NUM_ZERO as i32 {
                let sig = unsafe { (*shm_ptr).crash_signal };
                if sig != NUM_ZERO as i32 {
                    crash_signal = Some(sig);
                }
                failed_payload = Some(unsafe { (*shm_ptr).failed_payload });
            }
        }

        unsafe {
            libc::munmap(shm_ptr as *mut libc::c_void, shm_size);
        }

        let duration_ms = start.elapsed().as_secs_f64() * MILLIS_PER_SEC;

        Ok(FuzzResult {
            total_iterations: self.max_iterations,
            passed,
            failed,
            duration_ms,
            crash_signal,
            failed_payload,
        })
    }

    #[cfg(any(not(target_os = "linux"), miri))]
    pub fn run_fuzz_loop(
        &self,
        tcc: &TccEngine,
        code: &CompiledCode,
    ) -> Result<FuzzResult, VerifyError> {
        let start = std::time::Instant::now();
        let mut passed = NUM_ZERO;
        let mut failed = NUM_ZERO;
        let mut crash_signal = None;
        let mut failed_payload = None;

        let payloads = Self::edge_payloads();
        let payload_count = payloads.len();

        for i in NUM_ZERO..self.max_iterations {
            let payload = payloads[i % payload_count];
            let exec_res = tcc.execute_entry_with_payload(code, payload);
            match exec_res {
                Ok(_) => {
                    passed += NUM_ONE;
                }
                Err(VerifyError::CrashSignal(sig)) => {
                    failed += NUM_ONE;
                    crash_signal = Some(sig);
                    failed_payload = Some(payload);
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let duration_ms = start.elapsed().as_secs_f64() * MILLIS_PER_SEC;

        Ok(FuzzResult {
            total_iterations: self.max_iterations,
            passed,
            failed,
            duration_ms,
            crash_signal,
            failed_payload,
        })
    }
}

pub fn install_signal_handlers() {}
