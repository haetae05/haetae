pub const BYTE_ZERO: usize = 0;
pub const BYTE_ONE: usize = 1;

pub const NUM_NEG_ONE: i64 = 0 - 1;
pub const NUM_ZERO: usize = 0;
pub const NUM_ONE: usize = 1;
pub const NUM_TWO: usize = 2;
pub const NUM_THREE: usize = 3;
pub const NUM_FOUR: usize = 4;
pub const NUM_FIVE: usize = 5;
pub const NUM_SIX: usize = 6;
pub const NUM_SEVEN: usize = 7;
pub const NUM_EIGHT: usize = 8;
pub const NUM_NINE: usize = 9;
pub const NUM_TEN: usize = 10;
pub const NUM_ELEVEN: usize = 11;
pub const NUM_TWELVE: usize = 12;
pub const NUM_THIRTEEN: usize = 13;
pub const NUM_FOURTEEN: usize = 14;
pub const NUM_FIFTEEN: usize = 15;
pub const NUM_SIXTEEN: usize = 16;
pub const NUM_SEVENTEEN: usize = 17;
pub const NUM_EIGHTEEN: usize = 18;
pub const NUM_NINETEEN: usize = 19;
pub const NUM_TWENTY: usize = 20;
pub const NUM_TWENTY_ONE: usize = 21;
pub const NUM_TWENTY_TWO: usize = 22;
pub const NUM_TWENTY_THREE: usize = 23;
pub const NUM_TWENTY_FOUR: usize = 24;
pub const NUM_TWENTY_FIVE: usize = 25;
pub const NUM_TWENTY_SIX: usize = 26;
pub const NUM_TWENTY_SEVEN: usize = 27;
pub const NUM_TWENTY_EIGHT: usize = 28;
pub const NUM_TWENTY_NINE: usize = 29;
pub const NUM_THIRTY: usize = 30;
pub const NUM_THIRTY_ONE: usize = 31;
pub const NUM_THIRTY_TWO: usize = 32;
pub const NUM_THIRTY_THREE: usize = 33;
pub const NUM_THIRTY_FOUR: usize = 34;
pub const NUM_THIRTY_FIVE: usize = 35;
pub const NUM_THIRTY_SIX: usize = 36;
pub const NUM_THIRTY_SEVEN: usize = 37;
pub const NUM_THIRTY_EIGHT: usize = 38;
pub const NUM_THIRTY_NINE: usize = 39;
pub const NUM_FORTY: usize = 40;
pub const NUM_FORTY_ONE: usize = 41;
pub const NUM_FORTY_TWO: usize = 42;
pub const NUM_FORTY_THREE: usize = 43;
pub const NUM_FORTY_FOUR: usize = 44;
pub const NUM_FORTY_FIVE: usize = 45;
pub const NUM_FORTY_SIX: usize = 46;
pub const NUM_FORTY_SEVEN: usize = 47;
pub const NUM_FORTY_EIGHT: usize = 48;
pub const NUM_FORTY_NINE: usize = 49;
pub const NUM_FIFTY: usize = 50;
pub const NUM_FIFTY_ONE: usize = 51;
pub const NUM_FIFTY_TWO: usize = 52;
pub const NUM_FIFTY_THREE: usize = 53;
pub const NUM_FIFTY_FOUR: usize = 54;
pub const NUM_SIXTY: usize = 60;
pub const NUM_SIXTY_FOUR: usize = 64;
pub const NUM_SEVENTY: usize = 70;
pub const NUM_SEVENTY_FIVE: usize = 75;
pub const NUM_EIGHTY: usize = 80;
pub const NUM_NINETY_SIX: usize = 96;
pub const NUM_ONE_HUNDRED: usize = 100;
pub const NUM_ONE_HUNDRED_TWENTY: usize = 120;
pub const NUM_ONE_TWENTY_EIGHT: usize = 128;
pub const NUM_ONE_HUNDRED_FIFTY: usize = 150;
pub const NUM_TWO_HUNDRED: usize = 200;
pub const NUM_TWO_HUNDRED_TEN: usize = 210;
pub const NUM_TWO_HUNDRED_TWENTY_FIVE: usize = 225;
pub const NUM_TWO_HUNDRED_FORTY: usize = 240;
pub const NUM_TWO_HUNDRED_FIFTY: usize = 250;
pub const NUM_THREE_HUNDRED: usize = 300;
pub const NUM_THREE_HUNDRED_SIXTY: usize = 360;
pub const NUM_FOUR_HUNDRED_FIFTY: usize = 450;
pub const NUM_FIVE_HUNDRED: usize = 500;
pub const NUM_FIVE_HUNDRED_TWELVE: usize = 512;
pub const NUM_EIGHT_HUNDRED_TEN: usize = 810;
pub const NUM_ONE_THOUSAND: usize = 1000;
pub const NUM_ONE_THOUSAND_TWENTY_FOUR: usize = 1024;
pub const NUM_TWO_THOUSAND: usize = 2000;
pub const NUM_FOUR_THOUSAND_NINETY_SIX: usize = 4096;
pub const NUM_FIVE_THOUSAND: usize = 5000;
pub const NUM_TEN_THOUSAND: usize = 10_000;
pub const NUM_TWENTY_THOUSAND: usize = 20_000;
pub const NUM_TWENTY_FIVE_THOUSAND: usize = 25_000;
pub const NUM_FIFTY_THOUSAND: usize = 50_000;
pub const NUM_SIXTY_THOUSAND: usize = 60_000;
pub const NUM_ONE_HUNDRED_THOUSAND: usize = 100_000;
pub const NUM_TWO_HUNDRED_THOUSAND: usize = 200_000;
pub const NUM_THREE_HUNDRED_THOUSAND: usize = 300_000;
pub const NUM_FOUR_HUNDRED_THOUSAND: usize = 400_000;
pub const NUM_FIVE_HUNDRED_THOUSAND: usize = 500_000;

pub const BOOST_STAGE1_SWAP_ITERATIONS: usize = 50;
pub const BOOST_STAGE2_MIM_TOKENS: usize = 50_000;
pub const BOOST_STAGE3_FUZZ_ITERATIONS: usize = 100_000;
pub const BOOST_STAGE12_ALGO_ITERATIONS: usize = 20_000;

pub const NUM_TWO_HUNDRED_FIFTY_FIVE: i64 = 255;
pub const NUM_TWO_HUNDRED_FIFTY_SIX: i64 = 256;
pub const NUM_SIXTY_FIVE_THOUSAND_FIVE_HUNDRED_THIRTY_FIVE: i64 = 65535;
pub const NUM_SIXTY_FIVE_THOUSAND_FIVE_HUNDRED_THIRTY_SIX: i64 = 65536;

pub const DEFAULT_FUZZ_ITERATIONS: usize = 10_000;
pub const MAX_STACK_FRAME_BYTES: usize = 1_048_576;
pub const MAX_CALL_STACK_DEPTH: usize = NUM_SIXTY_FOUR;
pub const DEFAULT_CONTEXT_WINDOW_TOKENS: usize = 1_024;
pub const BYTES_PER_KB: usize = 1_024;
pub const BYTES_PER_MB: usize = 1_048_576;
pub const MILLIS_PER_SEC: f64 = 1000.0;
pub const BENCHMARK_SAMPLE_DURATION_MS: f64 = 1.5;

pub const BUFFER_CAPACITY_BYTES: usize = 4096;
pub const JMP_BUF_SIZE_BYTES: usize = 1024;

pub const X86_NOP_OPCODE: u8 = 0x90;
pub const X86_RET_OPCODE: u8 = 0xc3;
pub const X86_REX_W: u8 = 0x48;
pub const X86_SUB_IMM8_OP: u8 = 0x83;
pub const X86_SUB_IMM32_OP: u8 = 0x81;
pub const X86_RSP_MODRM: u8 = 0xec;
pub const X86_RET_OP: u8 = 0xc3;
pub const X86_RET_FAR_OP: u8 = 0xcb;
pub const X86_RET_IMM16_OP: u8 = 0xc2;
pub const X86_RET_FAR_IMM16_OP: u8 = 0xca;
pub const X86_PUSH_RBP: u8 = 0x55;
pub const X86_MOV_RBP_RSP_B0: u8 = 0x48;
pub const X86_MOV_RBP_RSP_B1: u8 = 0x89;
pub const X86_MOV_RBP_RSP_B2: u8 = 0xe5;
pub const X86_ENTER_OP: u8 = 0xc8;
pub const X86_ENDBR64_B0: u8 = 0xf3;
pub const X86_ENDBR64_B1: u8 = 0x0f;
pub const X86_ENDBR64_B2: u8 = 0x1e;
pub const X86_ENDBR64_B3: u8 = 0xfa;
pub const X86_ADD_IMM8_OP: u8 = 0x83;
pub const X86_ADD_IMM32_OP: u8 = 0x81;
pub const X86_RSP_ADD_MODRM: u8 = 0xc4;
pub const X86_LEAVE_OP: u8 = 0xc9;

pub const AARCH64_INSN_BYTES: usize = NUM_FOUR;
pub const AARCH64_RET_INSN: u32 = 0xd65f03c0;
pub const AARCH64_RETAA_INSN: u32 = 0xd65f0bff;
pub const AARCH64_RETAB_INSN: u32 = 0xd65f0fff;
pub const AARCH64_RET_MASK: u32 = 0xfffffc1f;
pub const AARCH64_RET_BASE: u32 = 0xd65f0000;
pub const AARCH64_SUB_SP_MASK: u32 = 0xffc003e0;
pub const AARCH64_SUB_SP_MATCH: u32 = 0xd10003e0;
pub const AARCH64_SUB_SP_EXACT_MASK: u32 = 0xff8003ff;
pub const AARCH64_SUB_SP_EXACT_MATCH: u32 = 0xd10003ff;
pub const AARCH64_ADD_SP_MASK: u32 = 0xffc003e0;
pub const AARCH64_ADD_SP_MATCH: u32 = 0x910003e0;
pub const AARCH64_IMM12_SHIFT: u32 = NUM_TEN as u32;
pub const AARCH64_IMM12_MASK: u32 = 0xfff;
pub const AARCH64_SUB_SP_SH_BIT: u32 = 0x00400000;
pub const AARCH64_SUB_SP_SH_AMOUNT: u32 = NUM_TWELVE as u32;
pub const AARCH64_STP_PRE_MASK: u32 = 0xffc003e0;
pub const AARCH64_STP_PRE_MATCH: u32 = 0xa98003e0;
pub const AARCH64_LDP_POST_MASK: u32 = 0xffc003e0;
pub const AARCH64_LDP_POST_MATCH: u32 = 0xa8c003e0;
pub const AARCH64_STP_IMM7_SHIFT: u32 = NUM_FIFTEEN as u32;
pub const AARCH64_STP_IMM7_MASK: u32 = 0x7f;
pub const AARCH64_STP_IMM7_SIGN_BIT: u32 = 0x40;
pub const AARCH64_STP_SCALE_BYTES: usize = NUM_EIGHT;
pub const AARCH64_STP_SIGN_EXTEND: i32 = NUM_ONE_TWENTY_EIGHT as i32;
pub const AARCH64_BTI_MATCH: u32 = 0xd503241f;
pub const AARCH64_STP_X29_X30_MINUS_16: u32 = 0xa9bf7bfd;

pub const RISCV_INSN_BYTES: usize = NUM_FOUR;
pub const RISCV_COMPRESSED_INSN_BYTES: usize = NUM_TWO;
pub const RISCV_INSN_LEN_MASK: u8 = 0x03;
pub const RISCV_INSN_32BIT_MARKER: u8 = 0x03;
pub const RISCV_RET_INSN: u32 = 0x00008067;
pub const RISCV_C_JR_RA_INSN: u16 = 0x8082;
pub const RISCV_C_JALR_RA_INSN: u16 = 0x9082;
pub const RISCV_ADDI_SP_MASK: u32 = 0x000fffff;
pub const RISCV_ADDI_SP_MATCH: u32 = 0x00010113;
pub const RISCV_ADDI_IMM12_SHIFT: u32 = NUM_TWENTY as u32;
pub const RISCV_ADDI_IMM12_MASK: u32 = 0xfff;
pub const RISCV_ADDI_IMM12_SIGN_BIT: u32 = 0x800;
pub const RISCV_ADDI_SIGN_EXTEND: i32 = NUM_FOUR_THOUSAND_NINETY_SIX as i32;
pub const RISCV_ADDI_SP_MINUS_64: u32 = 0xfc010113;
pub const RISCV_C_ADDI16SP_MASK: u16 = 0xef83;
pub const RISCV_C_ADDI16SP_MATCH: u16 = 0x6101;
pub const RISCV_C_ADDI16SP_SIGN_BIT: u16 = 0x1000;
pub const RISCV_C_ADDI16SP_SIGN_EXTEND: i32 = NUM_ONE_THOUSAND_TWENTY_FOUR as i32;

pub const ISA_SCORE_STRONG_MARKER: usize = NUM_TEN;
pub const ISA_SCORE_WEAK_MARKER: usize = NUM_ONE;
pub const DIAG_LINE_NUMBER_WIDTH: usize = NUM_FOUR;
pub const HASH_MULTIPLIER_PRIME: usize = NUM_THIRTY_ONE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TargetIsa {
    X86_64,
    AArch64,
    RiscV64,
}

impl TargetIsa {
    pub const fn native() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self::X86_64
        }
        #[cfg(target_arch = "aarch64")]
        {
            Self::AArch64
        }
        #[cfg(target_arch = "riscv64")]
        {
            Self::RiscV64
        }
        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64"
        )))]
        {
            Self::X86_64
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::AArch64 => "aarch64",
            Self::RiscV64 => "riscv64",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(name: &str) -> Option<Self> {
        let lower = name.trim().to_ascii_lowercase();
        match lower.as_str() {
            "x86_64" | "x86-64" | "x64" | "amd64" => Some(Self::X86_64),
            "aarch64" | "arm64" => Some(Self::AArch64),
            "riscv64" | "riscv" | "rv64" => Some(Self::RiscV64),
            _ => None,
        }
    }
}

impl std::str::FromStr for TargetIsa {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(())
    }
}

impl Default for TargetIsa {
    fn default() -> Self {
        Self::native()
    }
}

pub const POSIX_SIGFPE: i32 = 8;
pub const POSIX_SIGSEGV: i32 = 11;
pub const POSIX_SIGILL: i32 = 4;
pub const POSIX_SIGBUS: i32 = 7;
pub const C_INT_MAX: i64 = 2147483647;
pub const C_INT_MIN: i64 = -2147483648;
pub const HEX_RADIX: u32 = NUM_SIXTEEN as u32;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub fuzz_iterations: usize,
    pub max_stack_bytes: usize,
    pub target_isa: TargetIsa,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fuzz_iterations: DEFAULT_FUZZ_ITERATIONS,
            max_stack_bytes: MAX_STACK_FRAME_BYTES,
            target_isa: TargetIsa::default(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(v) = std::env::var("HAETAE_FUZZ_ITERATIONS") {
            if let Ok(n) = v.parse::<usize>() {
                cfg.fuzz_iterations = n;
            }
        }
        if let Ok(v) = std::env::var("HAETAE_MAX_STACK_BYTES") {
            if let Ok(n) = v.parse::<usize>() {
                cfg.max_stack_bytes = n;
            }
        }
        if let Ok(v) = std::env::var("HAETAE_TARGET_ISA") {
            if let Some(isa) = TargetIsa::from_str(&v) {
                cfg.target_isa = isa;
            }
        }
        cfg
    }
}
