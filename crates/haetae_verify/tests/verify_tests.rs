use haetae_core::config::{
    TargetIsa, AARCH64_IMM12_SHIFT, AARCH64_RETAA_INSN, AARCH64_RETAB_INSN, AARCH64_RET_INSN,
    AARCH64_STP_X29_X30_MINUS_16, AARCH64_SUB_SP_EXACT_MATCH, AARCH64_SUB_SP_MATCH,
    AARCH64_SUB_SP_SH_BIT, BENCHMARK_SAMPLE_DURATION_MS, BUFFER_CAPACITY_BYTES,
    MAX_STACK_FRAME_BYTES, NUM_EIGHTY, NUM_FIVE_THOUSAND, NUM_FOUR, NUM_NINETY_SIX, NUM_ONE,
    NUM_ONE_HUNDRED, NUM_ONE_HUNDRED_FIFTY, NUM_ONE_HUNDRED_THOUSAND, NUM_ONE_HUNDRED_TWENTY,
    NUM_ONE_THOUSAND, NUM_ONE_TWENTY_EIGHT, NUM_SIXTEEN, NUM_SIXTY, NUM_SIXTY_FOUR, NUM_TEN,
    NUM_THIRTY_TWO, NUM_TWENTY, NUM_TWO_HUNDRED, NUM_TWO_THOUSAND, NUM_ZERO,
    RISCV_ADDI_SP_MINUS_64, RISCV_C_JR_RA_INSN, RISCV_RET_INSN, X86_NOP_OPCODE, X86_RET_OPCODE,
    X86_REX_W, X86_RSP_MODRM, X86_SUB_IMM8_OP,
};
use haetae_verify::diagnostics::DiagnosticReport;
use haetae_verify::fuzzer::FuzzerEngine;
use haetae_verify::headers::bundle_headers;
use haetae_verify::tcc_engine::{TccEngine, VerifyError};
use haetae_verify::zydis_cfi::{
    detect_isa, get_verifier, verify_cfi_for_isa, verify_cfi_integrity, verify_stack_frame_bound,
    verify_stack_frame_bound_for_isa,
};

#[test]
fn test_headers_bundled() {
    let headers = bundle_headers();
    assert!(headers.contains("PT_H"));
    assert!(headers.contains("CEXCEPTION_H"));
}

#[test]
fn test_diagnostic_render_success() {
    let line = DiagnosticReport::render_success_line(
        NUM_ONE_THOUSAND,
        BENCHMARK_SAMPLE_DURATION_MS,
        NUM_ONE_HUNDRED,
    );
    assert!(line.contains("1000 runs verified"));
    assert!(line.contains("memory: safe"));
}

#[test]
fn test_diagnostic_box_drawing() {
    let report = DiagnosticReport {
        error_type: "SIGFPE".to_string(),
        file_path: "test.c".to_string(),
        line: NUM_ONE,
        column: NUM_ONE,
        function_name: "test_fn".to_string(),
        message: "Division by zero".to_string(),
        snippet_prev: None,
        snippet_curr: "return a / b;".to_string(),
        underline_label: "zero denominator".to_string(),
        iteration_triggered: NUM_ONE,
        total_iterations: NUM_ONE_THOUSAND,
        payload_str: "0".to_string(),
        hardware_detail: "Signal 8".to_string(),
        metrics_passed: NUM_ONE_THOUSAND - NUM_ONE,
        metrics_failed: NUM_ONE,
        duration_ms: 1.0,
        stack_frame_bytes: NUM_ONE_HUNDRED,
    };
    let box_str = report.render_box_drawing();
    assert!(box_str.contains("error[SIGFPE]"));
    assert!(box_str.contains("test.c:1:1"));
}

#[test]
fn test_cfi_integrity_safe_bytecode() {
    let mut safe_bytecode = vec![X86_NOP_OPCODE; NUM_TEN];
    safe_bytecode.push(X86_RET_OPCODE);
    assert!(verify_cfi_integrity(&safe_bytecode).is_ok());
}

#[test]
fn test_stack_frame_bound_safe() {
    let bytecode = vec![X86_RET_OPCODE; NUM_TEN];
    assert!(verify_stack_frame_bound(&bytecode, MAX_STACK_FRAME_BYTES).is_ok());
}

#[test]
fn test_tcc_compile_and_fuzz_loop() {
    let tcc = TccEngine::new();
    let code_str =
        "int haetae_fuzz_entry(long long p) { if (p == 0) return 0; return (int)(p & 1); }";
    if let Ok(compiled) = tcc.compile_in_memory(code_str, "haetae_fuzz_entry") {
        let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
        let res = fuzzer.run_fuzz_loop(&tcc, &compiled);
        assert!(res.is_ok());
        if let Ok(r) = res {
            assert_eq!(r.failed, NUM_ZERO);
            assert_eq!(r.total_iterations, NUM_ONE_THOUSAND);
        }
    }
}

#[test]
fn test_lower_source_to_c_ir_python_guarded() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let py_src = "def divide_scalar(base_val: int, factor: int) -> int:\n    if factor == 0:\n        return 0\n    return base_val // factor\n";
    let lowered = lower_source_to_c_ir(py_src, "py");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile failed");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz failed");
    assert_eq!(res.failed, NUM_ZERO);
}

#[test]
fn test_lower_source_to_c_ir_python_unguarded_crash() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let py_src =
        "def divide_scalar(base_val: int, factor: int) -> int:\n    return base_val // factor\n";
    let lowered = lower_source_to_c_ir(py_src, "py");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile failed");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz failed");
    assert_eq!(res.failed, NUM_ONE);
}

#[test]
fn test_lower_source_to_c_ir_multi_languages() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let tcc = TccEngine::new();
    let test_cases = [
        (
            "function divideScalar(baseVal: number, factor: number): number { if (factor === 0) return 0; return baseVal / factor; }",
            "ts",
        ),
        (
            "pub fn divide_scalar(base_val: i32, factor: i32) -> i32 { if factor == 0 { return 0; } base_val / factor }",
            "rs",
        ),
        (
            "func DivideScalar(baseVal int, factor int) int { if factor == 0 { return 0 } return baseVal / factor }",
            "go",
        ),
        (
            "public int divideScalar(int baseVal, int factor) { if (factor == 0) return 0; return baseVal / factor; }",
            "java",
        ),
        (
            "public static int DivideScalar(int baseVal, int factor) { if (factor == 0) return 0; return baseVal / factor; }",
            "cs",
        ),
        (
            "fun divideScalar(baseVal: Int, factor: Int): Int { if (factor == 0) return 0; return baseVal / factor; }",
            "kt",
        ),
        (
            "func divideScalar(_ baseVal: Int, _ factor: Int) -> Int { if factor == 0 { return 0 } return baseVal / factor }",
            "swift",
        ),
        (
            "function divideScalar($baseVal, $factor) { if ($factor == 0) return 0; return $baseVal / $factor; }",
            "php",
        ),
        (
            "def divide_scalar(\n    base_val: int,\n    factor: int,\n) -> int:\n    if factor == 0:\n        return 0\n    return base_val // factor\n",
            "py",
        ),
        (
            "function divideScalar(\n    baseVal: number,\n    factor: number\n): number {\n    if (factor === 0) return 0;\n    return baseVal / factor;\n}",
            "ts",
        ),
        (
            "function branchlessSelect(cond: number, a: number, b: number): number { const mask = -cond; return (a & mask) | (b & ~mask); }",
            "ts",
        ),
    ];

    for (src, ext) in &test_cases {
        let lowered = lower_source_to_c_ir(src, ext);
        let full = format!("{}\n{}\n", bundle_headers(), lowered);
        let compiled = tcc
            .compile_in_memory(&full, "haetae_fuzz_entry")
            .expect("compile failed");
        let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
        let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz failed");
        assert_eq!(res.failed, NUM_ZERO);
    }
}

#[test]
fn test_jit_micro_fuzzing_hundred_thousand_iterations() {
    let tcc = TccEngine::new();
    let safe_code =
        "int haetae_fuzz_entry(long long p) { if (p == 0) return 0; return (int)(p & 1); }";
    let compiled = tcc
        .compile_in_memory(safe_code, "haetae_fuzz_entry")
        .expect("compile safe code");
    let fuzzer = FuzzerEngine::new(NUM_ONE_HUNDRED_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz 100k loop");
    assert_eq!(res.total_iterations, NUM_ONE_HUNDRED_THOUSAND);
    assert_eq!(res.failed, NUM_ZERO);
}

#[test]
fn test_extreme_boundary_payloads() {
    let payloads = FuzzerEngine::edge_payloads();
    assert!(!payloads.is_empty());
    let zero_val = NUM_ZERO as i64;
    let one_val = NUM_ONE as i64;
    assert!(payloads.contains(&zero_val));
    assert!(payloads.contains(&one_val));
    assert!(payloads.contains(&i64::MIN));
    assert!(payloads.contains(&i64::MAX));
}

#[test]
fn test_stack_frame_bound_various_limits() {
    let bytecode = vec![X86_RET_OPCODE; NUM_SIXTY_FOUR];
    assert!(verify_stack_frame_bound(&bytecode, NUM_ONE_HUNDRED).is_ok());
    assert!(verify_stack_frame_bound(&bytecode, MAX_STACK_FRAME_BYTES).is_ok());
}

#[test]
fn test_twenty_diverse_algorithms_fuzzing() {
    let tcc = TccEngine::new();
    let codes = [
        "int haetae_fuzz_entry(long long p) { int mask = -(int)(p & 1); return ((int)p & mask); }",
        "int haetae_fuzz_entry(long long p) { if ((p & 1) == 0) return 0; return (int)p; }",
        "int haetae_fuzz_entry(long long p) { int a[5] = {1, 2, 3, 4, 5}; return a[(p & 3)]; }",
        "int haetae_fuzz_entry(long long p) { return (int)(p & 255); }",
        "int haetae_fuzz_entry(long long p) { unsigned int c = 0xFFFFFFFF; return (int)(c ^ (unsigned int)p); }",
        "int haetae_fuzz_entry(long long p) { return (p > 0) ? 1 : 0; }",
        "int haetae_fuzz_entry(long long p) { return (int)(p & 1); }",
        "int haetae_fuzz_entry(long long p) { float a = (float)(p & 7); return (int)(a * 2.0f); }",
        "int haetae_fuzz_entry(long long p) { int dp[4] = {0}; dp[1] = (int)(p & 3); return dp[1]; }",
        "int haetae_fuzz_entry(long long p) { return (int)p; }",
        "int haetae_fuzz_entry(long long p) { if (p <= 0) return 0; return (int)(p >> 1); }",
        "int haetae_fuzz_entry(long long p) { int next = ((int)p + 1) & 15; return next; }",
        "int haetae_fuzz_entry(long long p) { unsigned char b = (unsigned char)(p & 255); return (b < 128) ? 1 : 2; }",
        "int haetae_fuzz_entry(long long p) { int mat[4] = {(int)p, 1, 2, 3}; return mat[2]; }",
        "int haetae_fuzz_entry(long long p) { unsigned int f = (1u << ((int)p & 31)); return (f != 0) ? 1 : 0; }",
        "int haetae_fuzz_entry(long long p) { int a[2] = {1, (int)(p & 7)}; return a[0]; }",
        "int haetae_fuzz_entry(long long p) { if (p <= 0) return 0; return (int)(p & 63); }",
        "int haetae_fuzz_entry(long long p) { int tree[3] = {1, 2, 3}; return tree[(p & 1)]; }",
        "int haetae_fuzz_entry(long long p) { long long f = (p & 1) * 2; return (int)f; }",
        "int haetae_fuzz_entry(long long p) { int q[4] = {(int)p, 0, 0, 0}; return q[0]; }",
    ];
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let mut verified = NUM_ZERO;
    for code in &codes {
        let compiled = tcc
            .compile_in_memory(code, "haetae_fuzz_entry")
            .expect("compile entry");
        let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz run");
        assert_eq!(res.failed, NUM_ZERO);
        verified += NUM_ONE;
    }
    assert_eq!(verified, NUM_TWENTY);
}

#[test]
fn test_stack_frame_bound_overflow_detected() {
    let mut bytecode = vec![X86_NOP_OPCODE; NUM_TEN];
    bytecode.push(X86_REX_W);
    bytecode.push(X86_SUB_IMM8_OP);
    bytecode.push(X86_RSP_MODRM);
    bytecode.push(NUM_TWO_HUNDRED as u8);
    bytecode.push(X86_RET_OPCODE);
    let res = verify_stack_frame_bound(&bytecode, NUM_ONE_HUNDRED);
    assert!(res.is_err());
    match res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, NUM_TWO_HUNDRED);
            assert_eq!(limit, NUM_ONE_HUNDRED);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_cfi_integrity_violation_missing_ret() {
    let bytecode = vec![X86_NOP_OPCODE; NUM_SIXTY_FOUR];
    let res = verify_cfi_integrity(&bytecode);
    assert!(res.is_err());
    match res {
        Err(VerifyError::CfiViolation(_)) => {}
        _ => panic!("expected CfiViolation error"),
    }
}

#[test]
fn test_cfi_integrity_aarch64_valid_ret() {
    let mut bytecode = vec![0u8; NUM_FOUR];
    bytecode.copy_from_slice(&AARCH64_RET_INSN.to_le_bytes());
    assert!(verify_cfi_integrity(&bytecode).is_ok());
}

#[test]
fn test_aarch64_stack_frame_bound_pass_and_fail() {
    let allocated_bytes: u32 = NUM_ONE_TWENTY_EIGHT as u32;
    let insn = AARCH64_SUB_SP_MATCH | (allocated_bytes << AARCH64_IMM12_SHIFT);
    let mut bytecode = vec![0u8; NUM_FOUR];
    bytecode.copy_from_slice(&insn.to_le_bytes());
    assert!(verify_stack_frame_bound(&bytecode, NUM_TWO_HUNDRED).is_ok());
    let overflow_res = verify_stack_frame_bound(&bytecode, NUM_ONE_HUNDRED);
    assert!(overflow_res.is_err());
}

#[test]
fn test_branchless_extreme_payload_simulation() {
    let tcc = TccEngine::new();
    let code =
        "int haetae_fuzz_entry(long long p) { int mask = -(int)(p & 1); return ((int)p & mask); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile branchless");
    let payloads = [
        NUM_ZERO as i64,
        NUM_ONE as i64,
        haetae_core::config::NUM_NEG_ONE,
        i64::MIN,
        i64::MAX,
        i32::MIN as i64,
        i32::MAX as i64,
    ];
    for p in payloads {
        let res = tcc.execute_entry_with_payload(&compiled, p);
        assert!(res.is_ok());
    }
}

#[test]
fn test_aarch64_cross_isa_c3_interference_isolated() {
    let fake_aarch64_bytecode = vec![
        NUM_ZERO as u8,
        NUM_ZERO as u8,
        X86_RET_OPCODE,
        NUM_ZERO as u8,
    ];
    let res = verify_cfi_for_isa(&fake_aarch64_bytecode, TargetIsa::AArch64);
    assert!(res.is_err());
    match res {
        Err(VerifyError::CfiViolation(_)) => {}
        _ => panic!("expected CfiViolation on isolated AArch64 check"),
    }
}

#[test]
fn test_aarch64_stp_stack_frame_bound_pass_and_fail() {
    let bytecode = AARCH64_STP_X29_X30_MINUS_16.to_le_bytes().to_vec();
    assert!(verify_stack_frame_bound_for_isa(&bytecode, NUM_TWENTY, TargetIsa::AArch64).is_ok());
    let overflow_res = verify_stack_frame_bound_for_isa(&bytecode, NUM_TEN, TargetIsa::AArch64);
    assert!(overflow_res.is_err());
    match overflow_res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, NUM_SIXTEEN);
            assert_eq!(limit, NUM_TEN);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_aarch64_shifted_immediate_sub_sp() {
    let insn = AARCH64_SUB_SP_EXACT_MATCH
        | AARCH64_SUB_SP_SH_BIT
        | ((NUM_ONE as u32) << AARCH64_IMM12_SHIFT);
    let bytecode = insn.to_le_bytes().to_vec();
    assert!(
        verify_stack_frame_bound_for_isa(&bytecode, NUM_FIVE_THOUSAND, TargetIsa::AArch64).is_ok()
    );
    let overflow_res =
        verify_stack_frame_bound_for_isa(&bytecode, NUM_TWO_THOUSAND, TargetIsa::AArch64);
    assert!(overflow_res.is_err());
    match overflow_res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, BUFFER_CAPACITY_BYTES);
            assert_eq!(limit, NUM_TWO_THOUSAND);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_riscv_cfi_integrity_and_compressed_ret() {
    let valid_ret = RISCV_RET_INSN.to_le_bytes().to_vec();
    assert!(verify_cfi_for_isa(&valid_ret, TargetIsa::RiscV64).is_ok());

    let compressed_ret = RISCV_C_JR_RA_INSN.to_le_bytes().to_vec();
    assert!(verify_cfi_for_isa(&compressed_ret, TargetIsa::RiscV64).is_ok());

    let invalid_code = vec![X86_NOP_OPCODE; NUM_FOUR];
    assert!(verify_cfi_for_isa(&invalid_code, TargetIsa::RiscV64).is_err());
}

#[test]
fn test_riscv_stack_frame_bound_pass_and_fail() {
    let bytecode = RISCV_ADDI_SP_MINUS_64.to_le_bytes().to_vec();
    assert!(
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED, TargetIsa::RiscV64).is_ok()
    );
    let overflow_res =
        verify_stack_frame_bound_for_isa(&bytecode, NUM_THIRTY_TWO, TargetIsa::RiscV64);
    assert!(overflow_res.is_err());
    match overflow_res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, NUM_SIXTY_FOUR);
            assert_eq!(limit, NUM_THIRTY_TWO);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_target_isa_parsing_and_verifier_dispatch() {
    assert_eq!(TargetIsa::from_str("x86_64"), Some(TargetIsa::X86_64));
    assert_eq!(TargetIsa::from_str("x64"), Some(TargetIsa::X86_64));
    assert_eq!(TargetIsa::from_str("amd64"), Some(TargetIsa::X86_64));
    assert_eq!(TargetIsa::from_str("aarch64"), Some(TargetIsa::AArch64));
    assert_eq!(TargetIsa::from_str("arm64"), Some(TargetIsa::AArch64));
    assert_eq!(TargetIsa::from_str("riscv64"), Some(TargetIsa::RiscV64));
    assert_eq!(TargetIsa::from_str("rv64"), Some(TargetIsa::RiscV64));
    assert_eq!(TargetIsa::from_str("unknown_isa"), None);

    assert_eq!(get_verifier(TargetIsa::X86_64).isa(), TargetIsa::X86_64);
    assert_eq!(get_verifier(TargetIsa::AArch64).isa(), TargetIsa::AArch64);
    assert_eq!(get_verifier(TargetIsa::RiscV64).isa(), TargetIsa::RiscV64);

    assert_eq!(TargetIsa::X86_64.as_str(), "x86_64");
    assert_eq!(TargetIsa::AArch64.as_str(), "aarch64");
    assert_eq!(TargetIsa::RiscV64.as_str(), "riscv64");
}

#[test]
fn test_detect_isa_multi_architecture() {
    let unaligned = vec![X86_NOP_OPCODE; NUM_TEN + NUM_ONE];
    assert_eq!(detect_isa(&unaligned), Some(TargetIsa::X86_64));

    let aarch64_code = AARCH64_RET_INSN.to_le_bytes().to_vec();
    assert_eq!(detect_isa(&aarch64_code), Some(TargetIsa::AArch64));

    let riscv_code = RISCV_RET_INSN.to_le_bytes().to_vec();
    assert_eq!(detect_isa(&riscv_code), Some(TargetIsa::RiscV64));
}

#[test]
fn test_empty_bytecode_across_all_verifiers() {
    let empty: [u8; NUM_ZERO] = [];
    assert!(verify_cfi_for_isa(&empty, TargetIsa::X86_64).is_ok());
    assert!(verify_cfi_for_isa(&empty, TargetIsa::AArch64).is_ok());
    assert!(verify_cfi_for_isa(&empty, TargetIsa::RiscV64).is_ok());
    assert!(verify_stack_frame_bound_for_isa(&empty, NUM_ONE_HUNDRED, TargetIsa::X86_64).is_ok());
    assert!(verify_stack_frame_bound_for_isa(&empty, NUM_ONE_HUNDRED, TargetIsa::AArch64).is_ok());
    assert!(verify_stack_frame_bound_for_isa(&empty, NUM_ONE_HUNDRED, TargetIsa::RiscV64).is_ok());
}

#[test]
fn test_aarch64_cumulative_stack_frame_multi_instruction_overflow() {
    let mut bytecode = Vec::new();
    bytecode.extend_from_slice(&AARCH64_STP_X29_X30_MINUS_16.to_le_bytes());
    let sub_insn = AARCH64_SUB_SP_MATCH | ((NUM_EIGHTY as u32) << AARCH64_IMM12_SHIFT);
    bytecode.extend_from_slice(&sub_insn.to_le_bytes());

    assert!(
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED, TargetIsa::AArch64).is_ok()
    );
    let overflow_res = verify_stack_frame_bound_for_isa(&bytecode, NUM_EIGHTY, TargetIsa::AArch64);
    assert!(overflow_res.is_err());
    match overflow_res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, NUM_NINETY_SIX);
            assert_eq!(limit, NUM_EIGHTY);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_x86_cumulative_stack_frame_multi_instruction_overflow() {
    let bytecode = vec![
        X86_REX_W,
        X86_SUB_IMM8_OP,
        X86_RSP_MODRM,
        NUM_SIXTY as u8,
        X86_REX_W,
        X86_SUB_IMM8_OP,
        X86_RSP_MODRM,
        NUM_SIXTY as u8,
    ];

    assert!(
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED_FIFTY, TargetIsa::X86_64)
            .is_ok()
    );
    let overflow_res =
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED, TargetIsa::X86_64);
    assert!(overflow_res.is_err());
    match overflow_res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, NUM_ONE_HUNDRED_TWENTY);
            assert_eq!(limit, NUM_ONE_HUNDRED);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_riscv_cumulative_stack_frame_multi_instruction_overflow() {
    let mut bytecode = Vec::new();
    bytecode.extend_from_slice(&RISCV_ADDI_SP_MINUS_64.to_le_bytes());
    bytecode.extend_from_slice(&RISCV_ADDI_SP_MINUS_64.to_le_bytes());

    assert!(
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED_FIFTY, TargetIsa::RiscV64)
            .is_ok()
    );
    let overflow_res =
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED, TargetIsa::RiscV64);
    assert!(overflow_res.is_err());
    match overflow_res {
        Err(VerifyError::StackOverflow(allocated, limit)) => {
            assert_eq!(allocated, NUM_ONE_TWENTY_EIGHT);
            assert_eq!(limit, NUM_ONE_HUNDRED);
        }
        _ => panic!("expected StackOverflow error"),
    }
}

#[test]
fn test_riscv_cfi_with_prefix_instruction_4byte_buffer() {
    let mut bytecode = Vec::new();
    bytecode.push(NUM_ONE as u8);
    bytecode.push(NUM_ZERO as u8);
    bytecode.extend_from_slice(&RISCV_C_JR_RA_INSN.to_le_bytes());
    assert_eq!(bytecode.len(), NUM_FOUR);
    assert!(verify_cfi_for_isa(&bytecode, TargetIsa::RiscV64).is_ok());
}

#[test]
fn test_aarch64_pac_ret_cfi() {
    let retaa_code = AARCH64_RETAA_INSN.to_le_bytes().to_vec();
    assert!(verify_cfi_for_isa(&retaa_code, TargetIsa::AArch64).is_ok());

    let retab_code = AARCH64_RETAB_INSN.to_le_bytes().to_vec();
    assert!(verify_cfi_for_isa(&retab_code, TargetIsa::AArch64).is_ok());
}

#[test]
fn test_stack_frame_resets_on_function_return() {
    let bytecode = vec![
        X86_REX_W,
        X86_SUB_IMM8_OP,
        X86_RSP_MODRM,
        NUM_EIGHTY as u8,
        X86_RET_OPCODE,
        X86_REX_W,
        X86_SUB_IMM8_OP,
        X86_RSP_MODRM,
        NUM_EIGHTY as u8,
        X86_RET_OPCODE,
    ];

    assert!(
        verify_stack_frame_bound_for_isa(&bytecode, NUM_ONE_HUNDRED, TargetIsa::X86_64).is_ok()
    );
}

#[test]
fn test_real_c_null_pointer_dereference_defect_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let c_src = "int crash_null(void) {\n    int* p = 0;\n    return *p;\n}\n";
    let lowered = lower_source_to_c_ir(c_src, "c");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile crash_null C code");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGSEGV));
}

#[test]
fn test_real_c_division_by_zero_defect_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let c_src = "int crash_div(int a, int b) {\n    return a / b;\n}\n";
    let lowered = lower_source_to_c_ir(c_src, "c");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile crash_div C code");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGFPE));
}

#[test]
fn test_single_line_c_null_pointer_dereference_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let c_src = "int crash_null() { int* p = 0; return *p; }";
    let lowered = lower_source_to_c_ir(c_src, "c");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile single line crash_null");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGSEGV));
}

#[test]
fn test_single_line_python_division_by_zero_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let py_src = "def divide(a, b): return a / b";
    let lowered = lower_source_to_c_ir(py_src, "py");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile single line py div");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGFPE));
}

#[test]
fn test_single_line_rust_division_by_zero_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let rs_src = "fn divide(a: i32, b: i32) -> i32 { a / b }";
    let lowered = lower_source_to_c_ir(rs_src, "rs");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile single line rs div");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGFPE));
}

#[test]
fn test_single_line_typescript_division_by_zero_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let ts_src = "function divide(a: number, b: number): number { return a / b; }";
    let lowered = lower_source_to_c_ir(ts_src, "ts");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile single line ts div");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGFPE));
}

#[test]
fn test_single_line_go_division_by_zero_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let go_src = "func divide(a int, b int) int { return a / b }";
    let lowered = lower_source_to_c_ir(go_src, "go");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile single line go div");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz crash probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGFPE));
}

#[test]
fn test_dynamic_list_indexing_bounds_crash_detected() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let py_src = "def get_item(idx):\n    lst = [10, 20]\n    return lst[idx]\n";
    let lowered = lower_source_to_c_ir(py_src, "py");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile dynamic bounds py");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz bounds probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGSEGV));
}

#[test]
fn test_multi_function_dispatch_and_verification() {
    use haetae_verify::c_lower::lower_source_to_c_ir;
    let py_src =
        "def safe_add(a, b):\n    return a + b\n\ndef unsafe_div(a, b):\n    return a / b\n";
    let lowered = lower_source_to_c_ir(py_src, "py");
    let full = format!("{}\n{}\n", bundle_headers(), lowered);
    let tcc = TccEngine::new();
    let compiled = tcc
        .compile_in_memory(&full, "haetae_fuzz_entry")
        .expect("compile multi-func py");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz multi-func probe");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGFPE));
}

#[test]
fn test_genuine_unbounded_recursion_stack_overflow_detected() {
    let tcc = TccEngine::new();
    let infinite_recurse_code =
        "int haetae_fuzz_entry(long long p) { return haetae_fuzz_entry(p + 1); }";
    let compiled = tcc
        .compile_in_memory(infinite_recurse_code, "haetae_fuzz_entry")
        .expect("compile infinite recursion");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz infinite recursion");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGSEGV));
}

#[test]
fn test_genuine_safe_recursion_with_base_case_succeeds() {
    let tcc = TccEngine::new();
    let safe_recurse_code = "int safe_recurse(int n) { if (n <= 0) return 0; return safe_recurse(n - 1); } int haetae_fuzz_entry(long long p) { return safe_recurse((int)(p & 7)); }";
    let compiled = tcc
        .compile_in_memory(safe_recurse_code, "haetae_fuzz_entry")
        .expect("compile safe recursion");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz safe recursion");
    assert_eq!(res.failed, NUM_ZERO);
}

#[test]
fn test_genuine_large_local_array_stack_overflow_detected() {
    let tcc = TccEngine::new();
    let large_stack_code =
        "int haetae_fuzz_entry(long long p) { char huge_buf[2000000]; return (int)(p & 1); }";
    let compiled = tcc
        .compile_in_memory(large_stack_code, "haetae_fuzz_entry")
        .expect("compile large stack code");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz large stack");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(haetae_core::config::POSIX_SIGSEGV));
}

#[test]
fn test_genuine_safe_local_array_succeeds() {
    let tcc = TccEngine::new();
    let safe_stack_code =
        "int haetae_fuzz_entry(long long p) { char safe_buf[1024]; return (int)(p & 1); }";
    let compiled = tcc
        .compile_in_memory(safe_stack_code, "haetae_fuzz_entry")
        .expect("compile safe stack code");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz safe stack");
    assert_eq!(res.failed, NUM_ZERO);
}
