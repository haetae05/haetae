use crate::tcc_engine::VerifyError;
pub use haetae_core::config::{
    TargetIsa, AARCH64_ADD_SP_MASK, AARCH64_ADD_SP_MATCH, AARCH64_BTI_MATCH, AARCH64_IMM12_MASK,
    AARCH64_IMM12_SHIFT, AARCH64_INSN_BYTES, AARCH64_LDP_POST_MASK, AARCH64_LDP_POST_MATCH,
    AARCH64_RETAA_INSN, AARCH64_RETAB_INSN, AARCH64_RET_BASE, AARCH64_RET_INSN, AARCH64_RET_MASK,
    AARCH64_STP_IMM7_MASK, AARCH64_STP_IMM7_SHIFT, AARCH64_STP_IMM7_SIGN_BIT, AARCH64_STP_PRE_MASK,
    AARCH64_STP_PRE_MATCH, AARCH64_STP_SCALE_BYTES, AARCH64_STP_SIGN_EXTEND,
    AARCH64_SUB_SP_EXACT_MASK, AARCH64_SUB_SP_EXACT_MATCH, AARCH64_SUB_SP_MASK,
    AARCH64_SUB_SP_MATCH, AARCH64_SUB_SP_SH_AMOUNT, AARCH64_SUB_SP_SH_BIT, ISA_SCORE_STRONG_MARKER,
    ISA_SCORE_WEAK_MARKER, NUM_EIGHT, NUM_FIVE, NUM_FOUR, NUM_ONE, NUM_ONE_TWENTY_EIGHT, NUM_SIX,
    NUM_SIXTEEN, NUM_TEN, NUM_THREE, NUM_TWELVE, NUM_TWENTY, NUM_TWO, NUM_ZERO,
    RISCV_ADDI_IMM12_MASK, RISCV_ADDI_IMM12_SHIFT, RISCV_ADDI_IMM12_SIGN_BIT,
    RISCV_ADDI_SIGN_EXTEND, RISCV_ADDI_SP_MASK, RISCV_ADDI_SP_MATCH, RISCV_COMPRESSED_INSN_BYTES,
    RISCV_C_ADDI16SP_MASK, RISCV_C_ADDI16SP_MATCH, RISCV_C_ADDI16SP_SIGN_BIT, RISCV_C_JALR_RA_INSN,
    RISCV_C_JR_RA_INSN, RISCV_INSN_32BIT_MARKER, RISCV_INSN_BYTES, RISCV_INSN_LEN_MASK,
    RISCV_RET_INSN, X86_ADD_IMM32_OP, X86_ADD_IMM8_OP, X86_ENDBR64_B0, X86_ENDBR64_B1,
    X86_ENDBR64_B2, X86_ENDBR64_B3, X86_ENTER_OP, X86_LEAVE_OP, X86_MOV_RBP_RSP_B0,
    X86_MOV_RBP_RSP_B1, X86_MOV_RBP_RSP_B2, X86_PUSH_RBP, X86_RET_FAR_IMM16_OP, X86_RET_FAR_OP,
    X86_RET_IMM16_OP, X86_RET_OP, X86_REX_W, X86_RSP_ADD_MODRM, X86_RSP_MODRM, X86_SUB_IMM32_OP,
    X86_SUB_IMM8_OP,
};

pub trait IsaVerifier: Send + Sync {
    fn isa(&self) -> TargetIsa;
    fn verify_stack_frame(&self, bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError>;
    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError>;
}

pub struct X86_64Verifier;

impl IsaVerifier for X86_64Verifier {
    fn isa(&self) -> TargetIsa {
        TargetIsa::X86_64
    }

    fn verify_stack_frame(&self, bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Ok(());
        }

        let mut current_allocated = NUM_ZERO;
        let mut i = NUM_ZERO;
        while i < bytecode.len() {
            let op = bytecode[i];
            if op == X86_RET_OP
                || op == X86_RET_FAR_OP
                || ((op == X86_RET_IMM16_OP || op == X86_RET_FAR_IMM16_OP)
                    && i + NUM_TWO < bytecode.len())
            {
                current_allocated = NUM_ZERO;
                i += NUM_ONE;
                continue;
            }

            if op == X86_LEAVE_OP {
                current_allocated = NUM_ZERO;
                i += NUM_ONE;
                continue;
            }

            if i + NUM_TWO < bytecode.len() && bytecode[i] == X86_REX_W {
                if bytecode[i + NUM_ONE] == X86_SUB_IMM8_OP
                    && bytecode[i + NUM_TWO] == X86_RSP_MODRM
                    && i + NUM_THREE < bytecode.len()
                {
                    let imm = bytecode[i + NUM_THREE] as usize;
                    current_allocated += imm;
                    if current_allocated > max_bytes {
                        return Err(VerifyError::StackOverflow(current_allocated, max_bytes));
                    }
                    i += NUM_FOUR;
                    continue;
                } else if bytecode[i + NUM_ONE] == X86_SUB_IMM32_OP
                    && bytecode[i + NUM_TWO] == X86_RSP_MODRM
                    && i + NUM_SIX < bytecode.len()
                {
                    let imm = u32::from_le_bytes([
                        bytecode[i + NUM_THREE],
                        bytecode[i + NUM_FOUR],
                        bytecode[i + NUM_FIVE],
                        bytecode[i + NUM_SIX],
                    ]) as usize;
                    current_allocated += imm;
                    if current_allocated > max_bytes {
                        return Err(VerifyError::StackOverflow(current_allocated, max_bytes));
                    }
                    i += NUM_SIX + NUM_ONE;
                    continue;
                } else if bytecode[i + NUM_ONE] == X86_ADD_IMM8_OP
                    && bytecode[i + NUM_TWO] == X86_RSP_ADD_MODRM
                    && i + NUM_THREE < bytecode.len()
                {
                    let imm = bytecode[i + NUM_THREE] as usize;
                    current_allocated = current_allocated.saturating_sub(imm);
                    i += NUM_FOUR;
                    continue;
                } else if bytecode[i + NUM_ONE] == X86_ADD_IMM32_OP
                    && bytecode[i + NUM_TWO] == X86_RSP_ADD_MODRM
                    && i + NUM_SIX < bytecode.len()
                {
                    let imm = u32::from_le_bytes([
                        bytecode[i + NUM_THREE],
                        bytecode[i + NUM_FOUR],
                        bytecode[i + NUM_FIVE],
                        bytecode[i + NUM_SIX],
                    ]) as usize;
                    current_allocated = current_allocated.saturating_sub(imm);
                    i += NUM_SIX + NUM_ONE;
                    continue;
                }
            } else if bytecode[i] == X86_ENTER_OP && i + NUM_TWO < bytecode.len() {
                let imm =
                    u16::from_le_bytes([bytecode[i + NUM_ONE], bytecode[i + NUM_TWO]]) as usize;
                current_allocated += imm;
                if current_allocated > max_bytes {
                    return Err(VerifyError::StackOverflow(current_allocated, max_bytes));
                }
                i += NUM_THREE;
                continue;
            }

            i += NUM_ONE;
        }

        Ok(())
    }

    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Ok(());
        }

        let mut has_ret = false;
        let mut i = NUM_ZERO;
        while i < bytecode.len() {
            let op = bytecode[i];
            if op == X86_RET_OP
                || op == X86_RET_FAR_OP
                || ((op == X86_RET_IMM16_OP || op == X86_RET_FAR_IMM16_OP)
                    && i + NUM_TWO < bytecode.len())
            {
                has_ret = true;
                break;
            }
            i += NUM_ONE;
        }

        if !has_ret {
            return Err(VerifyError::CfiViolation(
                "no valid function return instruction detected".to_string(),
            ));
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
            return Ok(());
        }

        let mut current_allocated = NUM_ZERO;
        let mut j = NUM_ZERO;
        while j + AARCH64_INSN_BYTES <= bytecode.len() {
            let insn = u32::from_le_bytes([
                bytecode[j],
                bytecode[j + NUM_ONE],
                bytecode[j + NUM_TWO],
                bytecode[j + NUM_THREE],
            ]);

            if insn == AARCH64_RET_INSN
                || (insn & AARCH64_RET_MASK) == AARCH64_RET_BASE
                || insn == AARCH64_RETAA_INSN
                || insn == AARCH64_RETAB_INSN
            {
                current_allocated = NUM_ZERO;
            } else if (insn & AARCH64_SUB_SP_EXACT_MASK) == AARCH64_SUB_SP_EXACT_MATCH
                || (insn & AARCH64_SUB_SP_MASK) == AARCH64_SUB_SP_MATCH
            {
                let imm12 = (insn >> AARCH64_IMM12_SHIFT) & AARCH64_IMM12_MASK;
                let shift = if (insn & AARCH64_SUB_SP_SH_BIT) != NUM_ZERO as u32 {
                    AARCH64_SUB_SP_SH_AMOUNT
                } else {
                    NUM_ZERO as u32
                };
                let allocated = (imm12 << shift) as usize;
                current_allocated += allocated;
                if current_allocated > max_bytes {
                    return Err(VerifyError::StackOverflow(current_allocated, max_bytes));
                }
            } else if (insn & AARCH64_STP_PRE_MASK) == AARCH64_STP_PRE_MATCH {
                let raw_imm7 = (insn >> AARCH64_STP_IMM7_SHIFT) & AARCH64_STP_IMM7_MASK;
                if (raw_imm7 & AARCH64_STP_IMM7_SIGN_BIT) != NUM_ZERO as u32 {
                    let signed_imm = (raw_imm7 as i32) - AARCH64_STP_SIGN_EXTEND;
                    let allocated = ((-signed_imm) as usize) * AARCH64_STP_SCALE_BYTES;
                    current_allocated += allocated;
                    if current_allocated > max_bytes {
                        return Err(VerifyError::StackOverflow(current_allocated, max_bytes));
                    }
                }
            } else if (insn & AARCH64_ADD_SP_MASK) == AARCH64_ADD_SP_MATCH {
                let imm12 = (insn >> AARCH64_IMM12_SHIFT) & AARCH64_IMM12_MASK;
                let shift = if (insn & AARCH64_SUB_SP_SH_BIT) != NUM_ZERO as u32 {
                    AARCH64_SUB_SP_SH_AMOUNT
                } else {
                    NUM_ZERO as u32
                };
                let deallocated = (imm12 << shift) as usize;
                current_allocated = current_allocated.saturating_sub(deallocated);
            } else if (insn & AARCH64_LDP_POST_MASK) == AARCH64_LDP_POST_MATCH {
                let raw_imm7 = (insn >> AARCH64_STP_IMM7_SHIFT) & AARCH64_STP_IMM7_MASK;
                if (raw_imm7 & AARCH64_STP_IMM7_SIGN_BIT) == NUM_ZERO as u32 {
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
            return Ok(());
        }

        let mut has_ret = false;
        let mut j = NUM_ZERO;
        while j + AARCH64_INSN_BYTES <= bytecode.len() {
            let insn = u32::from_le_bytes([
                bytecode[j],
                bytecode[j + NUM_ONE],
                bytecode[j + NUM_TWO],
                bytecode[j + NUM_THREE],
            ]);
            if insn == AARCH64_RET_INSN
                || (insn & AARCH64_RET_MASK) == AARCH64_RET_BASE
                || insn == AARCH64_RETAA_INSN
                || insn == AARCH64_RETAB_INSN
            {
                has_ret = true;
                break;
            }
            j += AARCH64_INSN_BYTES;
        }

        if !has_ret {
            return Err(VerifyError::CfiViolation(
                "no valid function return instruction detected".to_string(),
            ));
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
            return Ok(());
        }

        let mut current_allocated = NUM_ZERO;
        let mut idx = NUM_ZERO;
        while idx + RISCV_COMPRESSED_INSN_BYTES <= bytecode.len() {
            let is_32bit = (bytecode[idx] & RISCV_INSN_LEN_MASK) == RISCV_INSN_32BIT_MARKER;

            if is_32bit {
                if idx + RISCV_INSN_BYTES <= bytecode.len() {
                    let insn = u32::from_le_bytes([
                        bytecode[idx],
                        bytecode[idx + NUM_ONE],
                        bytecode[idx + NUM_TWO],
                        bytecode[idx + NUM_THREE],
                    ]);

                    if insn == RISCV_RET_INSN {
                        current_allocated = NUM_ZERO;
                    } else if (insn & RISCV_ADDI_SP_MASK) == RISCV_ADDI_SP_MATCH {
                        let raw_imm12 = (insn >> RISCV_ADDI_IMM12_SHIFT) & RISCV_ADDI_IMM12_MASK;
                        if (raw_imm12 & RISCV_ADDI_IMM12_SIGN_BIT) != NUM_ZERO as u32 {
                            let signed_imm = (raw_imm12 as i32) - RISCV_ADDI_SIGN_EXTEND;
                            let allocated = (-signed_imm) as usize;
                            current_allocated += allocated;
                            if current_allocated > max_bytes {
                                return Err(VerifyError::StackOverflow(
                                    current_allocated,
                                    max_bytes,
                                ));
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
                let c_insn = u16::from_le_bytes([bytecode[idx], bytecode[idx + NUM_ONE]]);
                if c_insn == RISCV_C_JR_RA_INSN || c_insn == RISCV_C_JALR_RA_INSN {
                    current_allocated = NUM_ZERO;
                } else if (c_insn & RISCV_C_ADDI16SP_MASK) == RISCV_C_ADDI16SP_MATCH
                    && (c_insn & RISCV_C_ADDI16SP_SIGN_BIT) != NUM_ZERO as u16
                {
                    let allocated = NUM_SIXTEEN;
                    current_allocated += allocated;
                    if current_allocated > max_bytes {
                        return Err(VerifyError::StackOverflow(current_allocated, max_bytes));
                    }
                }

                idx += RISCV_COMPRESSED_INSN_BYTES;
            }
        }

        Ok(())
    }

    fn verify_cfi(&self, bytecode: &[u8]) -> Result<(), VerifyError> {
        if bytecode.is_empty() {
            return Ok(());
        }

        let mut has_ret = false;
        let mut idx = NUM_ZERO;
        while idx + RISCV_COMPRESSED_INSN_BYTES <= bytecode.len() {
            let is_32bit = (bytecode[idx] & RISCV_INSN_LEN_MASK) == RISCV_INSN_32BIT_MARKER;

            if is_32bit {
                if idx + RISCV_INSN_BYTES <= bytecode.len() {
                    let insn = u32::from_le_bytes([
                        bytecode[idx],
                        bytecode[idx + NUM_ONE],
                        bytecode[idx + NUM_TWO],
                        bytecode[idx + NUM_THREE],
                    ]);
                    if insn == RISCV_RET_INSN {
                        has_ret = true;
                        break;
                    }
                    idx += RISCV_INSN_BYTES;
                } else {
                    break;
                }
            } else {
                let c_insn = u16::from_le_bytes([bytecode[idx], bytecode[idx + NUM_ONE]]);
                if c_insn == RISCV_C_JR_RA_INSN || c_insn == RISCV_C_JALR_RA_INSN {
                    has_ret = true;
                    break;
                }
                idx += RISCV_COMPRESSED_INSN_BYTES;
            }
        }

        if !has_ret {
            return Err(VerifyError::CfiViolation(
                "no valid function return instruction detected".to_string(),
            ));
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
    get_verifier(isa).verify_cfi(bytecode)
}

pub fn verify_stack_frame_bound_for_isa(
    bytecode: &[u8],
    max_bytes: usize,
    isa: TargetIsa,
) -> Result<(), VerifyError> {
    get_verifier(isa).verify_stack_frame(bytecode, max_bytes)
}

pub fn detect_isa(bytecode: &[u8]) -> Option<TargetIsa> {
    if bytecode.is_empty() {
        return None;
    }

    if bytecode.len() % RISCV_COMPRESSED_INSN_BYTES != NUM_ZERO {
        return Some(TargetIsa::X86_64);
    }

    let mut score_x86 = NUM_ZERO;
    let mut score_aarch64 = NUM_ZERO;
    let mut score_riscv = NUM_ZERO;

    if bytecode.len() % AARCH64_INSN_BYTES == NUM_ZERO {
        let mut j = NUM_ZERO;
        while j + AARCH64_INSN_BYTES <= bytecode.len() {
            let insn = u32::from_le_bytes([
                bytecode[j],
                bytecode[j + NUM_ONE],
                bytecode[j + NUM_TWO],
                bytecode[j + NUM_THREE],
            ]);

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

    let mut k = NUM_ZERO;
    while k + RISCV_COMPRESSED_INSN_BYTES <= bytecode.len() {
        let c_insn = u16::from_le_bytes([bytecode[k], bytecode[k + NUM_ONE]]);
        if c_insn == RISCV_C_JR_RA_INSN || c_insn == RISCV_C_JALR_RA_INSN {
            score_riscv += ISA_SCORE_STRONG_MARKER;
        }
        k += RISCV_COMPRESSED_INSN_BYTES;
    }

    let mut i = NUM_ZERO;
    while i < bytecode.len() {
        if bytecode[i] == X86_RET_OP || bytecode[i] == X86_RET_FAR_OP {
            score_x86 += ISA_SCORE_WEAK_MARKER;
        }
        if i + NUM_TWO < bytecode.len() {
            if bytecode[i] == X86_REX_W
                && (bytecode[i + NUM_ONE] == X86_SUB_IMM8_OP
                    || bytecode[i + NUM_ONE] == X86_SUB_IMM32_OP)
                && bytecode[i + NUM_TWO] == X86_RSP_MODRM
            {
                score_x86 += ISA_SCORE_STRONG_MARKER;
            }
            if bytecode[i] == X86_PUSH_RBP
                && bytecode[i + NUM_ONE] == X86_MOV_RBP_RSP_B0
                && bytecode[i + NUM_TWO] == X86_MOV_RBP_RSP_B1
            {
                score_x86 += ISA_SCORE_STRONG_MARKER;
            }
        }
        if i + NUM_FOUR <= bytecode.len()
            && bytecode[i] == X86_ENDBR64_B0
            && bytecode[i + NUM_ONE] == X86_ENDBR64_B1
            && bytecode[i + NUM_TWO] == X86_ENDBR64_B2
            && bytecode[i + NUM_THREE] == X86_ENDBR64_B3
        {
            score_x86 += ISA_SCORE_STRONG_MARKER;
        }
        i += NUM_ONE;
    }

    if score_aarch64 > score_x86 && score_aarch64 >= score_riscv {
        return Some(TargetIsa::AArch64);
    }
    if score_riscv > score_x86 && score_riscv > score_aarch64 {
        return Some(TargetIsa::RiscV64);
    }
    if score_x86 > NUM_ZERO {
        return Some(TargetIsa::X86_64);
    }

    None
}

pub fn verify_stack_frame_bound(bytecode: &[u8], max_bytes: usize) -> Result<(), VerifyError> {
    if bytecode.is_empty() {
        return Ok(());
    }
    let isa = detect_isa(bytecode).unwrap_or_else(TargetIsa::native);
    get_verifier(isa).verify_stack_frame(bytecode, max_bytes)
}

pub fn verify_cfi_integrity(bytecode: &[u8]) -> Result<(), VerifyError> {
    if bytecode.is_empty() {
        return Ok(());
    }
    let isa = detect_isa(bytecode).unwrap_or_else(TargetIsa::native);
    get_verifier(isa).verify_cfi(bytecode)
}
