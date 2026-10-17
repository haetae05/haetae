// ============================================================================
// Haetae System & Domain Configuration Constants
// ============================================================================

pub const BYTE_ZERO: usize = 0;
pub const BYTE_ONE: usize = 1;
pub const BISECTION_DIVISOR: usize = 2;
pub const STEP_PEEK: usize = 2;
pub const STEP_TRIO: usize = 3;

pub const BOOST_STAGE1_SWAP_ITERATIONS: usize = 50;
pub const BOOST_STAGE2_MIM_TOKENS: usize = 50_000;
pub const BOOST_STAGE3_FUZZ_ITERATIONS: usize = 100_000;
pub const BOOST_STAGE12_ALGO_ITERATIONS: usize = 20_000;
pub const BOOST_TOTAL_STAGES: usize = 12;

pub const BYTE_MAX_VALUE: i64 = 255;
pub const BYTE_VALUES_COUNT: i64 = 256;
pub const U16_MAX_VALUE: i64 = 65535;
pub const U16_VALUES_COUNT: i64 = 65536;

pub const MAX_FILES_FOR_CROSS_REF: usize = 50;
pub const MAX_SYMBOLS_FOR_CROSS_REF: usize = 100;
pub const MAX_COLLECTED_SOURCE_FILES: usize = 100;
pub const SUPPORTED_LANGUAGES_COUNT: usize = 15;

pub const DEFAULT_FUZZ_ITERATIONS: usize = 10_000;
pub const MAX_STACK_FRAME_BYTES: usize = 1_048_576;
pub const ALT_SIGNAL_STACK_SIZE_BYTES: usize = 131_072;
pub const MAX_CALL_STACK_DEPTH: usize = 64;
pub const CACHE_LINE_BYTES: usize = 64;
pub const L3_CACHE_BOUND_MB: usize = 32;
pub const WEIGHT_SLICE_SIZE_MB: usize = 35;
pub const L3_SAFETY_MARGIN_MB: usize = 5;
pub const TOKENS_PER_CYCLE: usize = 2;
pub const COLLISION_WINDOW_TOKENS: usize = 3;
pub const DEFAULT_CONTEXT_WINDOW_TOKENS: usize = 1_024;
pub const MAX_CONTEXT_WINDOW_TOKENS: usize = 32_768;
pub const SERVER_PORT: u16 = 8080;
pub const VSIX_MAX_SIZE_MB: usize = 5;
pub const MARKETPLACE_LIMIT_MB: usize = 25;
pub const BYTES_PER_KB: usize = 1_024;
pub const BYTES_PER_MB: usize = 1_048_576;
pub const HTTP_TIMEOUT_SECS: u64 = 30;
pub const HTTP_CONNECT_TIMEOUT_SECS: u64 = 2;
pub const PAGERANK_DAMPING_FACTOR: f64 = 0.85;
pub const PAGERANK_MAX_ITERATIONS: usize = 100;
pub const PAGERANK_CONVERGENCE_EPSILON: f64 = 0.0001;
pub const MODEL_DOWNLOAD_CHUNK_SIZE: usize = 65_536;
pub const GGUF_HEADER_PROBE_BYTES: usize = 1_024;
pub const GGUF_MAGIC_LE: u32 = 0x46554747;
pub const GGUF_VERSION_V3: u32 = 3;
pub const ELF_CLASS_64: u8 = 2;
pub const MILLIS_PER_SEC: f64 = 1000.0;
pub const FLOAT_EPSILON: f64 = 0.01;
pub const MB_PER_GB: f64 = 1024.0;
pub const MIN_BENCHMARK_ELAPSED_SECS: f64 = 0.0001;
pub const BENCHMARK_SAMPLE_DURATION_MS: f64 = 1.5;

pub const HTTP_OK: u16 = 200;
pub const HTTP_PARTIAL_CONTENT: u16 = 206;
pub const HTTP_BAD_REQUEST: u16 = 400;
pub const HTTP_UNAUTHORIZED: u16 = 401;
pub const HTTP_FORBIDDEN: u16 = 403;
pub const HTTP_NOT_FOUND: u16 = 404;
pub const HTTP_INTERNAL_ERROR: u16 = 500;
pub const HTTP_USER_AGENT: &str = "Haetae-Contract-Engine/2.0";

pub const ESTIMATED_CHARS_PER_TOKEN: usize = 4;
pub const BUFFER_CAPACITY_BYTES: usize = 4096;
pub const JMP_BUF_SIZE_BYTES: usize = 1024;
pub const DEFAULT_LICENSE_EXPIRY_TIMESTAMP: u64 = 1_791_642_618;
pub const OFFLINE_GRACE_PERIOD_SECS: u64 = 604_800;
pub const DEFAULT_OFFLINE_GRACE_DAYS: u64 = 7;
pub const LICENSE_VALIDITY_DAYS: u64 = 30;
pub const SECS_PER_DAY: u64 = 86_400;

pub const SERVER_BUFFER_CHUNK_SIZE: usize = MODEL_DOWNLOAD_CHUNK_SIZE;
pub const SIZE_HAETAE_EXE_BYTES: u64 = 3_203_072;
pub const SIZE_HAETAE_LINUX_ARM64_BYTES: u64 = 3_348_488;
pub const SIZE_HAETAE_VSIX_BYTES: u64 = 3_224_622;

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

pub const AARCH64_INSN_BYTES: usize = 4;
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
pub const AARCH64_IMM12_SHIFT: u32 = 10;
pub const AARCH64_IMM12_MASK: u32 = 0xfff;
pub const AARCH64_SUB_SP_SH_BIT: u32 = 0x00400000;
pub const AARCH64_SUB_SP_SH_AMOUNT: u32 = 12;
pub const AARCH64_STP_PRE_MASK: u32 = 0xffc003e0;
pub const AARCH64_STP_PRE_MATCH: u32 = 0xa98003e0;
pub const AARCH64_LDP_POST_MASK: u32 = 0xffc003e0;
pub const AARCH64_LDP_POST_MATCH: u32 = 0xa8c003e0;
pub const AARCH64_STP_IMM7_SHIFT: u32 = 15;
pub const AARCH64_STP_IMM7_MASK: u32 = 0x7f;
pub const AARCH64_STP_IMM7_SIGN_BIT: u32 = 0x40;
pub const AARCH64_STP_SCALE_BYTES: usize = 8;
pub const AARCH64_STP_SIGN_EXTEND: i32 = 128;
pub const AARCH64_BTI_MATCH: u32 = 0xd503241f;
pub const AARCH64_STP_X29_X30_MINUS_16: u32 = 0xa9bf7bfd;

pub const RISCV_INSN_BYTES: usize = 4;
pub const RISCV_COMPRESSED_INSN_BYTES: usize = 2;
pub const RISCV_INSN_LEN_MASK: u8 = 0x03;
pub const RISCV_INSN_32BIT_MARKER: u8 = 0x03;
pub const RISCV_RET_INSN: u32 = 0x00008067;
pub const RISCV_C_JR_RA_INSN: u16 = 0x8082;
pub const RISCV_C_JALR_RA_INSN: u16 = 0x9082;
pub const RISCV_ADDI_SP_MASK: u32 = 0x000fffff;
pub const RISCV_ADDI_SP_MATCH: u32 = 0x00010113;
pub const RISCV_ADDI_IMM12_SHIFT: u32 = 20;
pub const RISCV_ADDI_IMM12_MASK: u32 = 0xfff;
pub const RISCV_ADDI_IMM12_SIGN_BIT: u32 = 0x800;
pub const RISCV_ADDI_SIGN_EXTEND: i32 = 4096;
pub const RISCV_ADDI_SP_MINUS_64: u32 = 0xfc010113;
pub const RISCV_C_ADDI16SP_MASK: u16 = 0xef83;
pub const RISCV_C_ADDI16SP_MATCH: u16 = 0x6101;
pub const RISCV_C_ADDI16SP_SIGN_BIT: u16 = 0x1000;
pub const RISCV_C_ADDI16SP_SIGN_EXTEND: i32 = 1024;

pub const ISA_SCORE_STRONG_MARKER: usize = 10;
pub const ISA_SCORE_WEAK_MARKER: usize = 1;
pub const DIAG_LINE_NUMBER_WIDTH: usize = 4;
pub const HASH_MULTIPLIER_PRIME: usize = 31;

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
pub const HEX_CHARS_PER_BYTE: usize = 2;
pub const TOKEN_PARTS_COUNT: usize = 2;
pub const HMAC_SHA256_TAG_BYTES: usize = 32;
pub const HEX_RADIX: u32 = 16;
pub const ENV_HAETAE_LICENSE_KEY: &str = "HAETAE_LICENSE_KEY";
pub const ENV_HAETAE_LICENSE_SECRET: &str = "HAETAE_LICENSE_SECRET";
pub const ENV_HAETAE_MACHINE_ID: &str = "HAETAE_MACHINE_ID";
pub const ENV_HAETAE_SERVER_URL: &str = "HAETAE_SERVER_URL";
pub const ENV_HAETAE_OLLAMA_HOST: &str = "HAETAE_OLLAMA_HOST";
pub const SHA256_HEX_STRING_LEN: usize = 64;
pub const SHA256_DIGEST_BYTES: usize = 32;
pub const MAC_ADDRESS_STR_LEN: usize = 17;
pub const LICENSE_KEY_PREFIX: &str = "HAETAE-";
pub const LICENSE_KEY_PARTS_COUNT: usize = 4;
pub const LEMONSQUEEZY_ACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/activate";
pub const LEMONSQUEEZY_VALIDATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/validate";
pub const LEMONSQUEEZY_TIMEOUT_SECS: u64 = 5;
pub const LEMONSQUEEZY_STATUS_ACTIVE: &str = "active";

pub const L3_BENCHMARK_ITERATIONS: usize = 10;
pub const MIM_BENCHMARK_ITERATIONS: usize = 1000;
pub const MIM_COLLISION_CYCLE_STEPS: usize = 10;
pub const BENCHMARK_INFERENCE_PROMPT: &str = "int add(int a, int b) {";

pub const QWEN_MODEL_URL: &str =
    "https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf";
pub const JINA_SEARCH_BASE_URL: &str = "https://s.jina.ai/";
pub const JINA_READER_BASE_URL: &str = "https://r.jina.ai/";
pub const DEFAULT_ORACLE_SERVER_HOST: &str = "http://152.69.226.30:8080";
pub const DEFAULT_LOCAL_SERVER_HOST: &str = "http://127.0.0.1:8080";
pub const LOCAL_HOST_IPV4: &str = "127.0.0.1";

pub const DEFAULT_OLLAMA_HOST: &str = "http://127.0.0.1:11434";
pub const DEFAULT_OLLAMA_MODEL: &str = "huihui_ai/qwen2.5-coder-abliterate:1.5b";
pub const OLLAMA_GENERATE_ENDPOINT: &str = "/api/generate";
pub const OLLAMA_PULL_ENDPOINT: &str = "/api/pull";
pub const OLLAMA_TAGS_ENDPOINT: &str = "/api/tags";
pub const OLLAMA_NUM_PREDICT: usize = DEFAULT_CONTEXT_WINDOW_TOKENS;
pub const OLLAMA_PROBE_ENDPOINT: &str = "/";
pub const OLLAMA_PROBE_BODY_MARKER: &str = "Ollama is running";
pub const OLLAMA_POLL_INTERVAL_MS: u64 = 250;
pub const OLLAMA_POLL_MAX_ATTEMPTS: usize = 20;
pub const OLLAMA_CONNECT_TIMEOUT_SECS: u64 = 2;
pub const OLLAMA_DOWNLOAD_PAGE_URL: &str = "https://ollama.com/download";
pub const OLLAMA_CMD_WINGET_INSTALL: &str = "winget install --id Ollama.Ollama -e";
pub const OLLAMA_CMD_BREW_INSTALL: &str = "brew install --cask ollama";
pub const OLLAMA_CMD_CURL_INSTALL: &str = "curl -fsSL https://ollama.com/install.sh | sh";
pub const WIN_DETACHED_PROCESS_FLAG: u32 = 0x00000008;
pub const WIN_CREATE_NO_WINDOW_FLAG: u32 = 0x08000000;
pub const INFERENCE_TIMEOUT_SECS: u64 = 300;
pub const MODEL_PULL_TIMEOUT_SECS: u64 = 600;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub fuzz_iterations: usize,
    pub max_stack_bytes: usize,
    pub context_tokens: usize,
    pub server_url: String,
    pub l3_cache_bound_mb: usize,
    pub weight_slice_size_mb: usize,
    pub tokens_per_cycle: usize,
    pub collision_window_tokens: usize,
    pub model_download_url: String,
    pub jina_search_url: String,
    pub jina_reader_url: String,
    pub ollama_host: String,
    pub ollama_model: String,
    pub target_isa: TargetIsa,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fuzz_iterations: DEFAULT_FUZZ_ITERATIONS,
            max_stack_bytes: MAX_STACK_FRAME_BYTES,
            context_tokens: DEFAULT_CONTEXT_WINDOW_TOKENS,
            server_url: DEFAULT_ORACLE_SERVER_HOST.to_string(),
            l3_cache_bound_mb: L3_CACHE_BOUND_MB,
            weight_slice_size_mb: WEIGHT_SLICE_SIZE_MB,
            tokens_per_cycle: TOKENS_PER_CYCLE,
            collision_window_tokens: COLLISION_WINDOW_TOKENS,
            model_download_url: QWEN_MODEL_URL.to_string(),
            jina_search_url: JINA_SEARCH_BASE_URL.to_string(),
            jina_reader_url: JINA_READER_BASE_URL.to_string(),
            ollama_host: DEFAULT_OLLAMA_HOST.to_string(),
            ollama_model: DEFAULT_OLLAMA_MODEL.to_string(),
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
        if let Ok(v) = std::env::var("HAETAE_CONTEXT_TOKENS") {
            if let Ok(n) = v.parse::<usize>() {
                cfg.context_tokens = n;
            }
        }
        if let Ok(v) = std::env::var("HAETAE_SERVER_URL") {
            cfg.server_url = v;
        }
        if let Ok(v) = std::env::var("HAETAE_OLLAMA_HOST") {
            cfg.ollama_host = v;
        } else if let Ok(v) = std::env::var("OLLAMA_HOST") {
            cfg.ollama_host = v;
        }
        if let Ok(v) = std::env::var("HAETAE_MODEL").or_else(|_| std::env::var("HAETAE_MODEL_NAME"))
        {
            cfg.ollama_model = v;
        }
        if let Ok(v) = std::env::var("HAETAE_TARGET_ISA") {
            if let Some(isa) = TargetIsa::from_str(&v) {
                cfg.target_isa = isa;
            }
        }
        cfg
    }
}

// ============================================================================
// XorShift64 PRNG & Multi-Parameter Payload Derivation Constants
// ============================================================================
pub const XORSHIFT_SHIFT_A: u32 = 13;
pub const XORSHIFT_SHIFT_B: u32 = 7;
pub const XORSHIFT_SHIFT_C: u32 = 17;
pub const XORSHIFT_DEFAULT_SEED: u64 = 0x8a5cd789635d2dff;
pub const XORSHIFT_PAYLOAD_MASK: u64 = 0xffffffffffffffff;
pub const PRNG_DERIVE_PRIME_1: u64 = 0x9e3779b97f4a7c15;
pub const PRNG_DERIVE_PRIME_2: u64 = 0x6a09e667f3bcc908;
pub const PRNG_DERIVE_SHIFT_1: u32 = 30;
pub const PRNG_DERIVE_SHIFT_2: u32 = 27;
pub const PRNG_DERIVE_SHIFT_3: u32 = 31;

// ============================================================================
// Fuzzer Mutation & Boundary Configuration
// ============================================================================
pub const FUZZ_BOUNDARY_POWER_TWO_COUNT: usize = 64;
pub const FUZZ_DEFAULT_MUTATION_RATE: usize = 16;
pub const FUZZ_SAMPLE_STRIDE: usize = 7;
pub const FUZZ_MUTATION_MASK_1: u64 = 0x0000000000000001;
pub const FUZZ_MUTATION_MASK_2: u64 = 0x8000000000000000;
pub const FUZZ_MUTATION_MASK_3: u64 = 0x0000000080000000;
pub const FUZZ_MUTATION_MASK_4: u64 = 0x0000000000008000;
pub const FUZZ_MUTATION_MASK_5: u64 = 0x0000000000000080;
pub const FUZZ_MAX_ARG_COUNT: usize = 8;

// ============================================================================
// Clang JIT Compilation & Native Execution
// ============================================================================
pub const CLANG_DEFAULT_PATH_WIN: &str = r"C:\Program Files\LLVM\bin\clang.exe";
pub const CLANG_ENV_VAR: &str = "HAETAE_CLANG_PATH";
pub const CLANG_OPT_O2: &str = "-O2";
pub const CLANG_OPT_O0: &str = "-O0";
pub const CLANG_FLAG_SHARED: &str = "-shared";
pub const CLANG_FLAG_FPIC: &str = "-fPIC";
pub const CLANG_FLAG_COMPILE_ONLY: &str = "-c";
pub const CLANG_TEMP_PREFIX: &str = "haetae_jit_";
pub const WIN_EXCEPTION_EXECUTE_HANDLER: i32 = 1;
pub const WIN_EXCEPTION_CONTINUE_SEARCH: i32 = 0;
pub const WIN_EXCEPTION_ACCESS_VIOLATION: u32 = 0xC0000005;
pub const WIN_EXCEPTION_INT_DIVIDE_BY_ZERO: u32 = 0xC0000094;
pub const WIN_EXCEPTION_FLT_DIVIDE_BY_ZERO: u32 = 0xC000008E;
pub const WIN_EXCEPTION_STACK_OVERFLOW: u32 = 0xC00000FD;

// ============================================================================
// PE / COFF Object & DLL Parsing Constants
// ============================================================================
pub const PE_DOS_HEADER_LFANEW_OFFSET: usize = 0x3C;
pub const PE_SIGNATURE_BYTES: usize = 4;
pub const COFF_FILE_HEADER_BYTES: usize = 20;
pub const COFF_SECTION_HEADER_BYTES: usize = 40;
pub const COFF_SECTION_NAME_BYTES: usize = 8;
pub const COFF_SECTION_RAW_SIZE_OFFSET: usize = 16;
pub const COFF_SECTION_RAW_PTR_OFFSET: usize = 20;
pub const COFF_NUM_SECTIONS_OFFSET: usize = 2;
pub const COFF_OPTHDR_SIZE_OFFSET: usize = 16;
pub const COFF_SECT_HDR_START: usize = 20;

// ============================================================================
// Extended Instruction Decoding Constants (x86_64, AArch64, RISC-V)
// ============================================================================
pub const X86_MAX_INSN_LEN: usize = 15;
pub const X86_OPCODE_JMP_REL8: u8 = 0xeb;
pub const X86_OPCODE_JMP_REL32: u8 = 0xe9;
pub const X86_OPCODE_CALL_REL32: u8 = 0xe8;
pub const X86_OPCODE_JCC_REL8_BASE: u8 = 0x70;
pub const X86_OPCODE_JCC_REL8_COUNT: u8 = 16;
pub const X86_OPCODE_TWO_BYTE_ESCAPE: u8 = 0x0f;
pub const X86_OPCODE_JCC_REL32_BASE: u8 = 0x80;
pub const X86_OPCODE_JCC_REL32_COUNT: u8 = 16;
pub const X86_MODRM_MOD_MASK: u8 = 0xc0;
pub const X86_MODRM_REG_MASK: u8 = 0x38;
pub const X86_MODRM_RM_MASK: u8 = 0x07;
pub const X86_MODRM_MOD_DIRECT: u8 = 0xc0;
pub const X86_MODRM_MOD_DISP8: u8 = 0x40;
pub const X86_MODRM_MOD_DISP32: u8 = 0x80;
pub const X86_MODRM_RM_SIB: u8 = 0x04;
pub const X86_MODRM_RM_DISP32_ONLY: u8 = 0x05;
pub const X86_SIB_BASE_DISP32_ONLY: u8 = 0x05;
pub const X86_REG_RSP_CODE: u8 = 4;
pub const X86_SUB_REG_CODE: u8 = 5;
pub const X86_ADD_REG_CODE: u8 = 0;
pub const X86_OPCODE_INT3: u8 = 0xcc;
pub const X86_OPCODE_GRP1_IMM8: u8 = X86_SUB_IMM8_OP;
pub const X86_OPCODE_GRP1_IMM32: u8 = X86_SUB_IMM32_OP;
pub const X86_OPCODE_MOV_RM_R: u8 = 0x89;
pub const X86_OPCODE_MOV_R_RM: u8 = 0x8b;
pub const X86_OPCODE_LEA: u8 = 0x8d;
pub const X86_OPCODE_GRP3_NOT_NEG: u8 = 0xf7;
pub const X86_OPCODE_GRP5_JMP_CALL: u8 = 0xff;
pub const X86_OPCODE_PUSH_REG_MIN: u8 = 0x50;
pub const X86_OPCODE_PUSH_REG_MAX: u8 = 0x57;
pub const X86_OPCODE_POP_REG_MIN: u8 = 0x58;
pub const X86_OPCODE_POP_REG_MAX: u8 = 0x5f;
pub const X86_PREFIX_LOCK: u8 = 0xf0;
pub const X86_PREFIX_REPNE: u8 = 0xf2;
pub const X86_PREFIX_REP: u8 = 0xf3;
pub const X86_PREFIX_CS_OVERRIDE: u8 = 0x2e;
pub const X86_PREFIX_SS_OVERRIDE: u8 = 0x36;
pub const X86_PREFIX_DS_OVERRIDE: u8 = 0x3e;
pub const X86_PREFIX_ES_OVERRIDE: u8 = 0x26;
pub const X86_PREFIX_FS_OVERRIDE: u8 = 0x64;
pub const X86_PREFIX_GS_OVERRIDE: u8 = 0x65;
pub const X86_PREFIX_OPERAND_SIZE: u8 = 0x66;
pub const X86_PREFIX_ADDRESS_SIZE: u8 = 0x67;
pub const X86_REX_MIN: u8 = 0x40;
pub const X86_REX_MAX: u8 = 0x4f;

pub const AARCH64_BRANCH_SCALE_BYTES: usize = 4;

pub const AARCH64_B_BL_MASK: u32 = 0xfc000000;
pub const AARCH64_B_MATCH: u32 = 0x14000000;
pub const AARCH64_BL_MATCH: u32 = 0x94000000;
pub const AARCH64_B_COND_MASK: u32 = 0xff000010;
pub const AARCH64_B_COND_MATCH: u32 = 0x54000000;
pub const AARCH64_CBZ_CBNZ_MASK: u32 = 0x7e000000;
pub const AARCH64_CBZ_MATCH: u32 = 0x34000000;
pub const AARCH64_CBNZ_MATCH: u32 = 0x35000000;
pub const AARCH64_IMM26_MASK: u32 = 0x03ffffff;
pub const AARCH64_IMM26_SIGN_BIT: u32 = 0x02000000;
pub const AARCH64_IMM26_SIGN_EXTEND: i32 = 0x04000000;
pub const AARCH64_IMM19_MASK: u32 = 0x7ffff;
pub const AARCH64_IMM19_SHIFT: u32 = 5;
pub const AARCH64_IMM19_SIGN_BIT: u32 = 0x40000;
pub const AARCH64_IMM19_SIGN_EXTEND: i32 = 0x80000;
pub const AARCH64_BRANCH_ALIGN_SHIFT: u32 = 2;

pub const RISCV_JAL_OPCODE_MASK: u32 = 0x7f;
pub const RISCV_JAL_OPCODE_MATCH: u32 = 0x6f;
pub const RISCV_BRANCH_OPCODE_MATCH: u32 = 0x63;
pub const RISCV_C_J_MASK: u16 = 0xe003;
pub const RISCV_C_J_MATCH: u16 = 0xa001;
pub const RISCV_C_BEQZ_MASK: u16 = 0xe003;
pub const RISCV_C_BEQZ_MATCH: u16 = 0xc001;
pub const RISCV_C_BNEZ_MATCH: u16 = 0xe001;
pub const RISCV_BRANCH_ALIGN_SHIFT: u32 = 1;
