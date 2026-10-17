use crate::tcc_engine::{CompiledCode, TccEngine, VerifyError};
pub use haetae_core::config::{
    BYTE_MAX_VALUE, BYTE_VALUES_COUNT, DEFAULT_FUZZ_ITERATIONS, FUZZ_ARG_RANDOM_CHANCE,
    FUZZ_BOUNDARY_POWER_TWO_COUNT, FUZZ_DEFAULT_MUTATION_RATE, FUZZ_MASKS_COUNT,
    FUZZ_MAX_ARG_COUNT, FUZZ_MODE_ADD_ONE, FUZZ_MODE_MASK, FUZZ_MODE_RANDOM,
    FUZZ_MODE_SMALL_OFFSET, FUZZ_MUTATION_MASK_1, FUZZ_MUTATION_MASK_2, FUZZ_MUTATION_MASK_3,
    FUZZ_MUTATION_MASK_4, FUZZ_MUTATION_MASK_5, FUZZ_MUTATION_MODES_COUNT,
    FUZZ_OFFSET_RANGE_BIAS, FUZZ_OFFSET_RANGE_SPAN, MILLIS_PER_SEC, POSIX_SIGFPE, POSIX_SIGSEGV,
    U16_MAX_VALUE, U16_VALUES_COUNT, XORSHIFT_DEFAULT_SEED, XORSHIFT_SHIFT_A, XORSHIFT_SHIFT_B,
    XORSHIFT_SHIFT_C,
};
use std::sync::atomic::{AtomicBool, AtomicI32};

pub static CRASH_DETECTED: AtomicBool = AtomicBool::new(false);
pub static LAST_SIGNAL: AtomicI32 = AtomicI32::new(0);

pub const FUZZ_MASKS: [u64; FUZZ_MASKS_COUNT] = [
    FUZZ_MUTATION_MASK_1,
    FUZZ_MUTATION_MASK_2,
    FUZZ_MUTATION_MASK_3,
    FUZZ_MUTATION_MASK_4,
    FUZZ_MUTATION_MASK_5,
];

#[cfg(all(target_os = "linux", not(miri)))]
const MMAP_ANONYMOUS_FD: libc::c_int = -1;

#[derive(Debug, Clone)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        let s = if seed == 0 {
            XORSHIFT_DEFAULT_SEED
        } else {
            seed
        };
        Self { state: s }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << XORSHIFT_SHIFT_A;
        x ^= x >> XORSHIFT_SHIFT_B;
        x ^= x << XORSHIFT_SHIFT_C;
        self.state = x;
        x
    }

    pub fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    pub fn next_range(&mut self, max: usize) -> usize {
        if max <= 1 {
            return 0;
        }
        (self.next_u64() as usize) % max
    }
}

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

    pub const PAYLOAD_BYTE_MAX: i64 = BYTE_MAX_VALUE;
    pub const PAYLOAD_BYTE_OVERFLOW: i64 = BYTE_VALUES_COUNT;
    pub const PAYLOAD_U16_MAX: i64 = U16_MAX_VALUE;
    pub const PAYLOAD_U16_OVERFLOW: i64 = U16_VALUES_COUNT;
    pub const PAYLOAD_NEG_ONE: i64 = -1;

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
            i16::MAX as i64,
            i16::MIN as i64,
            u16::MAX as i64,
            i8::MAX as i64,
            i8::MIN as i64,
            u8::MAX as i64,
            Self::PAYLOAD_BYTE_MAX,
            Self::PAYLOAD_BYTE_OVERFLOW,
            Self::PAYLOAD_U16_MAX,
            Self::PAYLOAD_U16_OVERFLOW,
        ]
    }

    pub fn build_comprehensive_boundaries() -> Vec<i64> {
        let mut boundaries = Self::edge_payloads().to_vec();

        // Add power of two boundaries (1 << k, (1 << k) - 1, (1 << k) + 1, -(1 << k))
        let max_shift = (u64::BITS as usize).saturating_sub(1);
        let shift_limit = FUZZ_BOUNDARY_POWER_TWO_COUNT.min(max_shift);
        for shift in 0..shift_limit {
            let p2 = 1i64 << shift;
            boundaries.push(p2);
            boundaries.push(p2.wrapping_sub(1));
            boundaries.push(p2.wrapping_add(1));
            boundaries.push(-p2);
        }

        boundaries
    }

    pub fn sample_payload(
        &self,
        boundaries: &[i64],
        prng: &mut XorShift64,
        iteration: usize,
    ) -> i64 {
        if iteration < boundaries.len() {
            boundaries[iteration]
        } else {
            let mode = prng.next_range(FUZZ_MUTATION_MODES_COUNT);
            if mode == FUZZ_MODE_RANDOM {
                prng.next_i64()
            } else if mode == FUZZ_MODE_MASK {
                let idx = prng.next_range(boundaries.len());
                let base = boundaries[idx];
                let mask = FUZZ_MASKS[prng.next_range(FUZZ_MASKS.len())];
                base ^ (mask as i64)
            } else if mode == FUZZ_MODE_ADD_ONE {
                let idx = prng.next_range(boundaries.len());
                boundaries[idx].wrapping_add(1)
            } else {
                (prng.next_range(FUZZ_OFFSET_RANGE_SPAN) as i64) - (FUZZ_OFFSET_RANGE_BIAS as i64)
            }
        }
    }

    pub fn sample_multi_args(
        &self,
        boundaries: &[i64],
        prng: &mut XorShift64,
        iteration: usize,
        payload: i64,
    ) -> [i64; FUZZ_MAX_ARG_COUNT] {
        let mut multi_args = [0i64; FUZZ_MAX_ARG_COUNT];
        if let Some((first, rest)) = multi_args.split_first_mut() {
            *first = payload;
            if iteration < boundaries.len() {
                for (arg, &boundary) in rest
                    .iter_mut()
                    .zip(boundaries.iter().cycle().skip(iteration + 1))
                {
                    *arg = boundary;
                }
            } else {
                for arg in rest.iter_mut() {
                    if prng.next_range(FUZZ_ARG_RANDOM_CHANCE) == 0 {
                        let b_idx = prng.next_range(boundaries.len());
                        *arg = boundaries[b_idx];
                    } else {
                        *arg = prng.next_i64();
                    }
                }
            }
        }
        multi_args
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
                MMAP_ANONYMOUS_FD,
                0 as libc::off_t,
            ) as *mut SharedFuzzState
        };
        if shm_ptr == libc::MAP_FAILED as *mut SharedFuzzState {
            return Err(VerifyError::InitFailed);
        }

        unsafe {
            (*shm_ptr).passed = 0;
            (*shm_ptr).failed = 0;
            (*shm_ptr).crash_signal = 0;
            (*shm_ptr).failed_payload = 0;
            (*shm_ptr).current_payload = 0;
            (*shm_ptr).completed = 0;
        }

        let pid = unsafe { libc::fork() };
        if pid < 0 {
            unsafe {
                libc::munmap(shm_ptr as *mut libc::c_void, shm_size);
            }
            return Err(VerifyError::InitFailed);
        }

        if pid == 0 {
            let boundaries = Self::build_comprehensive_boundaries();
            let mut prng = XorShift64::new(XORSHIFT_DEFAULT_SEED);

            for i in 0..self.max_iterations {
                let payload = self.sample_payload(&boundaries, &mut prng, i);
                let multi_args = self.sample_multi_args(&boundaries, &mut prng, i, payload);

                unsafe {
                    (*shm_ptr).current_payload = payload;
                }

                let exec_res = tcc.execute_entry_with_payloads(code, &multi_args);
                match exec_res {
                    Ok(_) => unsafe {
                        (*shm_ptr).passed += 1;
                    },
                    Err(e) => {
                        unsafe {
                            (*shm_ptr).failed += 1;
                            (*shm_ptr).failed_payload = payload;
                            match e {
                                VerifyError::CrashSignal(sig) => {
                                    (*shm_ptr).crash_signal = sig;
                                }
                                _ => {
                                    (*shm_ptr).crash_signal = POSIX_SIGFPE;
                                }
                            }
                        }
                        unsafe {
                            libc::_exit(1);
                        }
                    }
                }
            }
            unsafe {
                (*shm_ptr).completed = 1;
                libc::_exit(0);
            }
        }

        let mut status: libc::c_int = 0;
        unsafe {
            libc::waitpid(pid, &mut status, 0);
        }

        let passed = unsafe { (*shm_ptr).passed };
        let mut failed = unsafe { (*shm_ptr).failed };
        let mut crash_signal = None;
        let mut failed_payload = None;

        if libc::WIFSIGNALED(status) {
            let sig = libc::WTERMSIG(status);
            failed += 1;
            crash_signal = Some(sig);
            failed_payload = Some(unsafe { (*shm_ptr).current_payload });
        } else if libc::WIFEXITED(status) {
            let exit_code = libc::WEXITSTATUS(status);
            if exit_code != 0 {
                let sig = unsafe { (*shm_ptr).crash_signal };
                if sig != 0 {
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
        let mut passed = 0;
        let mut failed = 0;
        let mut crash_signal = None;
        let mut failed_payload = None;

        let boundaries = Self::build_comprehensive_boundaries();
        let mut prng = XorShift64::new(XORSHIFT_DEFAULT_SEED);

        for i in 0..self.max_iterations {
            let payload = self.sample_payload(&boundaries, &mut prng, i);
            let multi_args = self.sample_multi_args(&boundaries, &mut prng, i, payload);

            let exec_res = tcc.execute_entry_with_payloads(code, &multi_args);
            match exec_res {
                Ok(_) => {
                    passed += 1;
                }
                Err(VerifyError::CrashSignal(sig)) => {
                    failed += 1;
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
