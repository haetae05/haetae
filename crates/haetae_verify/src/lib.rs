pub mod c_lower;
pub mod diagnostics;
pub mod evaluator;
pub mod fuzzer;
pub mod headers;
pub mod tcc_engine;
pub mod zydis_cfi;

pub use c_lower::{
    generate_c_fuzz_harness, generate_fuzz_payload_assignment, generate_multi_c_fuzz_harness,
    lower_source_to_c_ir,
};
pub use diagnostics::DiagnosticReport;
pub use fuzzer::{FuzzResult, FuzzerEngine, XorShift64};
pub use headers::bundle_headers;
pub use tcc_engine::{CompiledCode, TccEngine, VerifyError};
pub use zydis_cfi::{
    detect_isa, get_verifier, verify_cfi_for_isa, verify_cfi_integrity,
    verify_stack_frame_4kb_bound, verify_stack_frame_4kb_bound_for_isa, verify_stack_frame_bound,
    verify_stack_frame_bound_for_isa, AArch64Verifier, IsaVerifier, RiscV64Verifier, TargetIsa,
    X86_64Verifier, STACK_FRAME_4KB_BOUND,
};
