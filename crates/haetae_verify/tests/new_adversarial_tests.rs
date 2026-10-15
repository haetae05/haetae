#![allow(unused_imports)]

use haetae_core::config::{
    TargetIsa, AARCH64_INSN_BYTES, AARCH64_RET_INSN, BENCHMARK_SAMPLE_DURATION_MS,
    MAX_CALL_STACK_DEPTH, MAX_STACK_FRAME_BYTES, NUM_EIGHT, NUM_FIFTY, NUM_FIVE,
    NUM_FIVE_HUNDRED_TWELVE, NUM_FORTY_EIGHT, NUM_FORTY_TWO, NUM_FOUR,
    NUM_FOUR_THOUSAND_NINETY_SIX, NUM_NEG_ONE, NUM_NINE, NUM_ONE, NUM_ONE_HUNDRED,
    NUM_ONE_THOUSAND, NUM_ONE_TWENTY_EIGHT, NUM_SEVEN, NUM_SIX, NUM_SIXTEEN, NUM_SIXTY_FOUR,
    NUM_TEN, NUM_THIRTY, NUM_THIRTY_TWO, NUM_THREE, NUM_TWO, NUM_TWO_HUNDRED,
    NUM_TWO_HUNDRED_FIFTY_SIX, NUM_TWO_THOUSAND, NUM_ZERO, POSIX_SIGFPE, POSIX_SIGSEGV,
    RISCV_RET_INSN, X86_NOP_OPCODE, X86_RET_OPCODE, X86_REX_W, X86_RSP_MODRM, X86_SUB_IMM32_OP,
};
use haetae_verify::c_lower::lower_source_to_c_ir;
use haetae_verify::diagnostics::DiagnosticReport;
use haetae_verify::fuzzer::FuzzerEngine;
use haetae_verify::headers::bundle_headers;
use haetae_verify::tcc_engine::{TccEngine, VerifyError};
use haetae_verify::zydis_cfi::{
    detect_isa, get_verifier, verify_cfi_for_isa, verify_cfi_integrity,
    verify_stack_frame_bound_for_isa, AArch64Verifier, IsaVerifier, RiscV64Verifier,
    X86_64Verifier,
};
use std::sync::Arc;

// ============================================================================
// Area 1: Adversarial C Code Execution & Safety
// ============================================================================

#[test]
fn test_adv_01_indirect_mutual_recursion_exhausts_call_stack() {
    let tcc = TccEngine::new();
    let code = "int mutual_b(int n);\nint mutual_a(int n) {\n  if (n <= 0) return 0;\n  return mutual_b(n - 1) + 1;\n}\nint mutual_b(int n) {\n  if (n <= 0) return 0;\n  return mutual_a(n - 1) + 1;\n}\nint haetae_fuzz_entry(long long p) {\n  return mutual_a((int)p);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile mutual recursion");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ONE_HUNDRED as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_02_safe_bounded_mutual_recursion_succeeds() {
    let tcc = TccEngine::new();
    let code = "int mutual_b(int n);\nint mutual_a(int n) {\n  if (n <= 0) return 0;\n  return mutual_b(n - 1) + 1;\n}\nint mutual_b(int n) {\n  if (n <= 0) return 0;\n  return mutual_a(n - 1) + 1;\n}\nint haetae_fuzz_entry(long long p) {\n  return mutual_a((int)p);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile safe mutual recursion");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_FOUR as i64)
        .expect("exec safe mutual recursion");
    assert_eq!(val, NUM_FOUR as i32);
}

#[test]
fn test_adv_03_deep_tree_recursion_branching_exhaustion() {
    let tcc = TccEngine::new();
    let code = "int tree_branch(int depth) {\n  if (depth <= 0) return 1;\n  return tree_branch(depth - 1) + tree_branch(depth - 1);\n}\nint haetae_fuzz_entry(long long p) {\n  return tree_branch((int)p);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile tree branch");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ONE_HUNDRED as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_04_deep_recursion_with_arithmetic_payload() {
    let tcc = TccEngine::new();
    let code = "int step_down(int x) {\n  if (x <= 1) return 1;\n  return step_down(x - 1) + 1;\n}\nint haetae_fuzz_entry(long long p) {\n  return step_down((int)p);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile step down");
    let val_safe = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("exec safe step down");
    assert_eq!(val_safe, NUM_FIVE as i32);

    let res_deep = tcc.execute_entry_with_payload(&compiled, NUM_ONE_HUNDRED as i64);
    match res_deep {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res_deep),
    }
}

#[test]
fn test_adv_05_null_pointer_direct_dereference_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int* ptr = 0;\n  *ptr;\n  return (int)(p & 1);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile null deref statement");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_06_null_pointer_return_dereference_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int* ptr = 0;\n  return *ptr;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile null return deref");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_07_array_out_of_bounds_negative_index_read() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int arr[5] = {10, 20, 30, 40, 50};\n  int idx = -1;\n  return arr[idx];\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile negative index read");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_08_array_out_of_bounds_upper_index_read() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int arr[5] = {10, 20, 30, 40, 50};\n  int idx = 10;\n  return arr[idx];\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile upper index read");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_09_array_out_of_bounds_negative_index_write() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int arr[5] = {10, 20, 30, 40, 50};\n  arr[-1] = 42;\n  return arr[0];\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile negative index write");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_10_array_out_of_bounds_upper_index_write() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int arr[5] = {10, 20, 30, 40, 50};\n  arr[10] = 42;\n  return arr[0];\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile upper index write");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_11_haetae_checked_index_negative_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  long long idx = haetae_checked_index(10, -1);\n  return (int)idx;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile checked index negative");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_12_haetae_checked_index_overflow_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  long long idx = haetae_checked_index(10, 10);\n  return (int)idx;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile checked index overflow");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_13_haetae_checked_index_safe_access() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  long long idx = haetae_checked_index(10, 5);\n  return (int)idx;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile checked index safe");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec checked index safe");
    assert_eq!(val, NUM_FIVE as i32);
}

#[test]
fn test_adv_14_direct_sigfpe_trap_invocation() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  haetae_trap_sigfpe();\n  return 0;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile direct trap");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGFPE);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGFPE), got {:?}", res),
    }
}

#[test]
fn test_adv_15_haetae_checked_div_zero_denominator() {
    let tcc = TccEngine::new();
    let code =
        "int haetae_fuzz_entry(long long p) {\n  return (int)haetae_checked_div(100, 0);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile checked div zero");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGFPE);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGFPE), got {:?}", res),
    }
}

#[test]
fn test_adv_16_haetae_checked_mod_zero_denominator() {
    let tcc = TccEngine::new();
    let code =
        "int haetae_fuzz_entry(long long p) {\n  return (int)haetae_checked_mod(100, 0);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile checked mod zero");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGFPE);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGFPE), got {:?}", res),
    }
}

#[test]
fn test_adv_17_haetae_aarch64_div_guard_zero() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int guard = (int)haetae_aarch64_div_guard(0);\n  return guard + 42;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile div guard zero");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGFPE);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGFPE), got {:?}", res),
    }
}

#[test]
fn test_adv_18_checked_multiplication_overflow_fpe() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  long long max_val = 9223372036854775807LL;\n  long long overflow = max_val * 2LL;\n  return (int)overflow;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile mul overflow");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGFPE);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGFPE), got {:?}", res),
    }
}

#[test]
fn test_adv_19_macro_bomb_chained_expression_evaluation() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int a = (1 ? (0 ? 10 : 20) : 30);\n  int b = (a > 10) ? (a < 30 ? 42 : 50) : 60;\n  return b;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile nested ternary");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec nested ternary");
    assert_eq!(val, NUM_FORTY_TWO as i32);
}

#[test]
fn test_adv_20_complex_nested_typedefs_and_casts() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  long x = (long)(int)(p + 42);\n  return (int)x;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile nested casts");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec nested casts");
    assert_eq!(val, NUM_FORTY_TWO as i32);
}

#[test]
fn test_adv_21_massive_stack_allocation_single_buffer_overflow() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  char huge_buf[2000000];\n  return (int)(p & 1);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile single huge stack");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_22_cumulative_stack_allocation_multi_buffer_overflow() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  char part_a[600000];\n  char part_b[600000];\n  return (int)(p & 1);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile cumulative huge stack");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_23_integer_array_stack_allocation_overflow() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int huge_int[300000];\n  return (int)(p & 1);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile huge int stack");
    let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
    match res {
        Err(VerifyError::CrashSignal(sig)) => {
            assert_eq!(sig, POSIX_SIGSEGV);
        }
        _ => panic!("Expected CrashSignal(POSIX_SIGSEGV), got {:?}", res),
    }
}

#[test]
fn test_adv_24_safe_stack_allocation_under_limit() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  char safe_buf[512];\n  int safe_arr[100];\n  return (int)(p + 1);\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile safe stack under limit");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec safe stack");
    assert_eq!(val, NUM_ONE as i32);
}

// ============================================================================
// Area 2: CFI, ISA & Instruction Verification
// ============================================================================

#[test]
fn test_adv_25_x86_cfi_rejection_missing_ret() {
    let verifier = X86_64Verifier;
    let malformed_bytecode = vec![X86_NOP_OPCODE; NUM_SIXTEEN];
    let res = verifier.verify_cfi(&malformed_bytecode);
    assert!(res.is_err());
    match res {
        Err(VerifyError::CfiViolation(msg)) => {
            assert!(msg.contains("no valid function return"));
        }
        _ => panic!("Expected CfiViolation, got {:?}", res),
    }
}

#[test]
fn test_adv_26_aarch64_cfi_rejection_missing_ret() {
    let verifier = AArch64Verifier;
    let malformed_bytecode = vec![NUM_ZERO as u8; NUM_SIXTEEN];
    let res = verifier.verify_cfi(&malformed_bytecode);
    assert!(res.is_err());
    match res {
        Err(VerifyError::CfiViolation(msg)) => {
            assert!(msg.contains("no valid function return"));
        }
        _ => panic!("Expected CfiViolation, got {:?}", res),
    }
}

#[test]
fn test_adv_27_riscv64_cfi_rejection_missing_ret() {
    let verifier = RiscV64Verifier;
    let malformed_bytecode = vec![NUM_ZERO as u8; NUM_SIXTEEN];
    let res = verifier.verify_cfi(&malformed_bytecode);
    assert!(res.is_err());
    match res {
        Err(VerifyError::CfiViolation(msg)) => {
            assert!(msg.contains("no valid function return"));
        }
        _ => panic!("Expected CfiViolation, got {:?}", res),
    }
}

#[test]
fn test_adv_28_x86_stack_overflow_sub_rsp_imm32() {
    let verifier = X86_64Verifier;
    let mut bytecode = vec![X86_REX_W, X86_SUB_IMM32_OP, X86_RSP_MODRM];
    let huge_imm: u32 = (MAX_STACK_FRAME_BYTES + NUM_ONE_THOUSAND) as u32;
    bytecode.extend_from_slice(&huge_imm.to_le_bytes());
    bytecode.push(X86_RET_OPCODE);

    let res = verifier.verify_stack_frame(&bytecode, MAX_STACK_FRAME_BYTES);
    assert!(res.is_err());
    match res {
        Err(VerifyError::StackOverflow(alloc, max)) => {
            assert_eq!(alloc, (MAX_STACK_FRAME_BYTES + NUM_ONE_THOUSAND));
            assert_eq!(max, MAX_STACK_FRAME_BYTES);
        }
        _ => panic!("Expected StackOverflow, got {:?}", res),
    }
}

#[test]
fn test_adv_29_aarch64_stack_overflow_sub_sp_imm() {
    let verifier = AArch64Verifier;
    let huge_imm = (NUM_TWO_HUNDRED) as u32;
    let sub_sp_match = haetae_core::config::AARCH64_SUB_SP_MATCH;
    let insn = sub_sp_match | (huge_imm << haetae_core::config::AARCH64_IMM12_SHIFT);
    let mut bytecode = insn.to_le_bytes().to_vec();
    bytecode.extend_from_slice(&AARCH64_RET_INSN.to_le_bytes());

    let res = verifier.verify_stack_frame(&bytecode, NUM_ONE_HUNDRED);
    assert!(res.is_err());
    match res {
        Err(VerifyError::StackOverflow(alloc, max)) => {
            assert_eq!(alloc, NUM_TWO_HUNDRED);
            assert_eq!(max, NUM_ONE_HUNDRED);
        }
        _ => panic!("Expected StackOverflow, got {:?}", res),
    }
}

#[test]
fn test_adv_30_riscv_stack_overflow_addi_sp_imm() {
    let verifier = RiscV64Verifier;
    let addi_sp = haetae_core::config::RISCV_ADDI_SP_MINUS_64;
    let mut bytecode = addi_sp.to_le_bytes().to_vec();
    bytecode.extend_from_slice(&RISCV_RET_INSN.to_le_bytes());

    let res = verifier.verify_stack_frame(&bytecode, NUM_THIRTY_TWO);
    assert!(res.is_err());
    match res {
        Err(VerifyError::StackOverflow(alloc, max)) => {
            assert_eq!(alloc, NUM_SIXTY_FOUR);
            assert_eq!(max, NUM_THIRTY_TWO);
        }
        _ => panic!("Expected StackOverflow, got {:?}", res),
    }
}

#[test]
fn test_adv_31_detect_isa_with_adversarial_padded_bytecode() {
    let aarch64_code = AARCH64_RET_INSN.to_le_bytes().to_vec();
    assert_eq!(detect_isa(&aarch64_code), Some(TargetIsa::AArch64));

    let riscv_code = RISCV_RET_INSN.to_le_bytes().to_vec();
    assert_eq!(detect_isa(&riscv_code), Some(TargetIsa::RiscV64));

    let x86_code = vec![X86_NOP_OPCODE, X86_RET_OPCODE];
    assert_eq!(detect_isa(&x86_code), Some(TargetIsa::X86_64));
}

#[test]
fn test_adv_32_cfi_verifier_dispatch_trait_consistency() {
    let v_x86 = get_verifier(TargetIsa::X86_64);
    assert_eq!(v_x86.isa(), TargetIsa::X86_64);

    let v_arm = get_verifier(TargetIsa::AArch64);
    assert_eq!(v_arm.isa(), TargetIsa::AArch64);

    let v_rv = get_verifier(TargetIsa::RiscV64);
    assert_eq!(v_rv.isa(), TargetIsa::RiscV64);
}

// ============================================================================
// Area 3: Concurrent & Thread-safe Engine Verification
// ============================================================================

#[test]
fn test_adv_33_multithreaded_parallel_compilation() {
    let mut handles = Vec::new();
    for thread_idx in NUM_ZERO..NUM_EIGHT {
        let handle = std::thread::spawn(move || {
            let tcc = TccEngine::new();
            let code = format!(
                "int haetae_fuzz_entry(long long p) {{\n  return (int)(p + {});\n}}\n",
                thread_idx
            );
            let compiled = tcc
                .compile_in_memory(&code, "haetae_fuzz_entry")
                .expect("parallel compile");
            let val = tcc
                .execute_entry_with_payload(&compiled, NUM_TEN as i64)
                .expect("parallel exec");
            assert_eq!(val, (NUM_TEN + thread_idx) as i32);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread join");
    }
}

#[test]
fn test_adv_34_multithreaded_parallel_execution_varying_payloads() {
    let tcc = Arc::new(TccEngine::new());
    let code = "int haetae_fuzz_entry(long long p) {\n  return (int)(p * 2);\n}\n";
    let compiled = Arc::new(
        tcc.compile_in_memory(code, "haetae_fuzz_entry")
            .expect("compile shared"),
    );

    let mut handles = Vec::new();
    for idx in NUM_ZERO..NUM_SIXTEEN {
        let tcc_clone = Arc::clone(&tcc);
        let compiled_clone = Arc::clone(&compiled);
        let handle = std::thread::spawn(move || {
            let payload = idx as i64;
            let val = tcc_clone
                .execute_entry_with_payload(&compiled_clone, payload)
                .expect("exec shared");
            assert_eq!(val, (idx * NUM_TWO) as i32);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread join");
    }
}

#[test]
fn test_adv_35_multithreaded_parallel_cfi_verification() {
    let mut handles = Vec::new();
    for _ in NUM_ZERO..NUM_EIGHT {
        let handle = std::thread::spawn(|| {
            let x86_valid = [X86_NOP_OPCODE, X86_RET_OPCODE];
            assert!(verify_cfi_integrity(&x86_valid).is_ok());

            let arm_valid = AARCH64_RET_INSN.to_le_bytes();
            assert!(verify_cfi_for_isa(&arm_valid, TargetIsa::AArch64).is_ok());

            let rv_valid = RISCV_RET_INSN.to_le_bytes();
            assert!(verify_cfi_for_isa(&rv_valid, TargetIsa::RiscV64).is_ok());
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread join");
    }
}

#[test]
fn test_adv_36_concurrent_crash_and_success_isolation() {
    let mut handles = Vec::new();

    // Crashing threads
    for _ in NUM_ZERO..NUM_FOUR {
        let handle = std::thread::spawn(|| {
            let tcc = TccEngine::new();
            let crash_code =
                "int haetae_fuzz_entry(long long p) {\n  int* p_null = 0;\n  return *p_null;\n}\n";
            let compiled = tcc
                .compile_in_memory(crash_code, "haetae_fuzz_entry")
                .expect("compile crash");
            let res = tcc.execute_entry_with_payload(&compiled, NUM_ZERO as i64);
            assert!(res.is_err());
        });
        handles.push(handle);
    }

    // Successful threads
    for _ in NUM_ZERO..NUM_FOUR {
        let handle = std::thread::spawn(|| {
            let tcc = TccEngine::new();
            let safe_code = "int haetae_fuzz_entry(long long p) {\n  return 42;\n}\n";
            let compiled = tcc
                .compile_in_memory(safe_code, "haetae_fuzz_entry")
                .expect("compile safe");
            let val = tcc
                .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
                .expect("exec safe");
            assert_eq!(val, NUM_FORTY_TWO as i32);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread join");
    }
}

#[test]
fn test_adv_37_parallel_fuzzer_engine_iterations() {
    let mut handles = Vec::new();
    for thread_idx in NUM_ZERO..NUM_FOUR {
        let handle = std::thread::spawn(move || {
            let tcc = TccEngine::new();
            let code = format!(
                "int haetae_fuzz_entry(long long p) {{\n  return (int)(p & {});\n}}\n",
                thread_idx + NUM_ONE
            );
            let compiled = tcc
                .compile_in_memory(&code, "haetae_fuzz_entry")
                .expect("compile fuzz code");
            let fuzzer = FuzzerEngine::new(NUM_ONE_HUNDRED);
            let res = fuzzer
                .run_fuzz_loop(&tcc, &compiled)
                .expect("run fuzz loop");
            assert_eq!(res.failed, NUM_ZERO);
            assert_eq!(res.total_iterations, NUM_ONE_HUNDRED);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread join");
    }
}

#[test]
fn test_adv_38_shared_arc_tcc_engine_thread_safety() {
    let tcc = Arc::new(TccEngine::new());
    let mut handles = Vec::new();

    for i in NUM_ZERO..NUM_EIGHT {
        let tcc_ref = Arc::clone(&tcc);
        let handle = std::thread::spawn(move || {
            let code = format!(
                "int haetae_fuzz_entry(long long p) {{\n  int val = {};\n  return val;\n}}\n",
                i
            );
            let compiled = tcc_ref
                .compile_in_memory(&code, "haetae_fuzz_entry")
                .expect("arc compile");
            let out = tcc_ref
                .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
                .expect("arc exec");
            assert_eq!(out, i as i32);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread join");
    }
}

// ============================================================================
// Area 4: Extreme Context & Token Boundaries
// ============================================================================

#[test]
fn test_adv_39_empty_c_code_compilation_error() {
    let tcc = TccEngine::new();
    let res = tcc.compile_in_memory("", "haetae_fuzz_entry");
    assert!(res.is_err());
    match res {
        Err(VerifyError::CompileError(msg)) => {
            assert!(msg.contains("Empty C code"));
        }
        _ => panic!("Expected CompileError"),
    }
}

#[test]
fn test_adv_40_whitespace_only_compilation_error() {
    let tcc = TccEngine::new();
    let res = tcc.compile_in_memory("   \n\t\r   ", "haetae_fuzz_entry");
    assert!(res.is_err());
    match res {
        Err(VerifyError::SymbolNotFound(sym)) => {
            assert_eq!(sym, "haetae_fuzz_entry");
        }
        _ => panic!("Expected SymbolNotFound"),
    }
}

#[test]
fn test_adv_41_massive_binary_garbage_rejection() {
    let tcc = TccEngine::new();
    let garbage =
        "\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x03\x00>\x00\x01\x00\x00\x00";
    let res = tcc.compile_in_memory(garbage, "haetae_fuzz_entry");
    assert!(res.is_err());
    match res {
        Err(VerifyError::SymbolNotFound(sym)) => {
            assert_eq!(sym, "haetae_fuzz_entry");
        }
        _ => panic!("Expected SymbolNotFound"),
    }
}

#[test]
fn test_adv_42_unicode_korean_comments_and_strings() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  /* 해태 가상머신 엔진 극한 신뢰성 테스트 */\n  int safe_val = 100;\n  return safe_val;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile korean comments");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec korean comments");
    assert_eq!(val, NUM_ONE_HUNDRED as i32);
}

#[test]
fn test_adv_43_utf8_emoji_and_special_symbols() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  /* 🛡️ 🔥 🚀 ⚡ 💎 */\n  int emoji_status = 1;\n  return emoji_status;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile emoji code");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec emoji code");
    assert_eq!(val, NUM_ONE as i32);
}

#[test]
fn test_adv_44_tokenizer_boundary_hex_and_suffixes() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) {\n  int hex_a = 0x10;\n  int hex_b = 0x20u;\n  return hex_a + hex_b;\n}\n";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile hex suffixes");
    let val = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec hex suffixes");
    assert_eq!(val, NUM_FORTY_EIGHT as i32);
}

// ============================================================================
// Area 5: Engine Error Recovery & Robustness
// ============================================================================

#[test]
fn test_adv_45_sequential_recovery_after_sigsegv() {
    let tcc = TccEngine::new();
    let crash_code =
        "int haetae_fuzz_entry(long long p) {\n  int* p_null = 0;\n  return *p_null;\n}\n";
    let compiled_crash = tcc
        .compile_in_memory(crash_code, "haetae_fuzz_entry")
        .expect("compile crash");
    let crash_res = tcc.execute_entry_with_payload(&compiled_crash, NUM_ZERO as i64);
    assert!(crash_res.is_err());

    let valid_code = "int haetae_fuzz_entry(long long p) {\n  return 42;\n}\n";
    let compiled_valid = tcc
        .compile_in_memory(valid_code, "haetae_fuzz_entry")
        .expect("compile valid");
    let valid_res = tcc
        .execute_entry_with_payload(&compiled_valid, NUM_ZERO as i64)
        .expect("exec valid after crash");
    assert_eq!(valid_res, NUM_FORTY_TWO as i32);
}

#[test]
fn test_adv_46_sequential_recovery_after_sigfpe() {
    let tcc = TccEngine::new();
    let fpe_code =
        "int haetae_fuzz_entry(long long p) {\n  haetae_trap_sigfpe();\n  return 0;\n}\n";
    let compiled_fpe = tcc
        .compile_in_memory(fpe_code, "haetae_fuzz_entry")
        .expect("compile fpe");
    let fpe_res = tcc.execute_entry_with_payload(&compiled_fpe, NUM_ZERO as i64);
    assert!(fpe_res.is_err());

    let valid_code = "int haetae_fuzz_entry(long long p) {\n  return 100;\n}\n";
    let compiled_valid = tcc
        .compile_in_memory(valid_code, "haetae_fuzz_entry")
        .expect("compile valid");
    let valid_res = tcc
        .execute_entry_with_payload(&compiled_valid, NUM_ZERO as i64)
        .expect("exec valid after fpe");
    assert_eq!(valid_res, NUM_ONE_HUNDRED as i32);
}

#[test]
fn test_adv_47_interleaved_crash_recovery_stress_loop() {
    let tcc = TccEngine::new();
    let crash_code =
        "int haetae_fuzz_entry(long long p) {\n  int* p_null = 0;\n  return *p_null;\n}\n";
    let valid_code = "int haetae_fuzz_entry(long long p) {\n  return 42;\n}\n";

    let compiled_crash = tcc
        .compile_in_memory(crash_code, "haetae_fuzz_entry")
        .expect("compile crash");
    let compiled_valid = tcc
        .compile_in_memory(valid_code, "haetae_fuzz_entry")
        .expect("compile valid");

    for _ in NUM_ZERO..NUM_TEN {
        let crash_res = tcc.execute_entry_with_payload(&compiled_crash, NUM_ZERO as i64);
        assert!(crash_res.is_err());

        let valid_res = tcc
            .execute_entry_with_payload(&compiled_valid, NUM_ZERO as i64)
            .expect("exec valid in loop");
        assert_eq!(valid_res, NUM_FORTY_TWO as i32);
    }
}

#[test]
fn test_adv_48_fuzzer_recovery_after_defect_detection() {
    let tcc = TccEngine::new();
    let fuzzer = FuzzerEngine::new(NUM_ONE_HUNDRED);

    let defective_code = "int haetae_fuzz_entry(long long p) {\n  if (p == 0) {\n    haetae_trap_sigfpe();\n  }\n  return 1;\n}\n";
    let compiled_defective = tcc
        .compile_in_memory(defective_code, "haetae_fuzz_entry")
        .expect("compile defective");
    let res_defect = fuzzer
        .run_fuzz_loop(&tcc, &compiled_defective)
        .expect("fuzz defective");
    assert_eq!(res_defect.failed, NUM_ONE);
    assert_eq!(res_defect.crash_signal, Some(POSIX_SIGFPE));

    let robust_code = "int haetae_fuzz_entry(long long p) {\n  return (int)(p & 1);\n}\n";
    let compiled_robust = tcc
        .compile_in_memory(robust_code, "haetae_fuzz_entry")
        .expect("compile robust");
    let res_robust = fuzzer
        .run_fuzz_loop(&tcc, &compiled_robust)
        .expect("fuzz robust");
    assert_eq!(res_robust.failed, NUM_ZERO);
    assert_eq!(res_robust.total_iterations, NUM_ONE_HUNDRED);
}

#[test]
fn test_adv_49_c_lower_multi_language_with_guards_recovery() {
    let py_src = "def compute(a, b):\n    return a + b\n";
    let lowered = lower_source_to_c_ir(py_src, "py");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);

    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile lowered python");
    let fuzzer = FuzzerEngine::new(NUM_ONE_HUNDRED);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz lowered python");
    assert_eq!(res.failed, NUM_ZERO);
}

#[test]
fn test_adv_50_diagnostic_report_generation_after_adversarial_crash() {
    let report = DiagnosticReport {
        error_type: "SIGSEGV".to_string(),
        file_path: "adversarial_probe.c".to_string(),
        line: NUM_TEN,
        column: NUM_FIVE,
        function_name: "haetae_fuzz_entry".to_string(),
        message: "NULL pointer dereference defect detected".to_string(),
        snippet_prev: Some("int* ptr = 0;".to_string()),
        snippet_curr: "return *ptr;".to_string(),
        underline_label: "dereference of null address 0x0".to_string(),
        iteration_triggered: NUM_ONE,
        total_iterations: NUM_ONE_THOUSAND,
        payload_str: "0".to_string(),
        hardware_detail: "Signal 11 (SEGV_MAPERR)".to_string(),
        metrics_passed: NUM_ZERO,
        metrics_failed: NUM_ONE,
        duration_ms: 1.0,
        stack_frame_bytes: NUM_ZERO,
    };

    let rendered = report.render_box_drawing();
    assert!(rendered.contains("NULL pointer dereference"));
    assert!(rendered.contains("Signal 11"));
    assert!(rendered.contains("dereference of null address"));
}
