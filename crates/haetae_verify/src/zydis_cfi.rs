use crate::tcc_engine::VerifyError;
pub use haetae_core::config::{
    TargetIsa, AARCH64_ADD_SP_MASK, AARCH64_ADD_SP_MATCH, AARCH64_BL_MATCH,
    AARCH64_BRANCH_ALIGN_SHIFT, AARCH64_BRANCH_SCALE_BYTES, AARCH64_BTI_MATCH, AARCH64_B_BL_MASK,
    AARCH64_B_COND_MASK, AARCH64_B_COND_MATCH, AARCH64_B_MATCH, AARCH64_CBNZ_MATCH,
    AARCH64_CBZ_CBNZ_MASK, AARCH64_CBZ_MATCH, AARCH64_IMM12_MASK, AARCH64_IMM12_SHIFT,
    AARCH64_IMM19_MASK, AARCH64_IMM19_SHIFT, AARCH64_IMM19_SIGN_BIT, AARCH64_IMM19_SIGN_EXTEND,
    AARCH64_IMM26_MASK, AARCH64_IMM26_SIGN_BIT, AARCH64_IMM26_SIGN_EXTEND, AARCH64_INSN_BYTES,
    AARCH64_LDP_POST_MASK, AARCH64_LDP_POST_MATCH, AARCH64_RETAA_INSN, AARCH64_RETAB_INSN,
    AARCH64_RET_BASE, AARCH64_RET_INSN, AARCH64_RET_MASK, AARCH64_STP_IMM7_MASK,
    AARCH64_STP_IMM7_SHIFT, AARCH64_STP_IMM7_SIGN_BIT, AARCH64_STP_PRE_MASK, AARCH64_STP_PRE_MATCH,
    AARCH64_STP_SCALE_BYTES, AARCH64_STP_SIGN_EXTEND, AARCH64_SUB_SP_EXACT_MASK,
    AARCH64_SUB_SP_EXACT_MATCH, AARCH64_SUB_SP_MASK, AARCH64_SUB_SP_MATCH,
    AARCH64_SUB_SP_SH_AMOUNT, AARCH64_SUB_SP_SH_BIT, BASE_DECIMAL, BISECTION_DIVISOR,
    BUFFER_CAPACITY_BYTES, BYTE_ONE, BYTE_ZERO, DAYS_PER_WEEK, ISA_SCORE_STRONG_MARKER,
    ISA_SCORE_WEAK_MARKER, MAX_STACK_FRAME_BYTES, RISCV_ADDI_IMM12_MASK,
    RISCV_ADDI_IMM12_SHIFT, RISCV_ADDI_IMM12_SIGN_BIT, RISCV_ADDI_SIGN_EXTEND, RISCV_ADDI_SP_MASK,
    RISCV_ADDI_SP_MATCH, RISCV_BRANCH_ALIGN_SHIFT, RISCV_BRANCH_OPCODE_MATCH,
    RISCV_COMPRESSED_INSN_BYTES, RISCV_C_ADDI16SP_MASK, RISCV_C_ADDI16SP_MATCH,
    RISCV_C_ADDI16SP_SIGN_BIT, RISCV_C_BEQZ_MASK, RISCV_C_BEQZ_MATCH, RISCV_C_BNEZ_MATCH,
    RISCV_C_JALR_RA_INSN, RISCV_C_JR_RA_INSN, RISCV_C_J_MASK, RISCV_C_J_MATCH,
    RISCV_INSN_32BIT_MARKER, RISCV_INSN_BYTES, RISCV_INSN_LEN_MASK, RISCV_JAL_OPCODE_MASK,
    RISCV_JAL_OPCODE_MATCH, RISCV_RET_INSN, STEP_TRIO, VSIX_MAX_SIZE_MB, X86_ADD_IMM32_OP,
    X86_ADD_IMM8_OP, X86_ADD_REG_CODE, X86_ENDBR64_B0, X86_ENDBR64_B1, X86_ENDBR64_B2,
    X86_ENDBR64_B3, X86_ENTER_OP, X86_LEAVE_OP, X86_MAX_INSN_LEN, X86_MODRM_MOD_DIRECT,
    X86_MODRM_MOD_DISP32, X86_MODRM_MOD_DISP8, X86_MODRM_MOD_MASK, X86_MODRM_REG_MASK,
    X86_MODRM_RM_DISP32_ONLY, X86_MODRM_RM_MASK, X86_MODRM_RM_SIB, X86_MOV_RBP_RSP_B0,
    X86_MOV_RBP_RSP_B1, X86_MOV_RBP_RSP_B2, X86_NOP_OPCODE, X86_OPCODE_CALL_REL32,
    X86_OPCODE_GRP1_IMM32, X86_OPCODE_GRP1_IMM8, X86_OPCODE_GRP3_NOT_NEG,
    X86_OPCODE_GRP5_JMP_CALL, X86_OPCODE_INT3, X86_OPCODE_JCC_REL32_BASE,
    X86_OPCODE_JCC_REL32_COUNT, X86_OPCODE_JCC_REL8_BASE, X86_OPCODE_JCC_REL8_COUNT,
    X86_OPCODE_JMP_REL32, X86_OPCODE_JMP_REL8, X86_OPCODE_LEA, X86_OPCODE_MOV_RM_R,
    X86_OPCODE_MOV_R_RM, X86_OPCODE_POP_REG_MAX, X86_OPCODE_POP_REG_MIN,
    X86_OPCODE_PUSH_REG_MAX, X86_OPCODE_PUSH_REG_MIN, X86_OPCODE_TWO_BYTE_ESCAPE,
    X86_PREFIX_ADDRESS_SIZE, X86_PREFIX_CS_OVERRIDE, X86_PREFIX_DS_OVERRIDE,
    X86_PREFIX_ES_OVERRIDE, X86_PREFIX_FS_OVERRIDE, X86_PREFIX_GS_OVERRIDE, X86_PREFIX_LOCK,
    X86_PREFIX_OPERAND_SIZE, X86_PREFIX_REP, X86_PREFIX_REPNE, X86_PREFIX_SS_OVERRIDE,
    X86_PUSH_RBP, X86_REG_RSP_CODE, X86_RET_FAR_IMM16_OP, X86_RET_FAR_OP, X86_RET_IMM16_OP,
    X86_RET_OP, X86_REX_MAX, X86_REX_MIN, X86_REX_W, X86_RSP_ADD_MODRM, X86_RSP_MODRM,
    X86_SIB_BASE_DISP32_ONLY, X86_SUB_IMM32_OP, X86_SUB_IMM8_OP, X86_SUB_REG_CODE,
};
use std::collections::HashSet;

pub const STACK_FRAME_4KB_BOUND: usize = BUFFER_CAPACITY_BYTES;

pub const X86_MODRM_REG_SHIFT: u32 = STEP_TRIO as u32;
pub const X86_DISP32_BYTES: usize = std::mem::size_of::<u32>();
pub const X86_IMM16_BYTES: usize = std::mem::size_of::<u16>();
pub const X86_IMM32_BYTES: usize = std::mem::size_of::<u32>();
pub const X86_IMM64_BYTES: usize = std::mem::size_of::<u64>();
pub const X86_REL32_BYTES: usize = std::mem::size_of::<u32>();
pub const X86_ENTER_OPERANDS_BYTES: usize = STEP_TRIO;
pub const X86_REX_W_FLAG: u8 = 1 << (STEP_TRIO as u8);
pub const X86_PROLOGUE_SEQ_BYTES: usize = STEP_TRIO;
pub const X86_PROLOGUE_RSP_OFFSET: usize = BISECTION_DIVISOR;
pub const X86_ENDBR64_BYTES: usize = std::mem::size_of::<u32>();

pub const X86_OPCODE_MOV_R_IMM_MIN: u8 = 0xb8;
pub const X86_OPCODE_MOV_R_IMM_MAX: u8 = 0xbf;
pub const X86_OPCODE_MOV_RM8_IMM8: u8 = 0xc6;
pub const X86_OPCODE_MOV_RM_IMM: u8 = 0xc7;
pub const X86_OPCODE_IMUL_IMM32: u8 = 0x69;
pub const X86_OPCODE_IMUL_IMM8: u8 = 0x6b;
pub const X86_OPCODE_SHIFT_IMM8_MIN: u8 = 0xc0;
pub const X86_OPCODE_SHIFT_IMM8_MAX: u8 = 0xc1;
pub const X86_OPCODE_SHIFT_1_CL_MIN: u8 = 0xd0;
pub const X86_OPCODE_SHIFT_1_CL_MAX: u8 = 0xd3;
pub const X86_OPCODE_ADD_RM_R_MIN: u8 = 0x00;
pub const X86_OPCODE_ADD_RM_R_MAX: u8 = 0x03;
pub const X86_OPCODE_OR_RM_R_MIN: u8 = 0x08;
pub const X86_OPCODE_OR_RM_R_MAX: u8 = 0x0b;
pub const X86_OPCODE_ADC_RM_R_MIN: u8 = 0x10;
pub const X86_OPCODE_ADC_RM_R_MAX: u8 = 0x13;
pub const X86_OPCODE_SBB_RM_R_MIN: u8 = 0x18;
pub const X86_OPCODE_SBB_RM_R_MAX: u8 = 0x1b;
pub const X86_OPCODE_AND_RM_R_MIN: u8 = 0x20;
pub const X86_OPCODE_AND_RM_R_MAX: u8 = 0x23;
pub const X86_OPCODE_SUB_RM_R_MIN: u8 = 0x28;
pub const X86_OPCODE_SUB_RM_R_MAX: u8 = 0x2b;
pub const X86_OPCODE_XOR_RM_R_MIN: u8 = 0x30;
pub const X86_OPCODE_XOR_RM_R_MAX: u8 = 0x33;
pub const X86_OPCODE_CMP_RM_R_MIN: u8 = 0x38;
pub const X86_OPCODE_CMP_RM_R_MAX: u8 = 0x3b;
pub const X86_OPCODE_TEST_RM_R_MIN: u8 = 0x84;
pub const X86_OPCODE_TEST_RM_R_MAX: u8 = 0x85;
pub const X86_OPCODE_XCHG_RM_R_MIN: u8 = 0x86;
pub const X86_OPCODE_XCHG_RM_R_MAX: u8 = 0x87;
pub const X86_OPCODE_MOV_BYTE_RM_R_MIN: u8 = 0x88;
pub const X86_OPCODE_MOV_BYTE_RM_R_MAX: u8 = 0x8b;

pub const X86_OPCODE_SYSCALL: u8 = 0x05;
pub const X86_OPCODE_SYSENTER: u8 = 0x34;
pub const X86_OPCODE_INT_N: u8 = 0xcd;
pub const X86_INT_VECTOR_COMPAT_LINUX: u8 = 0x80;
pub const X86_OPCODE_IN_AL_IMM8: u8 = 0xe4;
pub const X86_OPCODE_IN_EAX_IMM8: u8 = 0xe5;
pub const X86_OPCODE_IN_AL_DX: u8 = 0xec;
pub const X86_OPCODE_IN_EAX_DX: u8 = 0xed;
pub const X86_OPCODE_OUT_IMM8_AL: u8 = 0xe6;
pub const X86_OPCODE_OUT_IMM8_EAX: u8 = 0xe7;
pub const X86_OPCODE_OUT_DX_AL: u8 = 0xee;
pub const X86_OPCODE_OUT_DX_EAX: u8 = 0xef;
pub const X86_OPCODE_CLI: u8 = 0xfa;
pub const X86_OPCODE_STI: u8 = 0xfb;
pub const X86_OPCODE_HLT: u8 = 0xf4;

pub const AARCH64_TBZ_TBNZ_MASK: u32 = 0x7e000000;
pub const AARCH64_TBZ_MATCH: u32 = 0x36000000;
pub const AARCH64_TBNZ_MATCH: u32 = 0x37000000;
pub const AARCH64_TBZ_IMM14_MASK: u32 = 0x3fff;
pub const AARCH64_TBZ_IMM14_SHIFT: u32 = VSIX_MAX_SIZE_MB as u32;
pub const AARCH64_TBZ_IMM14_SIGN_BIT: u32 = 0x2000;
pub const AARCH64_TBZ_IMM14_SIGN_EXTEND: i32 =
    1 << (((DAYS_PER_WEEK as u32) * (BISECTION_DIVISOR as u32)) as usize);

pub const RISCV_C_ADDI16SP_STEP: usize = 1 << std::mem::size_of::<u32>();
pub const RISCV_JALR_ALT_INSN: u32 = 0x000080e7;
pub const RISCV_JAL_IMM10_1_MASK: u32 = 0x3ff;
pub const RISCV_JAL_IMM19_12_MASK: u32 = 0xff;
pub const RISCV_JAL_SIGN_BIT: u32 = 0x100000;
pub const RISCV_JAL_SIGN_EXTEND: i32 =
    1 << (((BISECTION_DIVISOR * BASE_DECIMAL) + BYTE_ONE) as u32);
pub const RISCV_BRANCH_IMM10_5_MASK: u32 = 0x3f;
pub const RISCV_BRANCH_IMM4_1_MASK: u32 = 0x0f;
pub const RISCV_BRANCH_SIGN_BIT: u32 = 0x1000;
pub const RISCV_BRANCH_SIGN_EXTEND: i32 = 1 << ((BASE_DECIMAL + STEP_TRIO) as u32);
pub const RISCV_C_J_SIGN_BIT: u32 = 0x800;
pub const RISCV_C_J_SIGN_EXTEND: i32 = 1 << ((BASE_DECIMAL + BISECTION_DIVISOR) as u32);
pub const RISCV_C_B_SIGN_BIT: u32 = 0x100;
pub const RISCV_C_B_SIGN_EXTEND: i32 = 1 << ((std::mem::size_of::<u64>() + BYTE_ONE) as u32);

pub const RISCV_JAL_IMM20_SRC_SHIFT: u32 = ((1 << VSIX_MAX_SIZE_MB) - BYTE_ONE) as u32;
pub const RISCV_JAL_IMM10_1_SRC_SHIFT: u32 =
    ((BISECTION_DIVISOR * BASE_DECIMAL) + BYTE_ONE) as u32;
pub const RISCV_JAL_IMM11_SRC_SHIFT: u32 = (BISECTION_DIVISOR * BASE_DECIMAL) as u32;
pub const RISCV_JAL_IMM19_12_SRC_SHIFT: u32 = (BASE_DECIMAL + BISECTION_DIVISOR) as u32;
pub const RISCV_JAL_IMM20_DST_SHIFT: u32 = (BISECTION_DIVISOR * BASE_DECIMAL) as u32;
pub const RISCV_JAL_IMM19_12_DST_SHIFT: u32 = (BASE_DECIMAL + BISECTION_DIVISOR) as u32;
pub const RISCV_JAL_IMM11_DST_SHIFT: u32 = (BASE_DECIMAL + BYTE_ONE) as u32;
pub const RISCV_JAL_IMM10_1_DST_SHIFT: u32 = BYTE_ONE as u32;

pub const RISCV_BRANCH_IMM12_SRC_SHIFT: u32 = ((1 << VSIX_MAX_SIZE_MB) - BYTE_ONE) as u32;
pub const RISCV_BRANCH_IMM10_5_SRC_SHIFT: u32 = (VSIX_MAX_SIZE_MB * VSIX_MAX_SIZE_MB) as u32;
pub const RISCV_BRANCH_IMM4_1_SRC_SHIFT: u32 = std::mem::size_of::<u64>() as u32;
pub const RISCV_BRANCH_IMM11_SRC_SHIFT: u32 = DAYS_PER_WEEK as u32;
pub const RISCV_BRANCH_IMM12_DST_SHIFT: u32 = (BASE_DECIMAL + BISECTION_DIVISOR) as u32;
pub const RISCV_BRANCH_IMM11_DST_SHIFT: u32 = (BASE_DECIMAL + BYTE_ONE) as u32;
pub const RISCV_BRANCH_IMM10_5_DST_SHIFT: u32 = VSIX_MAX_SIZE_MB as u32;
pub const RISCV_BRANCH_IMM4_1_DST_SHIFT: u32 = BYTE_ONE as u32;

pub const RISCV_C_J_MASK_2BIT: u16 = 0x03;
pub const RISCV_C_J_MASK_3BIT: u16 = 0x07;
pub const RISCV_C_J_SRC_IMM11_SHIFT: u16 = (BASE_DECIMAL + BISECTION_DIVISOR) as u16;
pub const RISCV_C_J_SRC_IMM4_SHIFT: u16 = (BASE_DECIMAL + BYTE_ONE) as u16;
pub const RISCV_C_J_SRC_IMM9_8_SHIFT: u16 = (std::mem::size_of::<u64>() + BYTE_ONE) as u16;
pub const RISCV_C_J_SRC_IMM10_SHIFT: u16 = std::mem::size_of::<u64>() as u16;
pub const RISCV_C_J_SRC_IMM6_SHIFT: u16 = DAYS_PER_WEEK as u16;
pub const RISCV_C_J_SRC_IMM7_SHIFT: u16 = ((DAYS_PER_WEEK as usize) - BYTE_ONE) as u16;
pub const RISCV_C_J_SRC_IMM3_1_SHIFT: u16 = STEP_TRIO as u16;
pub const RISCV_C_J_SRC_IMM5_SHIFT: u16 = BISECTION_DIVISOR as u16;
pub const RISCV_C_J_DST_IMM11_SHIFT: u32 = (BASE_DECIMAL + BYTE_ONE) as u32;
pub const RISCV_C_J_DST_IMM10_SHIFT: u32 = BASE_DECIMAL as u32;
pub const RISCV_C_J_DST_IMM9_8_SHIFT: u32 = std::mem::size_of::<u64>() as u32;
pub const RISCV_C_J_DST_IMM7_SHIFT: u32 = DAYS_PER_WEEK as u32;
pub const RISCV_C_J_DST_IMM6_SHIFT: u32 = ((DAYS_PER_WEEK as usize) - BYTE_ONE) as u32;
pub const RISCV_C_J_DST_IMM5_SHIFT: u32 = VSIX_MAX_SIZE_MB as u32;
pub const RISCV_C_J_DST_IMM4_SHIFT: u32 = std::mem::size_of::<u32>() as u32;
pub const RISCV_C_J_DST_IMM3_1_SHIFT: u32 = BYTE_ONE as u32;

pub const RISCV_C_B_MASK_2BIT: u16 = 0x03;
pub const RISCV_C_B_SRC_IMM8_SHIFT: u16 = (BASE_DECIMAL + BISECTION_DIVISOR) as u16;
pub const RISCV_C_B_SRC_IMM4_3_SHIFT: u16 = BASE_DECIMAL as u16;
pub const RISCV_C_B_SRC_IMM7_6_SHIFT: u16 = VSIX_MAX_SIZE_MB as u16;
pub const RISCV_C_B_SRC_IMM2_1_SHIFT: u16 = STEP_TRIO as u16;
pub const RISCV_C_B_SRC_IMM5_SHIFT: u16 = BISECTION_DIVISOR as u16;
pub const RISCV_C_B_DST_IMM8_SHIFT: u32 = std::mem::size_of::<u64>() as u32;
pub const RISCV_C_B_DST_IMM7_6_SHIFT: u32 = ((DAYS_PER_WEEK as usize) - BYTE_ONE) as u32;
pub const RISCV_C_B_DST_IMM5_SHIFT: u32 = VSIX_MAX_SIZE_MB as u32;
pub const RISCV_C_B_DST_IMM4_3_SHIFT: u32 = STEP_TRIO as u32;
pub const RISCV_C_B_DST_IMM2_1_SHIFT: u32 = BYTE_ONE as u32;

#[allow(dead_code)]
fn decode_modrm_and_displacements(bytecode: &[u8], pos: &mut usize) -> Option<(u8, u8, u8)> {
    if *pos >= bytecode.len() {
        return None;
    }
    let m = bytecode[*pos];
    *pos += 1;
    let mod_bits = m & X86_MODRM_MOD_MASK;
    let reg_bits = (m & X86_MODRM_REG_MASK) >> X86_MODRM_REG_SHIFT;
    let rm_bits = m & X86_MODRM_RM_MASK;

    if rm_bits == X86_MODRM_RM_SIB && mod_bits != X86_MODRM_MOD_DIRECT {
        if *pos < bytecode.len() {
            let sib = bytecode[*pos];
            *pos += 1;
            if (sib & X86_MODRM_RM_MASK) == X86_SIB_BASE_DISP32_ONLY && mod_bits == 0 {
                *pos += X86_DISP32_BYTES;
            }
        }
    } else if mod_bits == 0 && rm_bits == X86_MODRM_RM_DISP32_ONLY {
        *pos += X86_DISP32_BYTES;
    }

    if mod_bits == X86_MODRM_MOD_DISP8 {
        *pos += 1;
    } else if mod_bits == X86_MODRM_MOD_DISP32 {
        *pos += X86_DISP32_BYTES;
    }

    Some((m, mod_bits, reg_bits))
}

pub trait IsaVerifier: Send + Sync {
    fn isa(&self) -> TargetIsa;
    fn verify_stack_frame(&self, bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError>;
    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError>;
}

pub struct X86_64Verifier;

#[derive(Debug, Clone, Copy)]
pub struct X86Instruction {
    pub offset: usize,
    pub length: usize,
    pub opcode: u8,
    pub is_two_byte: bool,
    pub has_rex_w: bool,
    pub modrm: Option<u8>,
    pub imm: Option<i64>,
    pub is_return: bool,
    pub is_stack_sub: bool,
    pub is_stack_add: bool,
    pub is_leave: bool,
    pub jump_target: Option<usize>,
    pub is_banned: bool,
}

pub fn scan_banned_instructions(bytecode: &[u8]) -> Result<(), VerifyError> {
    X86_64Verifier::scan_banned_instructions(bytecode)
}

impl X86_64Verifier {
    pub fn scan_banned_instructions(bytecode: &[u8]) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }
        let mut idx = 0;
        while idx < bytecode.len() {
            let b = bytecode[idx];
            if idx + 1 < bytecode.len() {
                let next = bytecode[idx + 1];
                if (b == X86_OPCODE_TWO_BYTE_ESCAPE
                    && (next == X86_OPCODE_SYSCALL || next == X86_OPCODE_SYSENTER))
                    || (b == X86_OPCODE_INT_N && next == X86_INT_VECTOR_COMPAT_LINUX)
                {
                    return Err(VerifyError::CfiViolation(
                        "Banned system/privileged instruction detected".to_string(),
                    ));
                }
            }
            if b == X86_OPCODE_INT3
                || b == X86_OPCODE_IN_AL_IMM8
                || b == X86_OPCODE_IN_EAX_IMM8
                || b == X86_OPCODE_IN_AL_DX
                || b == X86_OPCODE_IN_EAX_DX
                || b == X86_OPCODE_OUT_IMM8_AL
                || b == X86_OPCODE_OUT_IMM8_EAX
                || b == X86_OPCODE_OUT_DX_AL
                || b == X86_OPCODE_OUT_DX_EAX
                || b == X86_OPCODE_CLI
                || b == X86_OPCODE_STI
                || b == X86_OPCODE_HLT
            {
                return Err(VerifyError::CfiViolation(
                    "Banned system/privileged instruction detected".to_string(),
                ));
            }
            idx += 1;
        }
        Ok(())
    }

    pub fn decode_instruction(bytecode: &[u8], start: usize) -> Option<X86Instruction> {
        if start >= bytecode.len() {
            return None;
        }

        let mut pos = start;
        let mut has_rex_w = false;
        let mut has_op_size = false;

        // 1. Consume legacy prefixes and REX prefixes
        while pos < bytecode.len() {
            let b = bytecode[pos];
            if b == X86_PREFIX_LOCK
                || b == X86_PREFIX_REPNE
                || b == X86_PREFIX_REP
                || b == X86_PREFIX_CS_OVERRIDE
                || b == X86_PREFIX_SS_OVERRIDE
                || b == X86_PREFIX_DS_OVERRIDE
                || b == X86_PREFIX_ES_OVERRIDE
                || b == X86_PREFIX_FS_OVERRIDE
                || b == X86_PREFIX_GS_OVERRIDE
                || b == X86_PREFIX_ADDRESS_SIZE
            {
                pos += 1;
                continue;
            }
            if b == X86_PREFIX_OPERAND_SIZE {
                has_op_size = true;
                pos += 1;
                continue;
            }
            if (X86_REX_MIN..=X86_REX_MAX).contains(&b) {
                if (b & X86_REX_W_FLAG) != 0 {
                    has_rex_w = true;
                }
                pos += 1;
                continue;
            }
            break;
        }

        if pos >= bytecode.len() {
            return Some(X86Instruction {
                offset: start,
                length: pos - start,
                opcode: 0,
                is_two_byte: false,
                has_rex_w,
                modrm: None,
                imm: None,
                is_return: false,
                is_stack_sub: false,
                is_stack_add: false,
                is_leave: false,
                jump_target: None,
                is_banned: false,
            });
        }

        // 2. Decode Opcode
        let is_two_byte = bytecode[pos] == X86_OPCODE_TWO_BYTE_ESCAPE;
        if is_two_byte {
            pos += 1;
            if pos >= bytecode.len() {
                return Some(X86Instruction {
                    offset: start,
                    length: pos - start,
                    opcode: X86_OPCODE_TWO_BYTE_ESCAPE,
                    is_two_byte: true,
                    has_rex_w,
                    modrm: None,
                    imm: None,
                    is_return: false,
                    is_stack_sub: false,
                    is_stack_add: false,
                    is_leave: false,
                    jump_target: None,
                    is_banned: false,
                });
            }
        }

        let op = bytecode[pos];
        pos += 1;

        let mut modrm_val = None;
        let mut imm_val = None;
        let mut is_ret = false;
        let mut is_sub = false;
        let mut is_add = false;
        let mut is_leave = false;
        let mut jump_target = None;
        let mut is_banned = false;

        if !is_two_byte {
            // One-byte opcodes
            match op {
                X86_RET_OP | X86_RET_FAR_OP => {
                    is_ret = true;
                }
                X86_RET_IMM16_OP | X86_RET_FAR_IMM16_OP => {
                    is_ret = true;
                    if pos + X86_IMM16_BYTES <= bytecode.len() {
                        let imm = u16::from_le_bytes(
                            bytecode[pos..pos + X86_IMM16_BYTES].try_into().unwrap(),
                        );
                        imm_val = Some(imm as i64);
                        pos += X86_IMM16_BYTES;
                    }
                }
                X86_LEAVE_OP => {
                    is_leave = true;
                }
                X86_NOP_OPCODE => {}
                X86_OPCODE_INT_N => {
                    if pos < bytecode.len() && bytecode[pos] == X86_INT_VECTOR_COMPAT_LINUX {
                        is_banned = true;
                        pos += 1;
                    }
                }
                X86_OPCODE_INT3
                | X86_OPCODE_IN_AL_IMM8
                | X86_OPCODE_IN_EAX_IMM8
                | X86_OPCODE_IN_AL_DX
                | X86_OPCODE_IN_EAX_DX
                | X86_OPCODE_OUT_IMM8_AL
                | X86_OPCODE_OUT_IMM8_EAX
                | X86_OPCODE_OUT_DX_AL
                | X86_OPCODE_OUT_DX_EAX
                | X86_OPCODE_CLI
                | X86_OPCODE_STI
                | X86_OPCODE_HLT => {
                    is_banned = true;
                    if (op == X86_OPCODE_IN_AL_IMM8
                        || op == X86_OPCODE_IN_EAX_IMM8
                        || op == X86_OPCODE_OUT_IMM8_AL
                        || op == X86_OPCODE_OUT_IMM8_EAX)
                        && pos < bytecode.len()
                    {
                        pos += 1;
                    }
                }
                b if (X86_OPCODE_PUSH_REG_MIN..=X86_OPCODE_PUSH_REG_MAX).contains(&b) => {
                    is_sub = true;
                    let slot_size = if has_op_size {
                        std::mem::size_of::<u16>() as i64
                    } else {
                        std::mem::size_of::<u64>() as i64
                    };
                    imm_val = Some(slot_size);
                }
                b if (X86_OPCODE_POP_REG_MIN..=X86_OPCODE_POP_REG_MAX).contains(&b) => {
                    is_add = true;
                    let slot_size = if has_op_size {
                        std::mem::size_of::<u16>() as i64
                    } else {
                        std::mem::size_of::<u64>() as i64
                    };
                    imm_val = Some(slot_size);
                }
                X86_ENTER_OP => {
                    if pos + X86_ENTER_OPERANDS_BYTES <= bytecode.len() {
                        let imm = u16::from_le_bytes(
                            bytecode[pos..pos + X86_IMM16_BYTES].try_into().unwrap(),
                        );
                        imm_val = Some(imm as i64);
                        pos += X86_ENTER_OPERANDS_BYTES;
                        is_sub = true;
                    }
                }
                X86_OPCODE_JMP_REL8 => {
                    if pos < bytecode.len() {
                        let rel = bytecode[pos] as i8 as isize;
                        pos += 1;
                        let target = (pos as isize).wrapping_add(rel);
                        if target >= 0 {
                            jump_target = Some(target as usize);
                        }
                    }
                }
                X86_OPCODE_JMP_REL32 | X86_OPCODE_CALL_REL32 => {
                    if pos + X86_REL32_BYTES <= bytecode.len() {
                        let rel = i32::from_le_bytes(
                            bytecode[pos..pos + X86_REL32_BYTES].try_into().unwrap(),
                        ) as isize;
                        pos += X86_REL32_BYTES;
                        let target = (pos as isize).wrapping_add(rel);
                        if target >= 0 {
                            jump_target = Some(target as usize);
                        }
                    }
                }
                b if (X86_OPCODE_JCC_REL8_BASE
                    ..X86_OPCODE_JCC_REL8_BASE + X86_OPCODE_JCC_REL8_COUNT)
                    .contains(&b) =>
                {
                    if pos < bytecode.len() {
                        let rel = bytecode[pos] as i8 as isize;
                        pos += 1;
                        let target = (pos as isize).wrapping_add(rel);
                        if target >= 0 {
                            jump_target = Some(target as usize);
                        }
                    }
                }
                X86_OPCODE_GRP1_IMM8 | X86_OPCODE_GRP1_IMM32 => {
                    if let Some((m, _mod_bits, reg_bits)) =
                        decode_modrm_and_displacements(bytecode, &mut pos)
                    {
                        modrm_val = Some(m);
                        if op == X86_OPCODE_GRP1_IMM8 {
                            if pos < bytecode.len() {
                                let imm = bytecode[pos] as usize;
                                imm_val = Some(imm as i64);
                                pos += 1;
                            }
                        } else if has_op_size {
                            if pos + X86_IMM16_BYTES <= bytecode.len() {
                                let imm = u16::from_le_bytes(
                                    bytecode[pos..pos + X86_IMM16_BYTES].try_into().unwrap(),
                                ) as usize;
                                imm_val = Some(imm as i64);
                                pos += X86_IMM16_BYTES;
                            }
                        } else if pos + X86_IMM32_BYTES <= bytecode.len() {
                            let imm = u32::from_le_bytes(
                                bytecode[pos..pos + X86_IMM32_BYTES].try_into().unwrap(),
                            ) as usize;
                            imm_val = Some(imm as i64);
                            pos += X86_IMM32_BYTES;
                        }

                        if has_rex_w && m == X86_RSP_MODRM && reg_bits == X86_SUB_REG_CODE {
                            is_sub = true;
                        } else if has_rex_w
                            && m == X86_RSP_ADD_MODRM
                            && reg_bits == X86_ADD_REG_CODE
                        {
                            is_add = true;
                        }
                    }
                }
                b if (X86_OPCODE_MOV_R_IMM_MIN..=X86_OPCODE_MOV_R_IMM_MAX).contains(&b) => {
                    if has_rex_w {
                        if pos + X86_IMM64_BYTES <= bytecode.len() {
                            let imm = i64::from_le_bytes(
                                bytecode[pos..pos + X86_IMM64_BYTES].try_into().unwrap(),
                            );
                            imm_val = Some(imm);
                            pos += X86_IMM64_BYTES;
                        }
                    } else if has_op_size {
                        if pos + X86_IMM16_BYTES <= bytecode.len() {
                            let imm = u16::from_le_bytes(
                                bytecode[pos..pos + X86_IMM16_BYTES].try_into().unwrap(),
                            ) as usize;
                            imm_val = Some(imm as i64);
                            pos += X86_IMM16_BYTES;
                        }
                    } else if pos + X86_IMM32_BYTES <= bytecode.len() {
                        let imm = u32::from_le_bytes(
                            bytecode[pos..pos + X86_IMM32_BYTES].try_into().unwrap(),
                        ) as usize;
                        imm_val = Some(imm as i64);
                        pos += X86_IMM32_BYTES;
                    }
                }
                X86_OPCODE_MOV_RM8_IMM8 => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                        if pos < bytecode.len() {
                            pos += 1;
                        }
                    }
                }
                X86_OPCODE_MOV_RM_IMM => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                        if has_op_size {
                            if pos + X86_IMM16_BYTES <= bytecode.len() {
                                pos += X86_IMM16_BYTES;
                            }
                        } else if pos + X86_IMM32_BYTES <= bytecode.len() {
                            pos += X86_IMM32_BYTES;
                        }
                    }
                }
                X86_OPCODE_IMUL_IMM32 => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                        if has_op_size {
                            if pos + X86_IMM16_BYTES <= bytecode.len() {
                                pos += X86_IMM16_BYTES;
                            }
                        } else if pos + X86_IMM32_BYTES <= bytecode.len() {
                            pos += X86_IMM32_BYTES;
                        }
                    }
                }
                X86_OPCODE_IMUL_IMM8 => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                        if pos < bytecode.len() {
                            pos += 1;
                        }
                    }
                }
                b if (X86_OPCODE_SHIFT_IMM8_MIN..=X86_OPCODE_SHIFT_IMM8_MAX).contains(&b) => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                        if pos < bytecode.len() {
                            pos += 1;
                        }
                    }
                }
                b if (X86_OPCODE_SHIFT_1_CL_MIN..=X86_OPCODE_SHIFT_1_CL_MAX).contains(&b) => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                    }
                }
                b if (X86_OPCODE_ADD_RM_R_MIN..=X86_OPCODE_ADD_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_OR_RM_R_MIN..=X86_OPCODE_OR_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_ADC_RM_R_MIN..=X86_OPCODE_ADC_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_SBB_RM_R_MIN..=X86_OPCODE_SBB_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_AND_RM_R_MIN..=X86_OPCODE_AND_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_SUB_RM_R_MIN..=X86_OPCODE_SUB_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_XOR_RM_R_MIN..=X86_OPCODE_XOR_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_CMP_RM_R_MIN..=X86_OPCODE_CMP_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_TEST_RM_R_MIN..=X86_OPCODE_TEST_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_XCHG_RM_R_MIN..=X86_OPCODE_XCHG_RM_R_MAX).contains(&b)
                    || (X86_OPCODE_MOV_BYTE_RM_R_MIN..=X86_OPCODE_MOV_BYTE_RM_R_MAX)
                        .contains(&b)
                    || b == X86_OPCODE_LEA
                    || b == X86_OPCODE_GRP3_NOT_NEG
                    || b == X86_OPCODE_GRP5_JMP_CALL =>
                {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                    }
                }
                _ => {}
            }
        } else {
            // Two-byte opcodes (0x0F ...)
            match op {
                X86_OPCODE_SYSCALL | X86_OPCODE_SYSENTER => {
                    is_banned = true;
                }
                b if (X86_OPCODE_JCC_REL32_BASE
                    ..X86_OPCODE_JCC_REL32_BASE + X86_OPCODE_JCC_REL32_COUNT)
                    .contains(&b) =>
                {
                    if pos + X86_REL32_BYTES <= bytecode.len() {
                        let rel = i32::from_le_bytes(
                            bytecode[pos..pos + X86_REL32_BYTES].try_into().unwrap(),
                        ) as isize;
                        pos += X86_REL32_BYTES;
                        let target = (pos as isize).wrapping_add(rel);
                        if target >= 0 {
                            jump_target = Some(target as usize);
                        }
                    }
                }
                _ => {
                    if let Some((m, _, _)) = decode_modrm_and_displacements(bytecode, &mut pos) {
                        modrm_val = Some(m);
                    }
                }
            }
        }

        let length = (pos - start).clamp(1, X86_MAX_INSN_LEN);
        Some(X86Instruction {
            offset: start,
            length,
            opcode: op,
            is_two_byte,
            has_rex_w,
            modrm: modrm_val,
            imm: imm_val,
            is_return: is_ret,
            is_stack_sub: is_sub,
            is_stack_add: is_add,
            is_leave,
            jump_target,
            is_banned,
        })
    }

    pub fn decode_all_instructions(bytecode: &[u8]) -> Vec<X86Instruction> {
        let mut insns = Vec::new();
        let mut offset = 0;
        while offset < bytecode.len() {
            if let Some(insn) = Self::decode_instruction(bytecode, offset) {
                let len = insn.length.max(1);
                insns.push(insn);
                offset += len;
            } else {
                break;
            }
        }
        insns
    }
}

impl IsaVerifier for X86_64Verifier {
    fn isa(&self) -> TargetIsa {
        TargetIsa::X86_64
    }

    fn verify_stack_frame(&self, bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }

        Self::scan_banned_instructions(bytecode)?;

        let instructions = Self::decode_all_instructions(bytecode);
        let mut current_allocated = 0;
        let bound = max_bytes.min(MAX_STACK_FRAME_BYTES);

        for insn in instructions {
            if insn.is_banned {
                return Err(VerifyError::CfiViolation(
                    "Banned system/privileged instruction detected".to_string(),
                ));
            }
            if insn.is_return || insn.is_leave {
                current_allocated = 0;
                continue;
            }

            if insn.is_stack_sub {
                if let Some(imm) = insn.imm {
                    current_allocated += imm as usize;
                    if current_allocated > bound {
                        return Err(VerifyError::StackOverflow(current_allocated, bound));
                    }
                }
            } else if insn.is_stack_add {
                if let Some(imm) = insn.imm {
                    current_allocated = current_allocated.saturating_sub(imm as usize);
                }
            }
        }

        Ok(())
    }

    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }

        Self::scan_banned_instructions(bytecode)?;

        let instructions = Self::decode_all_instructions(bytecode);
        let mut valid_boundaries = HashSet::new();
        let mut has_ret = false;

        for insn in &instructions {
            if insn.is_banned {
                return Err(VerifyError::CfiViolation(
                    "Banned system/privileged instruction detected".to_string(),
                ));
            }
            valid_boundaries.insert(insn.offset);
            if insn.is_return {
                has_ret = true;
            }
        }
        valid_boundaries.insert(bytecode.len());

        if !has_ret {
            return Err(VerifyError::CfiViolation(
                "no valid function return instruction detected".to_string(),
            ));
        }

        for insn in &instructions {
            if let Some(target) = insn.jump_target {
                if target <= bytecode.len() {
                    if !valid_boundaries.contains(&target) {
                        return Err(VerifyError::CfiViolation(format!(
                            "jump target 0x{:x} violates instruction boundary alignment",
                            target
                        )));
                    }
                } else if insn.opcode == X86_OPCODE_JMP_REL8
                    || (insn.opcode >= X86_OPCODE_JCC_REL8_BASE
                        && insn.opcode < X86_OPCODE_JCC_REL8_BASE + X86_OPCODE_JCC_REL8_COUNT)
                {
                    return Err(VerifyError::CfiViolation(format!(
                        "local jump target 0x{:x} out of bounds",
                        target
                    )));
                }
            }
        }

        Ok(())
    }
}

pub struct AArch64Verifier;

impl IsaVerifier for AArch64Verifier {
    fn isa(&self) -> TargetIsa {
        TargetIsa::AArch64
    }

    fn verify_stack_frame(&self, bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }

        scan_banned_instructions(bytecode)?;

        let mut current_allocated = 0;
        let mut j = 0;
        let bound = max_bytes.min(MAX_STACK_FRAME_BYTES);

        while j + AARCH64_INSN_BYTES <= bytecode.len() {
            let insn = u32::from_le_bytes(bytecode[j..j + AARCH64_INSN_BYTES].try_into().unwrap());

            if insn == AARCH64_RET_INSN
                || (insn & AARCH64_RET_MASK) == AARCH64_RET_BASE
                || insn == AARCH64_RETAA_INSN
                || insn == AARCH64_RETAB_INSN
            {
                current_allocated = 0;
            } else if (insn & AARCH64_SUB_SP_EXACT_MASK) == AARCH64_SUB_SP_EXACT_MATCH
                || (insn & AARCH64_SUB_SP_MASK) == AARCH64_SUB_SP_MATCH
            {
                let imm12 = (insn >> AARCH64_IMM12_SHIFT) & AARCH64_IMM12_MASK;
                let shift = if (insn & AARCH64_SUB_SP_SH_BIT) != 0 {
                    AARCH64_SUB_SP_SH_AMOUNT
                } else {
                    0
                };
                let allocated = (imm12 << shift) as usize;
                current_allocated += allocated;
                if current_allocated > bound {
                    return Err(VerifyError::StackOverflow(current_allocated, bound));
                }
            } else if (insn & AARCH64_STP_PRE_MASK) == AARCH64_STP_PRE_MATCH {
                let raw_imm7 = (insn >> AARCH64_STP_IMM7_SHIFT) & AARCH64_STP_IMM7_MASK;
                if (raw_imm7 & AARCH64_STP_IMM7_SIGN_BIT) != 0 {
                    let signed_imm = (raw_imm7 as i32) - AARCH64_STP_SIGN_EXTEND;
                    let allocated = ((-signed_imm) as usize) * AARCH64_STP_SCALE_BYTES;
                    current_allocated += allocated;
                    if current_allocated > bound {
                        return Err(VerifyError::StackOverflow(current_allocated, bound));
                    }
                }
            } else if (insn & AARCH64_ADD_SP_MASK) == AARCH64_ADD_SP_MATCH {
                let imm12 = (insn >> AARCH64_IMM12_SHIFT) & AARCH64_IMM12_MASK;
                let shift = if (insn & AARCH64_SUB_SP_SH_BIT) != 0 {
                    AARCH64_SUB_SP_SH_AMOUNT
                } else {
                    0
                };
                let deallocated = (imm12 << shift) as usize;
                current_allocated = current_allocated.saturating_sub(deallocated);
            } else if (insn & AARCH64_LDP_POST_MASK) == AARCH64_LDP_POST_MATCH {
                let raw_imm7 = (insn >> AARCH64_STP_IMM7_SHIFT) & AARCH64_STP_IMM7_MASK;
                if (raw_imm7 & AARCH64_STP_IMM7_SIGN_BIT) == 0 {
                    let deallocated = (raw_imm7 as usize) * AARCH64_STP_SCALE_BYTES;
                    current_allocated = current_allocated.saturating_sub(deallocated);
                }
            }

            j += AARCH64_INSN_BYTES;
        }

        Ok(())
    }

    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }

        scan_banned_instructions(bytecode)?;

        if !bytecode.len().is_multiple_of(AARCH64_INSN_BYTES) {
            return Err(VerifyError::CfiViolation(
                "AArch64 bytecode length must be aligned to 4 bytes".to_string(),
            ));
        }

        let mut valid_boundaries = HashSet::new();
        let mut branch_targets = Vec::new();
        let mut has_ret = false;
        let mut j = 0;

        while j + AARCH64_INSN_BYTES <= bytecode.len() {
            valid_boundaries.insert(j);
            let insn = u32::from_le_bytes(bytecode[j..j + AARCH64_INSN_BYTES].try_into().unwrap());

            if insn == AARCH64_RET_INSN
                || (insn & AARCH64_RET_MASK) == AARCH64_RET_BASE
                || insn == AARCH64_RETAA_INSN
                || insn == AARCH64_RETAB_INSN
            {
                has_ret = true;
            }

            // Check B / BL jump target alignment
            if (insn & AARCH64_B_BL_MASK) == AARCH64_B_MATCH
                || (insn & AARCH64_B_BL_MASK) == AARCH64_BL_MATCH
            {
                let raw_imm26 = insn & AARCH64_IMM26_MASK;
                let signed_imm26 = if (raw_imm26 & AARCH64_IMM26_SIGN_BIT) != 0 {
                    (raw_imm26 as i32) - AARCH64_IMM26_SIGN_EXTEND
                } else {
                    raw_imm26 as i32
                };
                let target = (j as isize)
                    .wrapping_add((signed_imm26 as isize) * (AARCH64_BRANCH_SCALE_BYTES as isize));
                branch_targets.push((target, false));
            } else if (insn & AARCH64_B_COND_MASK) == AARCH64_B_COND_MATCH
                || (insn & AARCH64_CBZ_CBNZ_MASK) == AARCH64_CBZ_MATCH
            {
                let raw_imm19 = (insn >> AARCH64_IMM19_SHIFT) & AARCH64_IMM19_MASK;
                let signed_imm19 = if (raw_imm19 & AARCH64_IMM19_SIGN_BIT) != 0 {
                    (raw_imm19 as i32) - AARCH64_IMM19_SIGN_EXTEND
                } else {
                    raw_imm19 as i32
                };
                let target = (j as isize)
                    .wrapping_add((signed_imm19 as isize) * (AARCH64_BRANCH_SCALE_BYTES as isize));
                branch_targets.push((target, true));
            } else if (insn & AARCH64_TBZ_TBNZ_MASK) == AARCH64_TBZ_MATCH {
                let raw_imm14 = (insn >> AARCH64_TBZ_IMM14_SHIFT) & AARCH64_TBZ_IMM14_MASK;
                let signed_imm14 = if (raw_imm14 & AARCH64_TBZ_IMM14_SIGN_BIT) != 0 {
                    (raw_imm14 as i32) - AARCH64_TBZ_IMM14_SIGN_EXTEND
                } else {
                    raw_imm14 as i32
                };
                let target = (j as isize)
                    .wrapping_add((signed_imm14 as isize) * (AARCH64_BRANCH_SCALE_BYTES as isize));
                branch_targets.push((target, true));
            }

            j += AARCH64_INSN_BYTES;
        }
        valid_boundaries.insert(bytecode.len());

        if !has_ret {
            return Err(VerifyError::CfiViolation(
                "no valid function return instruction detected".to_string(),
            ));
        }

        for (target, is_local_cond) in branch_targets {
            if target >= 0 && (target as usize) <= bytecode.len() {
                if !valid_boundaries.contains(&(target as usize)) {
                    return Err(VerifyError::CfiViolation(format!(
                        "AArch64 branch target 0x{:x} is not on a valid instruction boundary",
                        target
                    )));
                }
            } else if is_local_cond || target < 0 {
                return Err(VerifyError::CfiViolation(format!(
                    "AArch64 branch target 0x{:x} out of bounds",
                    target
                )));
            }
        }

        Ok(())
    }
}

pub struct RiscV64Verifier;

impl IsaVerifier for RiscV64Verifier {
    fn isa(&self) -> TargetIsa {
        TargetIsa::RiscV64
    }

    fn verify_stack_frame(&self, bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }

        scan_banned_instructions(bytecode)?;

        let mut current_allocated = 0;
        let mut idx = 0;
        let bound = max_bytes.min(MAX_STACK_FRAME_BYTES);

        while idx + RISCV_COMPRESSED_INSN_BYTES <= bytecode.len() {
            let is_32bit = (bytecode[idx] & RISCV_INSN_LEN_MASK) == RISCV_INSN_32BIT_MARKER;

            if is_32bit {
                if idx + RISCV_INSN_BYTES <= bytecode.len() {
                    let insn = u32::from_le_bytes(
                        bytecode[idx..idx + RISCV_INSN_BYTES].try_into().unwrap(),
                    );

                    if insn == RISCV_RET_INSN {
                        current_allocated = 0;
                    } else if (insn & RISCV_ADDI_SP_MASK) == RISCV_ADDI_SP_MATCH {
                        let raw_imm12 = (insn >> RISCV_ADDI_IMM12_SHIFT) & RISCV_ADDI_IMM12_MASK;
                        if (raw_imm12 & RISCV_ADDI_IMM12_SIGN_BIT) != 0 {
                            let signed_imm = (raw_imm12 as i32) - RISCV_ADDI_SIGN_EXTEND;
                            let allocated = (-signed_imm) as usize;
                            current_allocated += allocated;
                            if current_allocated > bound {
                                return Err(VerifyError::StackOverflow(current_allocated, bound));
                            }
                        } else {
                            let deallocated = raw_imm12 as usize;
                            current_allocated = current_allocated.saturating_sub(deallocated);
                        }
                    }

                    idx += RISCV_INSN_BYTES;
                } else {
                    break;
                }
            } else {
                let c_insn = u16::from_le_bytes(
                    bytecode[idx..idx + RISCV_COMPRESSED_INSN_BYTES]
                        .try_into()
                        .unwrap(),
                );
                if c_insn == RISCV_C_JR_RA_INSN || c_insn == RISCV_C_JALR_RA_INSN {
                    current_allocated = 0;
                } else if (c_insn & RISCV_C_ADDI16SP_MASK) == RISCV_C_ADDI16SP_MATCH {
                    if (c_insn & RISCV_C_ADDI16SP_SIGN_BIT) != 0 {
                        let allocated = RISCV_C_ADDI16SP_STEP;
                        current_allocated += allocated;
                        if current_allocated > bound {
                            return Err(VerifyError::StackOverflow(current_allocated, bound));
                        }
                    } else {
                        let deallocated = RISCV_C_ADDI16SP_STEP;
                        current_allocated = current_allocated.saturating_sub(deallocated);
                    }
                }

                idx += RISCV_COMPRESSED_INSN_BYTES;
            }
        }

        Ok(())
    }

    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Err(VerifyError::CfiViolation(
                "Empty bytecode cannot be verified".to_string(),
            ));
        }

        scan_banned_instructions(bytecode)?;

        if !bytecode.len().is_multiple_of(RISCV_COMPRESSED_INSN_BYTES) {
            return Err(VerifyError::CfiViolation(
                "RISC-V bytecode length must be aligned to 2 bytes".to_string(),
            ));
        }

        let mut valid_boundaries = HashSet::new();
        let mut branch_targets = Vec::new();
        let mut has_ret = false;
        let mut idx = 0;

        while idx + RISCV_COMPRESSED_INSN_BYTES <= bytecode.len() {
            valid_boundaries.insert(idx);
            let is_32bit = (bytecode[idx] & RISCV_INSN_LEN_MASK) == RISCV_INSN_32BIT_MARKER;

            if is_32bit {
                if idx + RISCV_INSN_BYTES <= bytecode.len() {
                    let insn = u32::from_le_bytes(
                        bytecode[idx..idx + RISCV_INSN_BYTES].try_into().unwrap(),
                    );
                    if insn == RISCV_RET_INSN || insn == RISCV_JALR_ALT_INSN {
                        has_ret = true;
                    }

                    // JAL target decoding
                    if (insn & RISCV_JAL_OPCODE_MASK) == RISCV_JAL_OPCODE_MATCH {
                        let imm20 = (insn >> RISCV_JAL_IMM20_SRC_SHIFT) & 1;
                        let imm10_1 =
                            (insn >> RISCV_JAL_IMM10_1_SRC_SHIFT) & RISCV_JAL_IMM10_1_MASK;
                        let imm11 = (insn >> RISCV_JAL_IMM11_SRC_SHIFT) & 1;
                        let imm19_12 =
                            (insn >> RISCV_JAL_IMM19_12_SRC_SHIFT) & RISCV_JAL_IMM19_12_MASK;
                        let raw_imm = (imm20 << RISCV_JAL_IMM20_DST_SHIFT)
                            | (imm19_12 << RISCV_JAL_IMM19_12_DST_SHIFT)
                            | (imm11 << RISCV_JAL_IMM11_DST_SHIFT)
                            | (imm10_1 << RISCV_JAL_IMM10_1_DST_SHIFT);
                        let signed_imm = if (raw_imm & RISCV_JAL_SIGN_BIT) != 0 {
                            (raw_imm as i32) - RISCV_JAL_SIGN_EXTEND
                        } else {
                            raw_imm as i32
                        };
                        let target = (idx as isize).wrapping_add(signed_imm as isize);
                        branch_targets.push((target, false));
                    } else if (insn & RISCV_JAL_OPCODE_MASK) == RISCV_BRANCH_OPCODE_MATCH {
                        // B-type branches (BEQ, BNE, BLT, BGE, BLTU, BGEU)
                        let imm12 = (insn >> RISCV_BRANCH_IMM12_SRC_SHIFT) & 1;
                        let imm10_5 =
                            (insn >> RISCV_BRANCH_IMM10_5_SRC_SHIFT) & RISCV_BRANCH_IMM10_5_MASK;
                        let imm4_1 =
                            (insn >> RISCV_BRANCH_IMM4_1_SRC_SHIFT) & RISCV_BRANCH_IMM4_1_MASK;
                        let imm11 = (insn >> RISCV_BRANCH_IMM11_SRC_SHIFT) & 1;
                        let raw_imm = (imm12 << RISCV_BRANCH_IMM12_DST_SHIFT)
                            | (imm11 << RISCV_BRANCH_IMM11_DST_SHIFT)
                            | (imm10_5 << RISCV_BRANCH_IMM10_5_DST_SHIFT)
                            | (imm4_1 << RISCV_BRANCH_IMM4_1_DST_SHIFT);
                        let signed_imm = if (raw_imm & RISCV_BRANCH_SIGN_BIT) != 0 {
                            (raw_imm as i32) - RISCV_BRANCH_SIGN_EXTEND
                        } else {
                            raw_imm as i32
                        };
                        let target = (idx as isize).wrapping_add(signed_imm as isize);
                        branch_targets.push((target, true));
                    }

                    idx += RISCV_INSN_BYTES;
                } else {
                    return Err(VerifyError::CfiViolation(
                        "truncated 32-bit RISC-V instruction at buffer end".to_string(),
                    ));
                }
            } else {
                let c_insn = u16::from_le_bytes(
                    bytecode[idx..idx + RISCV_COMPRESSED_INSN_BYTES]
                        .try_into()
                        .unwrap(),
                );
                if c_insn == RISCV_C_JR_RA_INSN || c_insn == RISCV_C_JALR_RA_INSN {
                    has_ret = true;
                }

                // C.J target decoding
                if (c_insn & RISCV_C_J_MASK) == RISCV_C_J_MATCH {
                    let imm11 = ((c_insn >> RISCV_C_J_SRC_IMM11_SHIFT) & 1) as u32;
                    let imm4 = ((c_insn >> RISCV_C_J_SRC_IMM4_SHIFT) & 1) as u32;
                    let imm9_8 =
                        ((c_insn >> RISCV_C_J_SRC_IMM9_8_SHIFT) & RISCV_C_J_MASK_2BIT) as u32;
                    let imm10 = ((c_insn >> RISCV_C_J_SRC_IMM10_SHIFT) & 1) as u32;
                    let imm6 = ((c_insn >> RISCV_C_J_SRC_IMM6_SHIFT) & 1) as u32;
                    let imm7 = ((c_insn >> RISCV_C_J_SRC_IMM7_SHIFT) & 1) as u32;
                    let imm3_1 =
                        ((c_insn >> RISCV_C_J_SRC_IMM3_1_SHIFT) & RISCV_C_J_MASK_3BIT) as u32;
                    let imm5 = ((c_insn >> RISCV_C_J_SRC_IMM5_SHIFT) & 1) as u32;
                    let raw_imm = (imm11 << RISCV_C_J_DST_IMM11_SHIFT)
                        | (imm10 << RISCV_C_J_DST_IMM10_SHIFT)
                        | (imm9_8 << RISCV_C_J_DST_IMM9_8_SHIFT)
                        | (imm7 << RISCV_C_J_DST_IMM7_SHIFT)
                        | (imm6 << RISCV_C_J_DST_IMM6_SHIFT)
                        | (imm5 << RISCV_C_J_DST_IMM5_SHIFT)
                        | (imm4 << RISCV_C_J_DST_IMM4_SHIFT)
                        | (imm3_1 << RISCV_C_J_DST_IMM3_1_SHIFT);
                    let signed_imm = if (raw_imm & RISCV_C_J_SIGN_BIT) != 0 {
                        (raw_imm as i32) - RISCV_C_J_SIGN_EXTEND
                    } else {
                        raw_imm as i32
                    };
                    let target = (idx as isize).wrapping_add(signed_imm as isize);
                    branch_targets.push((target, false));
                } else if (c_insn & RISCV_C_BEQZ_MASK) == RISCV_C_BEQZ_MATCH
                    || (c_insn & RISCV_C_BEQZ_MASK) == RISCV_C_BNEZ_MATCH
                {
                    let imm8 = ((c_insn >> RISCV_C_B_SRC_IMM8_SHIFT) & 1) as u32;
                    let imm4_3 =
                        ((c_insn >> RISCV_C_B_SRC_IMM4_3_SHIFT) & RISCV_C_B_MASK_2BIT) as u32;
                    let imm7_6 =
                        ((c_insn >> RISCV_C_B_SRC_IMM7_6_SHIFT) & RISCV_C_B_MASK_2BIT) as u32;
                    let imm2_1 =
                        ((c_insn >> RISCV_C_B_SRC_IMM2_1_SHIFT) & RISCV_C_B_MASK_2BIT) as u32;
                    let imm5 = ((c_insn >> RISCV_C_B_SRC_IMM5_SHIFT) & 1) as u32;
                    let raw_imm = (imm8 << RISCV_C_B_DST_IMM8_SHIFT)
                        | (imm7_6 << RISCV_C_B_DST_IMM7_6_SHIFT)
                        | (imm5 << RISCV_C_B_DST_IMM5_SHIFT)
                        | (imm4_3 << RISCV_C_B_DST_IMM4_3_SHIFT)
                        | (imm2_1 << RISCV_C_B_DST_IMM2_1_SHIFT);
                    let signed_imm = if (raw_imm & RISCV_C_B_SIGN_BIT) != 0 {
                        (raw_imm as i32) - RISCV_C_B_SIGN_EXTEND
                    } else {
                        raw_imm as i32
                    };
                    let target = (idx as isize).wrapping_add(signed_imm as isize);
                    branch_targets.push((target, true));
                }

                idx += RISCV_COMPRESSED_INSN_BYTES;
            }
        }
        valid_boundaries.insert(bytecode.len());

        if !has_ret {
            return Err(VerifyError::CfiViolation(
                "no valid function return instruction detected".to_string(),
            ));
        }

        for (target, is_local_cond) in branch_targets {
            if target >= 0 && (target as usize) <= bytecode.len() {
                if !valid_boundaries.contains(&(target as usize)) {
                    return Err(VerifyError::CfiViolation(format!(
                        "RISC-V branch target 0x{:x} is not on a valid instruction boundary",
                        target
                    )));
                }
            } else if is_local_cond || target < 0 {
                return Err(VerifyError::CfiViolation(format!(
                    "RISC-V branch target 0x{:x} out of bounds",
                    target
                )));
            }
        }

        Ok(())
    }
}

static X86_64_VERIFIER: X86_64Verifier = X86_64Verifier;
static AARCH64_VERIFIER: AArch64Verifier = AArch64Verifier;
static RISCV64_VERIFIER: RiscV64Verifier = RiscV64Verifier;

pub fn get_verifier(isa: TargetIsa) -> &'static dyn IsaVerifier {
    match isa {
        TargetIsa::X86_64 => &X86_64_VERIFIER,
        TargetIsa::AArch64 => &AARCH64_VERIFIER,
        TargetIsa::RiscV64 => &RISCV64_VERIFIER,
    }
}

pub fn verify_cfi_for_isa(bytecode: &[u8], isa: TargetIsa) -> Result<(), VerifyError> {
    if bytecode.is_empty() {
        return Err(VerifyError::CfiViolation(
            "Empty bytecode cannot be verified".to_string(),
        ));
    }
    scan_banned_instructions(bytecode)?;
    get_verifier(isa).verify_cfi(bytecode)
}

pub fn verify_stack_frame_bound_for_isa(
    bytecode: &[u8],
    max_bytes: usize,
    isa: TargetIsa,
) -> Result<(), VerifyError> {
    if bytecode.is_empty() {
        return Err(VerifyError::CfiViolation(
            "Empty bytecode cannot be verified".to_string(),
        ));
    }
    scan_banned_instructions(bytecode)?;
    get_verifier(isa).verify_stack_frame(bytecode, max_bytes)
}

pub fn verify_stack_frame_4kb_bound(bytecode: &[u8]) -> Result<(), VerifyError> {
    verify_stack_frame_bound(bytecode, STACK_FRAME_4KB_BOUND)
}

pub fn verify_stack_frame_4kb_bound_for_isa(
    bytecode: &[u8],
    isa: TargetIsa,
) -> Result<(), VerifyError> {
    verify_stack_frame_bound_for_isa(bytecode, STACK_FRAME_4KB_BOUND, isa)
}

pub fn detect_isa(bytecode: &[u8]) -> Option<TargetIsa> {
    if bytecode.is_empty() {
        return None;
    }

    if !bytecode.len().is_multiple_of(RISCV_COMPRESSED_INSN_BYTES) {
        return Some(TargetIsa::X86_64);
    }

    let mut score_x86 = 0;
    let mut score_aarch64 = 0;
    let mut score_riscv = 0;

    if bytecode.len().is_multiple_of(AARCH64_INSN_BYTES) {
        let mut j = 0;
        while j + AARCH64_INSN_BYTES <= bytecode.len() {
            let insn = u32::from_le_bytes(bytecode[j..j + AARCH64_INSN_BYTES].try_into().unwrap());

            if insn == AARCH64_RET_INSN
                || insn == AARCH64_RETAA_INSN
                || insn == AARCH64_RETAB_INSN
                || (insn & AARCH64_RET_MASK) == AARCH64_RET_BASE
                || (insn & AARCH64_SUB_SP_EXACT_MASK) == AARCH64_SUB_SP_EXACT_MATCH
                || (insn & AARCH64_SUB_SP_MASK) == AARCH64_SUB_SP_MATCH
                || (insn & AARCH64_STP_PRE_MASK) == AARCH64_STP_PRE_MATCH
                || insn == AARCH64_BTI_MATCH
            {
                score_aarch64 += ISA_SCORE_STRONG_MARKER;
            }

            if insn == RISCV_RET_INSN || (insn & RISCV_ADDI_SP_MASK) == RISCV_ADDI_SP_MATCH {
                score_riscv += ISA_SCORE_STRONG_MARKER;
            }

            j += AARCH64_INSN_BYTES;
        }
    }

    let mut k = 0;
    while k + RISCV_COMPRESSED_INSN_BYTES <= bytecode.len() {
        let c_insn = u16::from_le_bytes(
            bytecode[k..k + RISCV_COMPRESSED_INSN_BYTES]
                .try_into()
                .unwrap(),
        );
        if c_insn == RISCV_C_JR_RA_INSN || c_insn == RISCV_C_JALR_RA_INSN {
            score_riscv += ISA_SCORE_STRONG_MARKER;
        }
        k += RISCV_COMPRESSED_INSN_BYTES;
    }

    let mut i = 0;
    while i < bytecode.len() {
        if bytecode[i] == X86_RET_OP || bytecode[i] == X86_RET_FAR_OP {
            score_x86 += ISA_SCORE_WEAK_MARKER;
        }
        if i + X86_PROLOGUE_SEQ_BYTES <= bytecode.len() {
            let slice = &bytecode[i..i + X86_PROLOGUE_SEQ_BYTES];
            if slice[0] == X86_REX_W
                && (slice[1] == X86_SUB_IMM8_OP || slice[1] == X86_SUB_IMM32_OP)
                && slice[X86_PROLOGUE_RSP_OFFSET] == X86_RSP_MODRM
            {
                score_x86 += ISA_SCORE_STRONG_MARKER;
            }
            if slice[0] == X86_PUSH_RBP
                && slice[1] == X86_MOV_RBP_RSP_B0
                && slice[X86_PROLOGUE_RSP_OFFSET] == X86_MOV_RBP_RSP_B1
            {
                score_x86 += ISA_SCORE_STRONG_MARKER;
            }
        }
        if i + X86_ENDBR64_BYTES <= bytecode.len()
            && bytecode[i..i + X86_ENDBR64_BYTES]
                == [
                    X86_ENDBR64_B0,
                    X86_ENDBR64_B1,
                    X86_ENDBR64_B2,
                    X86_ENDBR64_B3,
                ]
        {
            score_x86 += ISA_SCORE_STRONG_MARKER;
        }
        i += 1;
    }

    if score_aarch64 > score_x86 && score_aarch64 >= score_riscv {
        return Some(TargetIsa::AArch64);
    }
    if score_riscv > score_x86 && score_riscv > score_aarch64 {
        return Some(TargetIsa::RiscV64);
    }
    if score_x86 > 0 {
        return Some(TargetIsa::X86_64);
    }

    None
}

pub fn verify_stack_frame_bound(bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError> {
    if bytecode.is_empty() {
        return Err(VerifyError::CfiViolation(
            "Empty bytecode cannot be verified".to_string(),
        ));
    }
    scan_banned_instructions(bytecode)?;
    let isa = detect_isa(bytecode).unwrap_or_else(TargetIsa::native);
    get_verifier(isa).verify_stack_frame(bytecode, max_bytes)
}

pub fn verify_cfi_integrity(bytecode: &[u8]) -> Result<(), VerifyError> {
    if bytecode.is_empty() {
        return Err(VerifyError::CfiViolation(
            "Empty bytecode cannot be verified".to_string(),
        ));
    }
    scan_banned_instructions(bytecode)?;
    let isa = detect_isa(bytecode).unwrap_or_else(TargetIsa::native);
    get_verifier(isa).verify_cfi(bytecode)
}
