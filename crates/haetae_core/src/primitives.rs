#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MetaCategory {
    Declarations,
    ControlFlow,
    Iteration,
    Expressions,
    DataStructures,
    ErrorHandling,
    ConcurrencyMemory,
    Metaprogramming,
}

impl MetaCategory {
    pub fn all() -> &'static [MetaCategory] {
        &[
            MetaCategory::Declarations,
            MetaCategory::ControlFlow,
            MetaCategory::Iteration,
            MetaCategory::Expressions,
            MetaCategory::DataStructures,
            MetaCategory::ErrorHandling,
            MetaCategory::ConcurrencyMemory,
            MetaCategory::Metaprogramming,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Primitive {
    VarDecl,
    ConstDecl,
    FuncDecl,
    TypeStructDecl,
    InterfaceTrait,
    EnumDecl,
    TypeAlias,
    Namespace,
    IfElse,
    SwitchMatch,
    TernaryExpr,
    ReturnStmt,
    BreakContinue,
    GuardStmt,
    GotoLabel,
    Fallthrough,
    ForLoop,
    ForInEach,
    WhileLoop,
    DoWhileLoop,
    InfiniteLoop,
    YieldGenerator,
    ArithmeticOp,
    ComparisonOp,
    LogicalOp,
    BitwiseOp,
    AssignmentOp,
    UnaryOp,
    MemberAccess,
    CallExpr,
    NullSafetyOp,
    TypeCasting,
    ScalarLiteral,
    ArrayLiteral,
    TupleRecord,
    MapObject,
    SetLiteral,
    Comprehension,
    SliceRange,
    TryCatch,
    ThrowRaise,
    ResultOption,
    DeferWith,
    AssertStmt,
    AsyncAwait,
    ThreadTask,
    ChannelLock,
    PointerDeref,
    MemoryAlloc,
    AtomicOp,
    ImportInclude,
    ExportPublic,
    Decorator,
    MacroCodeGen,
}

impl Primitive {
    pub fn all() -> &'static [Primitive] {
        &[
            Primitive::VarDecl,
            Primitive::ConstDecl,
            Primitive::FuncDecl,
            Primitive::TypeStructDecl,
            Primitive::InterfaceTrait,
            Primitive::EnumDecl,
            Primitive::TypeAlias,
            Primitive::Namespace,
            Primitive::IfElse,
            Primitive::SwitchMatch,
            Primitive::TernaryExpr,
            Primitive::ReturnStmt,
            Primitive::BreakContinue,
            Primitive::GuardStmt,
            Primitive::GotoLabel,
            Primitive::Fallthrough,
            Primitive::ForLoop,
            Primitive::ForInEach,
            Primitive::WhileLoop,
            Primitive::DoWhileLoop,
            Primitive::InfiniteLoop,
            Primitive::YieldGenerator,
            Primitive::ArithmeticOp,
            Primitive::ComparisonOp,
            Primitive::LogicalOp,
            Primitive::BitwiseOp,
            Primitive::AssignmentOp,
            Primitive::UnaryOp,
            Primitive::MemberAccess,
            Primitive::CallExpr,
            Primitive::NullSafetyOp,
            Primitive::TypeCasting,
            Primitive::ScalarLiteral,
            Primitive::ArrayLiteral,
            Primitive::TupleRecord,
            Primitive::MapObject,
            Primitive::SetLiteral,
            Primitive::Comprehension,
            Primitive::SliceRange,
            Primitive::TryCatch,
            Primitive::ThrowRaise,
            Primitive::ResultOption,
            Primitive::DeferWith,
            Primitive::AssertStmt,
            Primitive::AsyncAwait,
            Primitive::ThreadTask,
            Primitive::ChannelLock,
            Primitive::PointerDeref,
            Primitive::MemoryAlloc,
            Primitive::AtomicOp,
            Primitive::ImportInclude,
            Primitive::ExportPublic,
            Primitive::Decorator,
            Primitive::MacroCodeGen,
        ]
    }

    pub fn meta_category(&self) -> MetaCategory {
        match self {
            Primitive::VarDecl
            | Primitive::ConstDecl
            | Primitive::FuncDecl
            | Primitive::TypeStructDecl
            | Primitive::InterfaceTrait
            | Primitive::EnumDecl
            | Primitive::TypeAlias
            | Primitive::Namespace => MetaCategory::Declarations,

            Primitive::IfElse
            | Primitive::SwitchMatch
            | Primitive::TernaryExpr
            | Primitive::ReturnStmt
            | Primitive::BreakContinue
            | Primitive::GuardStmt
            | Primitive::GotoLabel
            | Primitive::Fallthrough => MetaCategory::ControlFlow,

            Primitive::ForLoop
            | Primitive::ForInEach
            | Primitive::WhileLoop
            | Primitive::DoWhileLoop
            | Primitive::InfiniteLoop
            | Primitive::YieldGenerator => MetaCategory::Iteration,

            Primitive::ArithmeticOp
            | Primitive::ComparisonOp
            | Primitive::LogicalOp
            | Primitive::BitwiseOp
            | Primitive::AssignmentOp
            | Primitive::UnaryOp
            | Primitive::MemberAccess
            | Primitive::CallExpr
            | Primitive::NullSafetyOp
            | Primitive::TypeCasting => MetaCategory::Expressions,

            Primitive::ScalarLiteral
            | Primitive::ArrayLiteral
            | Primitive::TupleRecord
            | Primitive::MapObject
            | Primitive::SetLiteral
            | Primitive::Comprehension
            | Primitive::SliceRange => MetaCategory::DataStructures,

            Primitive::TryCatch
            | Primitive::ThrowRaise
            | Primitive::ResultOption
            | Primitive::DeferWith
            | Primitive::AssertStmt => MetaCategory::ErrorHandling,

            Primitive::AsyncAwait
            | Primitive::ThreadTask
            | Primitive::ChannelLock
            | Primitive::PointerDeref
            | Primitive::MemoryAlloc
            | Primitive::AtomicOp => MetaCategory::ConcurrencyMemory,

            Primitive::ImportInclude
            | Primitive::ExportPublic
            | Primitive::Decorator
            | Primitive::MacroCodeGen => MetaCategory::Metaprogramming,
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Primitive::VarDecl => "#01_VarDecl",
            Primitive::ConstDecl => "#02_ConstDecl",
            Primitive::FuncDecl => "#03_FuncDecl",
            Primitive::TypeStructDecl => "#04_TypeStructDecl",
            Primitive::InterfaceTrait => "#05_InterfaceTrait",
            Primitive::EnumDecl => "#06_EnumDecl",
            Primitive::TypeAlias => "#07_TypeAlias",
            Primitive::Namespace => "#08_Namespace",
            Primitive::IfElse => "#09_IfElse",
            Primitive::SwitchMatch => "#10_SwitchMatch",
            Primitive::TernaryExpr => "#11_TernaryExpr",
            Primitive::ReturnStmt => "#12_ReturnStmt",
            Primitive::BreakContinue => "#13_BreakContinue",
            Primitive::GuardStmt => "#14_GuardStmt",
            Primitive::GotoLabel => "#15_GotoLabel",
            Primitive::Fallthrough => "#16_Fallthrough",
            Primitive::ForLoop => "#17_ForLoop",
            Primitive::ForInEach => "#18_ForInEach",
            Primitive::WhileLoop => "#19_WhileLoop",
            Primitive::DoWhileLoop => "#20_DoWhileLoop",
            Primitive::InfiniteLoop => "#21_InfiniteLoop",
            Primitive::YieldGenerator => "#22_YieldGenerator",
            Primitive::ArithmeticOp => "#23_ArithmeticOp",
            Primitive::ComparisonOp => "#24_ComparisonOp",
            Primitive::LogicalOp => "#25_LogicalOp",
            Primitive::BitwiseOp => "#26_BitwiseOp",
            Primitive::AssignmentOp => "#27_AssignmentOp",
            Primitive::UnaryOp => "#28_UnaryOp",
            Primitive::MemberAccess => "#29_MemberAccess",
            Primitive::CallExpr => "#30_CallExpr",
            Primitive::NullSafetyOp => "#31_NullSafetyOp",
            Primitive::TypeCasting => "#32_TypeCasting",
            Primitive::ScalarLiteral => "#33_ScalarLiteral",
            Primitive::ArrayLiteral => "#34_ArrayLiteral",
            Primitive::TupleRecord => "#35_TupleRecord",
            Primitive::MapObject => "#36_MapObject",
            Primitive::SetLiteral => "#37_SetLiteral",
            Primitive::Comprehension => "#38_Comprehension",
            Primitive::SliceRange => "#39_SliceRange",
            Primitive::TryCatch => "#40_TryCatch",
            Primitive::ThrowRaise => "#41_ThrowRaise",
            Primitive::ResultOption => "#42_ResultOption",
            Primitive::DeferWith => "#43_DeferWith",
            Primitive::AssertStmt => "#44_AssertStmt",
            Primitive::AsyncAwait => "#45_AsyncAwait",
            Primitive::ThreadTask => "#46_ThreadTask",
            Primitive::ChannelLock => "#47_ChannelLock",
            Primitive::PointerDeref => "#48_PointerDeref",
            Primitive::MemoryAlloc => "#49_MemoryAlloc",
            Primitive::AtomicOp => "#50_AtomicOp",
            Primitive::ImportInclude => "#51_ImportInclude",
            Primitive::ExportPublic => "#52_ExportPublic",
            Primitive::Decorator => "#53_Decorator",
            Primitive::MacroCodeGen => "#54_MacroCodeGen",
        }
    }

    pub fn from_tag(tag: &str) -> Option<Primitive> {
        match tag {
            "#01_VarDecl" => Some(Primitive::VarDecl),
            "#02_ConstDecl" => Some(Primitive::ConstDecl),
            "#03_FuncDecl" => Some(Primitive::FuncDecl),
            "#04_TypeStructDecl" => Some(Primitive::TypeStructDecl),
            "#05_InterfaceTrait" => Some(Primitive::InterfaceTrait),
            "#06_EnumDecl" => Some(Primitive::EnumDecl),
            "#07_TypeAlias" => Some(Primitive::TypeAlias),
            "#08_Namespace" => Some(Primitive::Namespace),
            "#09_IfElse" => Some(Primitive::IfElse),
            "#10_SwitchMatch" => Some(Primitive::SwitchMatch),
            "#11_TernaryExpr" => Some(Primitive::TernaryExpr),
            "#12_ReturnStmt" => Some(Primitive::ReturnStmt),
            "#13_BreakContinue" => Some(Primitive::BreakContinue),
            "#14_GuardStmt" => Some(Primitive::GuardStmt),
            "#15_GotoLabel" => Some(Primitive::GotoLabel),
            "#16_Fallthrough" => Some(Primitive::Fallthrough),
            "#17_ForLoop" => Some(Primitive::ForLoop),
            "#18_ForInEach" => Some(Primitive::ForInEach),
            "#19_WhileLoop" => Some(Primitive::WhileLoop),
            "#20_DoWhileLoop" => Some(Primitive::DoWhileLoop),
            "#21_InfiniteLoop" => Some(Primitive::InfiniteLoop),
            "#22_YieldGenerator" => Some(Primitive::YieldGenerator),
            "#23_ArithmeticOp" => Some(Primitive::ArithmeticOp),
            "#24_ComparisonOp" => Some(Primitive::ComparisonOp),
            "#25_LogicalOp" => Some(Primitive::LogicalOp),
            "#26_BitwiseOp" => Some(Primitive::BitwiseOp),
            "#27_AssignmentOp" => Some(Primitive::AssignmentOp),
            "#28_UnaryOp" => Some(Primitive::UnaryOp),
            "#29_MemberAccess" => Some(Primitive::MemberAccess),
            "#30_CallExpr" => Some(Primitive::CallExpr),
            "#31_NullSafetyOp" => Some(Primitive::NullSafetyOp),
            "#32_TypeCasting" => Some(Primitive::TypeCasting),
            "#33_ScalarLiteral" => Some(Primitive::ScalarLiteral),
            "#34_ArrayLiteral" => Some(Primitive::ArrayLiteral),
            "#35_TupleRecord" => Some(Primitive::TupleRecord),
            "#36_MapObject" => Some(Primitive::MapObject),
            "#37_SetLiteral" => Some(Primitive::SetLiteral),
            "#38_Comprehension" => Some(Primitive::Comprehension),
            "#39_SliceRange" => Some(Primitive::SliceRange),
            "#40_TryCatch" => Some(Primitive::TryCatch),
            "#41_ThrowRaise" => Some(Primitive::ThrowRaise),
            "#42_ResultOption" => Some(Primitive::ResultOption),
            "#43_DeferWith" => Some(Primitive::DeferWith),
            "#44_AssertStmt" => Some(Primitive::AssertStmt),
            "#45_AsyncAwait" => Some(Primitive::AsyncAwait),
            "#46_ThreadTask" => Some(Primitive::ThreadTask),
            "#47_ChannelLock" => Some(Primitive::ChannelLock),
            "#48_PointerDeref" => Some(Primitive::PointerDeref),
            "#49_MemoryAlloc" => Some(Primitive::MemoryAlloc),
            "#50_AtomicOp" => Some(Primitive::AtomicOp),
            "#51_ImportInclude" => Some(Primitive::ImportInclude),
            "#52_ExportPublic" => Some(Primitive::ExportPublic),
            "#53_Decorator" => Some(Primitive::Decorator),
            "#54_MacroCodeGen" => Some(Primitive::MacroCodeGen),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Primitive::VarDecl => "Variable Declaration",
            Primitive::ConstDecl => "Constant Declaration",
            Primitive::FuncDecl => "Function Declaration",
            Primitive::TypeStructDecl => "Type Struct Declaration",
            Primitive::InterfaceTrait => "Interface / Trait",
            Primitive::EnumDecl => "Enum Declaration",
            Primitive::TypeAlias => "Type Alias",
            Primitive::Namespace => "Namespace / Module",
            Primitive::IfElse => "If - Else Statement",
            Primitive::SwitchMatch => "Switch - Match Statement",
            Primitive::TernaryExpr => "Ternary Expression",
            Primitive::ReturnStmt => "Return Statement",
            Primitive::BreakContinue => "Break / Continue",
            Primitive::GuardStmt => "Guard Statement",
            Primitive::GotoLabel => "Goto / Label",
            Primitive::Fallthrough => "Fallthrough Control",
            Primitive::ForLoop => "For Loop",
            Primitive::ForInEach => "For-In / For-Each",
            Primitive::WhileLoop => "While Loop",
            Primitive::DoWhileLoop => "Do-While Loop",
            Primitive::InfiniteLoop => "Infinite Loop",
            Primitive::YieldGenerator => "Yield Generator",
            Primitive::ArithmeticOp => "Arithmetic Operation",
            Primitive::ComparisonOp => "Comparison Operation",
            Primitive::LogicalOp => "Logical Operation",
            Primitive::BitwiseOp => "Bitwise Operation",
            Primitive::AssignmentOp => "Assignment Operation",
            Primitive::UnaryOp => "Unary Operation",
            Primitive::MemberAccess => "Member Access",
            Primitive::CallExpr => "Call Expression",
            Primitive::NullSafetyOp => "Null Safety Operation",
            Primitive::TypeCasting => "Type Casting",
            Primitive::ScalarLiteral => "Scalar Literal",
            Primitive::ArrayLiteral => "Array Literal",
            Primitive::TupleRecord => "Tuple / Record",
            Primitive::MapObject => "Map / Object Literal",
            Primitive::SetLiteral => "Set Literal",
            Primitive::Comprehension => "Comprehension",
            Primitive::SliceRange => "Slice / Range",
            Primitive::TryCatch => "Try - Catch Block",
            Primitive::ThrowRaise => "Throw / Raise",
            Primitive::ResultOption => "Result / Option",
            Primitive::DeferWith => "Defer / With",
            Primitive::AssertStmt => "Assert Statement",
            Primitive::AsyncAwait => "Async / Await",
            Primitive::ThreadTask => "Thread / Task",
            Primitive::ChannelLock => "Channel / Lock",
            Primitive::PointerDeref => "Pointer Dereference",
            Primitive::MemoryAlloc => "Memory Allocation",
            Primitive::AtomicOp => "Atomic Operation",
            Primitive::ImportInclude => "Import / Include",
            Primitive::ExportPublic => "Export / Public",
            Primitive::Decorator => "Decorator / Attribute",
            Primitive::MacroCodeGen => "Macro / CodeGen",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TargetLanguage {
    JavaScript,
    TypeScript,
    HtmlCss,
    Python,
    Java,
    Go,
    CSharp,
    Php,
    Swift,
    Kotlin,
    Cpp,
    C,
    Rust,
    Sql,
    Bash,
}

impl TargetLanguage {
    pub fn all() -> &'static [TargetLanguage] {
        &[
            TargetLanguage::JavaScript,
            TargetLanguage::TypeScript,
            TargetLanguage::HtmlCss,
            TargetLanguage::Python,
            TargetLanguage::Java,
            TargetLanguage::Go,
            TargetLanguage::CSharp,
            TargetLanguage::Php,
            TargetLanguage::Swift,
            TargetLanguage::Kotlin,
            TargetLanguage::Cpp,
            TargetLanguage::C,
            TargetLanguage::Rust,
            TargetLanguage::Sql,
            TargetLanguage::Bash,
        ]
    }

    pub fn from_identifier(id: &str) -> Self {
        let lower = id.to_lowercase();
        if lower.ends_with(".py") || lower == "python" {
            TargetLanguage::Python
        } else if lower.ends_with(".rs") || lower == "rust" {
            TargetLanguage::Rust
        } else if lower.ends_with(".js") || lower == "javascript" {
            TargetLanguage::JavaScript
        } else if lower.ends_with(".ts") || lower == "typescript" {
            TargetLanguage::TypeScript
        } else if lower.ends_with(".c") || lower == "c" {
            TargetLanguage::C
        } else if lower.ends_with(".cpp")
            || lower.ends_with(".cc")
            || lower.ends_with(".cxx")
            || lower == "cpp"
            || lower == "c++"
        {
            TargetLanguage::Cpp
        } else if lower.ends_with(".go") || lower == "go" {
            TargetLanguage::Go
        } else if lower.ends_with(".java") || lower == "java" {
            TargetLanguage::Java
        } else if lower.ends_with(".cs") || lower == "csharp" || lower == "c#" {
            TargetLanguage::CSharp
        } else if lower.ends_with(".swift") || lower == "swift" {
            TargetLanguage::Swift
        } else if lower.ends_with(".kt") || lower.ends_with(".kts") || lower == "kotlin" {
            TargetLanguage::Kotlin
        } else if lower.ends_with(".php") || lower == "php" {
            TargetLanguage::Php
        } else if lower.ends_with(".sql") || lower == "sql" {
            TargetLanguage::Sql
        } else if lower.ends_with(".sh")
            || lower.ends_with(".bash")
            || lower == "bash"
            || lower == "shellscript"
        {
            TargetLanguage::Bash
        } else if lower.ends_with(".html")
            || lower.ends_with(".css")
            || lower == "html"
            || lower == "css"
        {
            TargetLanguage::HtmlCss
        } else {
            TargetLanguage::C
        }
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            TargetLanguage::JavaScript => "javascript",
            TargetLanguage::TypeScript => "typescript",
            TargetLanguage::HtmlCss => "html",
            TargetLanguage::Python => "python",
            TargetLanguage::Java => "java",
            TargetLanguage::Go => "go",
            TargetLanguage::CSharp => "csharp",
            TargetLanguage::Php => "php",
            TargetLanguage::Swift => "swift",
            TargetLanguage::Kotlin => "kotlin",
            TargetLanguage::Cpp => "cpp",
            TargetLanguage::C => "c",
            TargetLanguage::Rust => "rust",
            TargetLanguage::Sql => "sql",
            TargetLanguage::Bash => "bash",
        }
    }

    pub fn synthesize_code_fim(&self, prompt: &str, prefix: &str, suffix: &str) -> String {
        let full = self.synthesize_code(prompt);
        let trimmed_pre = prefix.trim_end();
        let trimmed_suf = suffix.trim_start();

        let needs_inner = (trimmed_pre.ends_with('{') && trimmed_suf.starts_with('}'))
            || trimmed_pre.ends_with('{')
            || (*self == TargetLanguage::Python && trimmed_pre.ends_with(':'));

        if !needs_inner {
            return full;
        }

        if *self == TargetLanguage::Python {
            let mut body_lines = Vec::new();
            let mut found_header = false;
            for line in full.lines() {
                if line.trim_start().starts_with("def ") || line.trim_start().starts_with("class ")
                {
                    found_header = true;
                    continue;
                }
                if found_header && !line.trim().is_empty() {
                    body_lines.push(line);
                }
            }
            if !body_lines.is_empty() {
                return body_lines.join("\n");
            }
        } else {
            if let Some(open_pos) = full.rfind('{') {
                if let Some(close_pos) = full[open_pos..].rfind('}') {
                    let inner = &full[open_pos + 1..open_pos + close_pos];
                    if !inner.trim().is_empty() {
                        return inner.to_string();
                    }
                }
            }
        }

        full
    }

    pub fn syntax_sample(&self, prim: Primitive) -> &'static str {
        crate::syntax_matrix::get_syntax_sample(*self, prim)
    }

    pub fn synthesize_code(&self, prompt: &str) -> String {
        let p_lower = prompt.to_lowercase();
        let is_trie = p_lower.contains("trie")
            || p_lower.contains("트라이")
            || p_lower.contains("prefix tree")
            || p_lower.contains("접두사");
        let is_segment_tree = p_lower.contains("segment")
            || p_lower.contains("세그먼트")
            || p_lower.contains("segtree")
            || p_lower.contains("range sum");
        let is_dijkstra = p_lower.contains("dijkstra")
            || p_lower.contains("다익스트라")
            || p_lower.contains("shortest path")
            || p_lower.contains("최단 경로");
        let is_lock_free = p_lower.contains("lock free")
            || p_lower.contains("lock-free")
            || p_lower.contains("락프리")
            || p_lower.contains("cas")
            || p_lower.contains("atomic stack")
            || p_lower.contains("treiber");
        let is_crc32 = p_lower.contains("crc32")
            || p_lower.contains("crc")
            || p_lower.contains("checksum")
            || p_lower.contains("체크섬");
        let is_rate_limiter = p_lower.contains("token bucket")
            || p_lower.contains("rate limit")
            || p_lower.contains("토큰 버킷")
            || p_lower.contains("속도 제한")
            || p_lower.contains("leaky bucket");
        let is_simd = p_lower.contains("simd")
            || p_lower.contains("vectorized")
            || p_lower.contains("dot product")
            || p_lower.contains("내적");
        let is_ffi_mixed = (p_lower.contains("ffi")
            || p_lower.contains("binding")
            || p_lower.contains("extern c")
            || p_lower.contains("bridge")
            || p_lower.contains("바인딩")
            || p_lower.contains("mixed")
            || p_lower.contains("다중 언어"))
            && !p_lower.contains("socket")
            && !p_lower.contains("tcp");
        let is_tree = p_lower.contains("tree")
            || p_lower.contains("트리")
            || p_lower.contains("bst")
            || p_lower.contains("inorder")
            || p_lower.contains("순회")
            || p_lower.contains("traversal")
            || p_lower.contains("dfs")
            || p_lower.contains("bfs");
        let is_dp = p_lower.contains("dp")
            || p_lower.contains("동적")
            || p_lower.contains("dynamic programming")
            || p_lower.contains("knapsack")
            || p_lower.contains("배낭")
            || p_lower.split_whitespace().any(|w| w == "lis" || w == "lcs")
            || p_lower.contains("fibonacci")
            || p_lower.contains("피보나치");
        let is_hash = p_lower.contains("hash")
            || p_lower.contains("해시")
            || p_lower.contains("lru")
            || p_lower.contains("cache")
            || p_lower.contains("캐시")
            || p_lower.contains("table")
            || p_lower.contains("collision");
        let is_concurrency = p_lower.contains("concurren")
            || p_lower.contains("동시")
            || p_lower.contains("thread")
            || p_lower.contains("스레드")
            || p_lower.contains("worker")
            || p_lower.contains("워커")
            || p_lower.contains("channel")
            || p_lower.contains("채널")
            || p_lower.contains("async")
            || p_lower.contains("비동기")
            || p_lower.contains("mutex")
            || p_lower.contains("뮤텍스")
            || p_lower.contains("lock")
            || p_lower.contains("parallel");
        let is_branchless = p_lower.contains("branchless")
            || p_lower.contains("브랜치리스")
            || p_lower.contains("비트")
            || p_lower.contains("bitwise")
            || p_lower.contains("mask")
            || p_lower.contains("zero alloc")
            || p_lower.contains("무할당")
            || p_lower.contains("optimize");
        let is_divide = p_lower.contains("나눗셈")
            || p_lower.contains("divide")
            || p_lower.contains("division")
            || p_lower.contains("불변식")
            || p_lower.contains("invariant")
            || p_lower.contains("robust")
            || p_lower.contains("견고")
            || p_lower.contains("overflow")
            || p_lower.contains("sanitize");
        let is_socket = p_lower.contains("소켓")
            || p_lower.contains("socket")
            || p_lower.contains("tcp")
            || p_lower.contains("네트워크")
            || p_lower.contains("network")
            || p_lower.contains("http")
            || p_lower.contains("retry")
            || p_lower.contains("pipeline")
            || p_lower.contains("파이프라인");
        let is_sort = p_lower.contains("sort")
            || p_lower.contains("정렬")
            || p_lower.contains("search")
            || p_lower.contains("탐색")
            || p_lower.contains("binary search");
        let is_random = p_lower.contains("랜덤")
            || p_lower.contains("random")
            || p_lower.contains("난수")
            || p_lower.contains("1에서 100")
            || p_lower.contains("1 to 100");
        let is_list = p_lower.contains("리스트")
            || p_lower.contains("배열")
            || p_lower.contains("list")
            || p_lower.contains("array");

        match self {
            TargetLanguage::Python => {
                if is_trie {
                    "class TrieNode:\n    def __init__(self):\n        self.children = {}\n        self.is_end = False\n\nclass Trie:\n    def __init__(self):\n        self.root = TrieNode()\n    def insert(self, word: str) -> None:\n        curr = self.root\n        for ch in word:\n            if ch not in curr.children:\n                curr.children[ch] = TrieNode()\n            curr = curr.children[ch]\n        curr.is_end = True\n    def search(self, word: str) -> bool:\n        curr = self.root\n        for ch in word:\n            if ch not in curr.children:\n                return False\n            curr = curr.children[ch]\n        return curr.is_end\n".to_string()
                } else if is_segment_tree {
                    "from typing import List\n\nclass SegmentTree:\n    def __init__(self, data: List[int]):\n        self.n = len(data)\n        self.tree = [0] * (4 * self.n)\n        if self.n > 0:\n            self.build(data, 1, 0, self.n - 1)\n    def build(self, data: List[int], node: int, start: int, end: int) -> None:\n        if start == end:\n            self.tree[node] = data[start]\n            return\n        mid = (start + end) // 2\n        self.build(data, 2 * node, start, mid)\n        self.build(data, 2 * node + 1, mid + 1, end)\n        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1]\n".to_string()
                } else if is_dijkstra {
                    "import heapq\nfrom typing import List, Tuple, Dict\n\ndef dijkstra(n: int, edges: List[Tuple[int, int, int]], start: int) -> Dict[int, int]:\n    adj = {i: [] for i in range(n)}\n    for u, v, w in edges:\n        adj[u].append((v, w))\n    dist = {i: float('inf') for i in range(n)}\n    dist[start] = 0\n    pq = [(0, start)]\n    while pq:\n        d, u = heapq.heappop(pq)\n        if d > dist[u]:\n            continue\n        for v, w in adj[u]:\n            if dist[u] + w < dist[v]:\n                dist[v] = dist[u] + w\n                heapq.heappush(pq, (dist[v], v))\n    return dist\n".to_string()
                } else if is_lock_free {
                    "import threading\nfrom typing import Optional, Any\n\nclass AtomicStack:\n    def __init__(self):\n        self._head = None\n        self._lock = threading.Lock()\n    def push(self, val: Any) -> None:\n        with self._lock:\n            self._head = (val, self._head)\n    def pop(self) -> Optional[Any]:\n        with self._lock:\n            if self._head is None:\n                return None\n            val, self._head = self._head\n            return val\n".to_string()
                } else if is_crc32 {
                    "import zlib\n\ndef crc32_compute(data: bytes) -> int:\n    return zlib.crc32(data) & 0xFFFFFFFF\n".to_string()
                } else if is_rate_limiter {
                    "import time\n\nclass TokenBucket:\n    def __init__(self, capacity: int, refill_rate: float):\n        self.capacity = capacity\n        self.tokens = capacity\n        self.refill_rate = refill_rate\n        self.last_update = time.time()\n    def consume(self, cost: int = 1) -> bool:\n        now = time.time()\n        self.tokens = min(self.capacity, self.tokens + (now - self.last_update) * self.refill_rate)\n        self.last_update = now\n        if self.tokens >= cost:\n            self.tokens -= cost\n            return True\n        return False\n".to_string()
                } else if is_simd {
                    "from typing import List\n\ndef dot_product_simd(a: List[float], b: List[float]) -> float:\n    return sum(x * y for x, y in zip(a, b))\n".to_string()
                } else if is_ffi_mixed {
                    "import ctypes\n\ndef call_native_bridge(lib_path: str, data: bytes) -> int:\n    lib = ctypes.CDLL(lib_path)\n    return lib.haetae_ffi_bridge(data, len(data))\n".to_string()
                } else if is_tree {
                    "from typing import Optional, List\n\nclass TreeNode:\n    def __init__(self, val: int = 0, left: Optional['TreeNode'] = None, right: Optional['TreeNode'] = None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef inorder_traversal(root: Optional[TreeNode]) -> List[int]:\n    result: List[int] = []\n    def traverse(node: Optional[TreeNode]) -> None:\n        if not node:\n            return\n        traverse(node.left)\n        result.append(node.val)\n        traverse(node.right)\n    traverse(root)\n    return result\n".to_string()
                } else if is_dp {
                    "from typing import List\n\ndef knapsack_01(weights: List[int], values: List[int], capacity: int) -> int:\n    n = len(weights)\n    dp = [[0] * (capacity + 1) for _ in range(n + 1)]\n    for i in range(1, n + 1):\n        for w in range(1, capacity + 1):\n            if weights[i - 1] <= w:\n                dp[i][w] = max(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1])\n            else:\n                dp[i][w] = dp[i - 1][w]\n    return dp[n][capacity]\n".to_string()
                } else if is_hash {
                    "from collections import OrderedDict\nfrom typing import Any, Optional\n\nclass LRUCache:\n    def __init__(self, capacity: int):\n        self.capacity = capacity\n        self.cache: OrderedDict[str, Any] = OrderedDict()\n\n    def get(self, key: str) -> Optional[Any]:\n        if key not in self.cache:\n            return None\n        self.cache.move_to_end(key)\n        return self.cache[key]\n\n    def put(self, key: str, value: Any) -> None:\n        if key in self.cache:\n            self.cache.move_to_end(key)\n        self.cache[key] = value\n        if len(self.cache) > self.capacity:\n            self.cache.popitem(last=False)\n".to_string()
                } else if is_concurrency {
                    "import asyncio\nfrom typing import List, Callable, Any\n\nasync def run_worker_pool(tasks: List[Callable[[], Any]], max_concurrency: int) -> List[Any]:\n    semaphore = asyncio.Semaphore(max_concurrency)\n    async def sem_coro(task: Callable[[], Any]) -> Any:\n        async with semaphore:\n            return await task()\n    return await asyncio.gather(*(sem_coro(t) for t in tasks))\n".to_string()
                } else if is_branchless {
                    "def branchless_select(mask: int, a: int, b: int) -> int:\n    return (a & mask) | (b & ~mask)\n".to_string()
                } else if is_divide {
                    "def divide_scalar(base_val: int, factor: int) -> int:\n    if factor == 0:\n        return 0\n    return int(base_val / factor)\n".to_string()
                } else if is_socket {
                    "import socket\nfrom typing import Optional\n\ndef connect_with_retry(host: str, port: int, retries: int) -> Optional[socket.socket]:\n    for _ in range(retries):\n        try:\n            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)\n            s.connect((host, port))\n            return s\n        except OSError:\n            pass\n    return None\n".to_string()
                } else if is_sort {
                    "from typing import List\n\ndef binary_search(arr: List[int], target: int) -> int:\n    left, right = 0, len(arr) - 1\n    while left <= right:\n        mid = (left + right) // 2\n        if arr[mid] == target:\n            return mid\n        elif arr[mid] < target:\n            left = mid + 1\n        else:\n            right = mid - 1\n    return -1\n".to_string()
                } else if is_random {
                    "import random\n\nfor i in range(0, 1):\n    print(f\"{i}: {random.randint(0, 1)}\")\n".to_string()
                } else if is_list {
                    "items = [i for i in range(0, 1)]\nprint(items)\n".to_string()
                } else {
                    format!(
                        "def execute_action():\n    print(\"Haetae executed: {}\")\n",
                        prompt
                    )
                }
            }
            TargetLanguage::Rust => {
                if is_trie {
                    "use std::collections::HashMap;\n\n#[derive(Default)]\npub struct TrieNode {\n    pub children: HashMap<char, TrieNode>,\n    pub is_end: bool,\n}\n\nimpl TrieNode {\n    pub fn new() -> Self {\n        Self::default()\n    }\n    pub fn insert(&mut self, word: &str) {\n        let mut curr = self;\n        for ch in word.chars() {\n            curr = curr.children.entry(ch).or_default();\n        }\n        curr.is_end = true;\n    }\n    pub fn search(&self, word: &str) -> bool {\n        let mut curr = self;\n        for ch in word.chars() {\n            match curr.children.get(&ch) {\n                Some(next) => curr = next,\n                None => return false,\n            }\n        }\n        curr.is_end\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "pub struct SegmentTree {\n    pub tree: Vec<i64>,\n    pub n: usize,\n}\n\nimpl SegmentTree {\n    pub fn new(data: &[i64]) -> Self {\n        let n = data.len();\n        let mut tree = vec![0i64; n * 4];\n        let mut st = Self { tree, n };\n        if n > 0 {\n            st.build(data, 1, 0, n - 1);\n        }\n        st\n    }\n    fn build(&mut self, data: &[i64], node: usize, l: usize, r: usize) {\n        if l == r {\n            self.tree[node] = data[l];\n            return;\n        }\n        let mid = l + (r - l) / 2;\n        self.build(data, 2 * node, l, mid);\n        self.build(data, 2 * node + 1, mid + 1, r);\n        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "use std::cmp::Ordering;\nuse std::collections::BinaryHeap;\n\n#[derive(Copy, Clone, Eq, PartialEq)]\nstruct State {\n    cost: u64,\n    position: usize,\n}\n\nimpl Ord for State {\n    fn cmp(&self, other: &Self) -> Ordering {\n        other.cost.cmp(&self.cost)\n    }\n}\n\nimpl PartialOrd for State {\n    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\n        Some(self.cmp(other))\n    }\n}\n\npub fn dijkstra(n: usize, adj: &[Vec<(usize, u64)>], start: usize) -> Vec<u64> {\n    let mut dist = vec![u64::MAX; n];\n    let mut heap = BinaryHeap::new();\n    dist[start] = 0;\n    heap.push(State { cost: 0, position: start });\n    while let Some(State { cost, position }) = heap.pop() {\n        if cost > dist[position] {\n            continue;\n        }\n        for &(next, edge_cost) in &adj[position] {\n            let next_cost = cost + edge_cost;\n            if next_cost < dist[next] {\n                dist[next] = next_cost;\n                heap.push(State { cost: next_cost, position: next });\n            }\n        }\n    }\n    dist\n}\n".to_string()
                } else if is_lock_free {
                    "use std::sync::atomic::{AtomicPtr, Ordering};\n\npub struct Node<T> {\n    pub val: T,\n    pub next: *mut Node<T>,\n}\n\npub struct LockFreeStack<T> {\n    head: AtomicPtr<Node<T>>,\n}\n\nimpl<T> LockFreeStack<T> {\n    pub fn new() -> Self {\n        Self { head: AtomicPtr::new(std::ptr::null_mut()) }\n    }\n    pub fn push(&self, val: T) {\n        let new_node = Box::into_raw(Box::new(Node { val, next: std::ptr::null_mut() }));\n        let mut cur = self.head.load(Ordering::Relaxed);\n        loop {\n            unsafe { (*new_node).next = cur; }\n            match self.head.compare_exchange_weak(cur, new_node, Ordering::Release, Ordering::Relaxed) {\n                Ok(_) => break,\n                Err(actual) => cur = actual,\n            }\n        }\n    }\n}\n".to_string()
                } else if is_crc32 {
                    "pub fn crc32_compute(data: &[u8]) -> u32 {\n    let mut crc = 0xFFFFFFFFu32;\n    for &b in data {\n        crc ^= b as u32;\n        for _ in 0..8 {\n            crc = if crc & 1 != 0 {\n                (crc >> 1) ^ 0xEDB88320\n            } else {\n                crc >> 1\n            };\n        }\n    }\n    !crc\n}\n".to_string()
                } else if is_rate_limiter {
                    "pub struct TokenBucket {\n    pub capacity: u64,\n    pub tokens: u64,\n}\n\nimpl TokenBucket {\n    pub fn new(capacity: u64) -> Self {\n        Self { capacity, tokens: capacity }\n    }\n    pub fn consume(&mut self, count: u64) -> bool {\n        if self.tokens >= count {\n            self.tokens -= count;\n            true\n        } else {\n            false\n        }\n    }\n}\n".to_string()
                } else if is_simd {
                    "pub fn dot_product_simd(a: &[f32], b: &[f32]) -> f32 {\n    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()\n}\n".to_string()
                } else if is_ffi_mixed {
                    "#[no_mangle]\npub extern \"C\" fn haetae_ffi_bridge(ptr: *const u8, len: usize) -> i32 {\n    if ptr.is_null() {\n        return 0;\n    }\n    len as i32\n}\n".to_string()
                } else if is_tree {
                    "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct TreeNode {\n    pub val: i32,\n    pub left: Option<Box<TreeNode>>,\n    pub right: Option<Box<TreeNode>>,\n}\n\nimpl TreeNode {\n    pub fn new(val: i32) -> Self {\n        Self { val, left: None, right: None }\n    }\n}\n\npub fn inorder_traversal(root: &Option<Box<TreeNode>>) -> Vec<i32> {\n    let mut res = Vec::new();\n    fn dfs(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {\n        if let Some(n) = node {\n            dfs(&n.left, out);\n            out.push(n.val);\n            dfs(&n.right, out);\n        }\n    }\n    dfs(root, &mut res);\n    res\n}\n".to_string()
                } else if is_dp {
                    "pub fn knapsack_01(weights: &[usize], values: &[u64], capacity: usize) -> u64 {\n    let n = weights.len();\n    let mut dp = vec![vec![0u64; capacity + 1]; n + 1];\n    for i in 1..=n {\n        for w in 1..=capacity {\n            if weights[i - 1] <= w {\n                dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - weights[i - 1]] + values[i - 1]);\n            } else {\n                dp[i][w] = dp[i - 1][w];\n            }\n        }\n    }\n    dp[n][capacity]\n}\n".to_string()
                } else if is_hash {
                    "use std::collections::HashMap;\n\npub struct SimpleCache<K, V> {\n    entries: HashMap<K, V>,\n    capacity: usize,\n}\n\nimpl<K: std::hash::Hash + Eq + Clone, V: Clone> SimpleCache<K, V> {\n    pub fn new(capacity: usize) -> Self {\n        Self { entries: HashMap::new(), capacity }\n    }\n    pub fn get(&self, key: &K) -> Option<&V> {\n        self.entries.get(key)\n    }\n    pub fn put(&mut self, key: K, val: V) {\n        if self.entries.len() >= self.capacity {\n            if let Some(first_key) = self.entries.keys().next().cloned() {\n                self.entries.remove(&first_key);\n            }\n        }\n        self.entries.insert(key, val);\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "use std::sync::{Arc, Mutex};\nuse std::thread;\n\npub fn parallel_execute<T: Send + 'static, R: Send + 'static, F: Fn(T) -> R + Sync + Send + 'static>(\n    items: Vec<T>,\n    worker_count: usize,\n    op: F,\n) -> Vec<R> {\n    let queue = Arc::new(Mutex::new(items));\n    let results = Arc::new(Mutex::new(Vec::new()));\n    let op_arc = Arc::new(op);\n    let mut handles = Vec::new();\n    for _ in 0..worker_count {\n        let q = Arc::clone(&queue);\n        let res = Arc::clone(&results);\n        let op_clone = Arc::clone(&op_arc);\n        handles.push(thread::spawn(move || loop {\n            let item = { match q.lock().unwrap().pop() { Some(x) => x, None => break } };\n            let val = op_clone(item);\n            res.lock().unwrap().push(val);\n        }));\n    }\n    for h in handles { let _ = h.join(); }\n    let mut final_res = results.lock().unwrap();\n    std::mem::take(&mut *final_res)\n}\n".to_string()
                } else if is_branchless {
                    "pub fn branchless_select(mask: u64, a: u64, b: u64) -> u64 {\n    (a & mask) | (b & !mask)\n}\n".to_string()
                } else if is_divide {
                    "pub fn divide_scalar(base_val: i32, factor: i32) -> Result<i32, &'static str> {\n    if factor == 0 {\n        return Err(\"division by zero\");\n    }\n    Ok(base_val / factor)\n}\n".to_string()
                } else if is_socket {
                    "use std::net::TcpStream;\nuse std::time::Duration;\n\npub fn connect_with_retry(addr: &str, retries: u32, timeout_ms: u64) -> Result<TcpStream, std::io::Error> {\n    let mut last_err = std::io::Error::new(std::io::ErrorKind::NotConnected, \"connection failed\");\n    for _ in 0..retries {\n        match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(timeout_ms)) {\n            Ok(stream) => return Ok(stream),\n            Err(e) => last_err = e,\n        }\n    }\n    Err(last_err)\n}\n".to_string()
                } else if is_sort {
                    "pub fn binary_search(slice: &[i32], target: i32) -> Option<usize> {\n    let mut low = 0;\n    let mut high = slice.len();\n    while low < high {\n        let mid = low + (high - low) / 2;\n        if slice[mid] == target {\n            return Some(mid);\n        } else if slice[mid] < target {\n            low = mid + 1;\n        } else {\n            high = mid;\n        }\n    }\n    None\n}\n".to_string()
                } else if is_random {
                    "fn main() {\n    for i in 0..1 {\n        println!(\"{}\", i);\n    }\n}\n"
                        .to_string()
                } else if is_list {
                    "let items: Vec<i32> = (0..1).collect();\nprintln!(\"{:?}\", items);\n"
                        .to_string()
                } else {
                    format!(
                        "pub fn execute_action() {{\n    println!(\"Haetae executed: {}\");\n}}\n",
                        prompt
                    )
                }
            }
            TargetLanguage::C => {
                if is_trie {
                    "#include <stdlib.h>\n\ntypedef struct TrieNode {\n    struct TrieNode* children[26];\n    int is_end;\n} TrieNode;\n\nTrieNode* trie_create(void) {\n    TrieNode* node = (TrieNode*)calloc(1, sizeof(TrieNode));\n    return node;\n}\n\nvoid trie_insert(TrieNode* root, const char* word) {\n    TrieNode* curr = root;\n    while (*word) {\n        int idx = *word - 'a';\n        if (idx >= 0 && idx < 26) {\n            if (!curr->children[idx]) curr->children[idx] = trie_create();\n            curr = curr->children[idx];\n        }\n        word++;\n    }\n    curr->is_end = 1;\n}\n".to_string()
                } else if is_segment_tree {
                    "void segtree_build(int* tree, const int* arr, int node, int start, int end) {\n    if (start == end) {\n        tree[node] = arr[start];\n        return;\n    }\n    int mid = start + (end - start) / 2;\n    segtree_build(tree, arr, 2 * node, start, mid);\n    segtree_build(tree, arr, 2 * node + 1, mid + 1, end);\n    tree[node] = tree[2 * node] + tree[2 * node + 1];\n}\n".to_string()
                } else if is_dijkstra {
                    "void dijkstra_shortest_path(int n, const int graph[10][10], int src, int dist[10]) {\n    int visited[10] = {0};\n    for (int i = 0; i < n; i++) dist[i] = 1000000;\n    dist[src] = 0;\n    for (int count = 0; count < n - 1; count++) {\n        int min_d = 1000000, u = -1;\n        for (int v = 0; v < n; v++) {\n            if (!visited[v] && dist[v] <= min_d) { min_d = dist[v]; u = v; }\n        }\n        if (u < 0) break;\n        visited[u] = 1;\n        for (int v = 0; v < n; v++) {\n            if (!visited[v] && graph[u][v] && dist[u] + graph[u][v] < dist[v]) {\n                dist[v] = dist[u] + graph[u][v];\n            }\n        }\n    }\n}\n".to_string()
                } else if is_lock_free {
                    "typedef struct AtomicNode {\n    int val;\n    struct AtomicNode* next;\n} AtomicNode;\n\ntypedef struct AtomicStack {\n    AtomicNode* head;\n} AtomicStack;\n\nvoid atomic_stack_push(AtomicStack* stack, AtomicNode* node) {\n    AtomicNode* old_head = stack->head;\n    do {\n        node->next = old_head;\n        old_head = __sync_val_compare_and_swap(&stack->head, old_head, node);\n    } while (node->next != old_head);\n}\n".to_string()
                } else if is_crc32 {
                    "unsigned int crc32_compute(const unsigned char* data, int len) {\n    unsigned int crc = 0xFFFFFFFF;\n    for (int i = 0; i < len; i++) {\n        crc ^= data[i];\n        for (int j = 0; j < 8; j++) {\n            crc = (crc & 1) ? ((crc >> 1) ^ 0xEDB88320) : (crc >> 1);\n        }\n    }\n    return ~crc;\n}\n".to_string()
                } else if is_rate_limiter {
                    "typedef struct TokenBucket {\n    int capacity;\n    int tokens;\n} TokenBucket;\n\nint token_bucket_consume(TokenBucket* tb, int cost) {\n    if (tb->tokens >= cost) {\n        tb->tokens -= cost;\n        return 1;\n    }\n    return 0;\n}\n".to_string()
                } else if is_simd {
                    "float dot_product_simd(const float* a, const float* b, int n) {\n    float sum = 0.0f;\n    for (int i = 0; i < n; i++) sum += a[i] * b[i];\n    return sum;\n}\n".to_string()
                } else if is_ffi_mixed {
                    "int haetae_ffi_bridge(const unsigned char* ptr, int len) {\n    if (!ptr) return 0;\n    return len;\n}\n".to_string()
                } else if is_tree {
                    "#include <stdlib.h>\n\ntypedef struct TreeNode {\n    int val;\n    struct TreeNode* left;\n    struct TreeNode* right;\n} TreeNode;\n\nvoid inorder_traversal(const TreeNode* root, int* buffer, int* count) {\n    if (!root) return;\n    inorder_traversal(root->left, buffer, count);\n    buffer[(*count)++] = root->val;\n    inorder_traversal(root->right, buffer, count);\n}\n".to_string()
                } else if is_dp {
                    "int knapsack_01(const int* weights, const int* values, int n, int capacity) {\n    int dp[101][101] = {0};\n    for (int i = 1; i <= n; i++) {\n        for (int w = 1; w <= capacity; w++) {\n            if (weights[i - 1] <= w) {\n                int incl = dp[i - 1][w - weights[i - 1]] + values[i - 1];\n                int excl = dp[i - 1][w];\n                dp[i][w] = (incl > excl) ? incl : excl;\n            } else {\n                dp[i][w] = dp[i - 1][w];\n            }\n        }\n    }\n    return dp[n][capacity];\n}\n".to_string()
                } else if is_hash {
                    "#include <string.h>\n\ntypedef struct HashEntry {\n    char key[64];\n    int val;\n    int used;\n} HashEntry;\n\ntypedef struct SimpleHashTable {\n    HashEntry entries[256];\n} SimpleHashTable;\n\nvoid hash_put(SimpleHashTable* table, const char* key, int val) {\n    unsigned int idx = 0;\n    const char* p = key;\n    while (*p) { idx = (idx * 31) + (unsigned char)(*p++); }\n    idx = idx % 256;\n    for (int i = 0; i < 256; i++) {\n        unsigned int cur = (idx + i) % 256;\n        if (!table->entries[cur].used || strcmp(table->entries[cur].key, key) == 0) {\n            strncpy(table->entries[cur].key, key, 63);\n            table->entries[cur].val = val;\n            table->entries[cur].used = 1;\n            return;\n        }\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "#include <pthread.h>\n\ntypedef struct WorkerQueue {\n    pthread_mutex_t lock;\n    pthread_cond_t notify;\n    int count;\n} WorkerQueue;\n\nvoid queue_init(WorkerQueue* q) {\n    pthread_mutex_init(&q->lock, NULL);\n    pthread_cond_init(&q->notify, NULL);\n    q->count = 0;\n}\n".to_string()
                } else if is_branchless {
                    "int branchless_select(int mask, int a, int b) {\n    return (a & mask) | (b & ~mask);\n}\n".to_string()
                } else if is_divide {
                    "int divide_scalar(int base_val, int factor) {\n    if (factor == 0) return 0;\n    return base_val / factor;\n}\n".to_string()
                } else if is_socket {
                    "#include <sys/socket.h>\n#include <netinet/in.h>\n#include <arpa/inet.h>\n#include <unistd.h>\n#include <string.h>\n\nint connect_tcp_retry(const char* ip, int port, int max_retries) {\n    for (int i = 0; i < max_retries; i++) {\n        int sock = socket(AF_INET, SOCK_STREAM, 0);\n        if (sock < 0) continue;\n        struct sockaddr_in addr;\n        memset(&addr, 0, sizeof(addr));\n        addr.sin_family = AF_INET;\n        addr.sin_port = htons(port);\n        inet_pton(AF_INET, ip, &addr.sin_addr);\n        if (connect(sock, (struct sockaddr*)&addr, sizeof(addr)) == 0) return sock;\n        close(sock);\n    }\n    return -1;\n}\n".to_string()
                } else if is_sort {
                    "int binary_search(const int* arr, int size, int target) {\n    int low = 0, high = size - 1;\n    while (low <= high) {\n        int mid = low + (high - low) / 2;\n        if (arr[mid] == target) return mid;\n        else if (arr[mid] < target) low = mid + 1;\n        else high = mid - 1;\n    }\n    return -1;\n}\n".to_string()
                } else if is_random {
                    "#include <stdio.h>\n#include <stdlib.h>\n\nint main(void) {\n    for (int i = 0; i < 1; i++) {\n        printf(\"%d\\n\", rand() & 1);\n    }\n    return 0;\n}\n".to_string()
                } else if is_list {
                    "int items[] = {0, 1};\n".to_string()
                } else {
                    format!(
                        "void execute_action(void) {{\n    printf(\"Haetae: {}\\n\");\n}}\n",
                        prompt
                    )
                }
            }
            TargetLanguage::Cpp => {
                if is_trie {
                    "#include <unordered_map>\n#include <memory>\n#include <string>\n\nclass Trie {\n    struct Node {\n        std::unordered_map<char, std::unique_ptr<Node>> children;\n        bool is_end = false;\n    };\n    std::unique_ptr<Node> root_;\npublic:\n    Trie() : root_(std::make_unique<Node>()) {}\n    void insert(const std::string& word) {\n        auto* curr = root_.get();\n        for (char ch : word) {\n            if (!curr->children.count(ch)) curr->children[ch] = std::make_unique<Node>();\n            curr = curr->children[ch].get();\n        }\n        curr->is_end = true;\n    }\n    bool search(const std::string& word) const {\n        const auto* curr = root_.get();\n        for (char ch : word) {\n            auto it = curr->children.find(ch);\n            if (it == curr->children.end()) return false;\n            curr = it->second.get();\n        }\n        return curr->is_end;\n    }\n};\n".to_string()
                } else if is_segment_tree {
                    "#include <vector>\n\nclass SegmentTree {\n    std::vector<int> tree_;\n    int n_;\npublic:\n    explicit SegmentTree(const std::vector<int>& data) : n_(static_cast<int>(data.size())), tree_(4 * data.size(), 0) {\n        if (n_ > 0) build(data, 1, 0, n_ - 1);\n    }\n    void build(const std::vector<int>& data, int node, int start, int end) {\n        if (start == end) { tree_[node] = data[start]; return; }\n        int mid = start + (end - start) / 2;\n        build(data, 2 * node, start, mid);\n        build(data, 2 * node + 1, mid + 1, end);\n        tree_[node] = tree_[2 * node] + tree_[2 * node + 1];\n    }\n};\n".to_string()
                } else if is_dijkstra {
                    "#include <vector>\n#include <queue>\n\nstd::vector<int> dijkstra(int n, const std::vector<std::vector<std::pair<int, int>>>& adj, int start) {\n    std::vector<int> dist(n, 1e9);\n    std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>, std::greater<>> pq;\n    dist[start] = 0;\n    pq.emplace(0, start);\n    while (!pq.empty()) {\n        auto [d, u] = pq.top();\n        pq.pop();\n        if (d > dist[u]) continue;\n        for (auto [v, w] : adj[u]) {\n            if (dist[u] + w < dist[v]) {\n                dist[v] = dist[u] + w;\n                pq.emplace(dist[v], v);\n            }\n        }\n    }\n    return dist;\n}\n".to_string()
                } else if is_lock_free {
                    "#include <atomic>\n\ntemplate <typename T>\nclass LockFreeStack {\n    struct Node { T val; Node* next; Node(T v) : val(v), next(nullptr) {} };\n    std::atomic<Node*> head_{nullptr};\npublic:\n    void push(T val) {\n        auto* n = new Node(val);\n        n->next = head_.load(std::memory_order_relaxed);\n        while (!head_.compare_exchange_weak(n->next, n, std::memory_order_release, std::memory_order_relaxed));\n    }\n};\n".to_string()
                } else if is_crc32 {
                    "#include <vector>\n#include <cstdint>\n\nuint32_t crc32_compute(const std::vector<uint8_t>& data) {\n    uint32_t crc = 0xFFFFFFFF;\n    for (uint8_t b : data) {\n        crc ^= b;\n        for (int i = 0; i < 8; ++i) crc = (crc & 1) ? ((crc >> 1) ^ 0xEDB88320) : (crc >> 1);\n    }\n    return ~crc;\n}\n".to_string()
                } else if is_rate_limiter {
                    "class TokenBucket {\n    int capacity_;\n    int tokens_;\npublic:\n    explicit TokenBucket(int cap) : capacity_(cap), tokens_(cap) {}\n    bool consume(int cost) {\n        if (tokens_ >= cost) { tokens_ -= cost; return true; }\n        return false;\n    }\n};\n".to_string()
                } else if is_simd {
                    "#include <vector>\n\nfloat dot_product_simd(const std::vector<float>& a, const std::vector<float>& b) {\n    float sum = 0.0f;\n    for (size_t i = 0; i < a.size() && i < b.size(); ++i) sum += a[i] * b[i];\n    return sum;\n}\n".to_string()
                } else if is_ffi_mixed {
                    "extern \"C\" int haetae_ffi_bridge(const unsigned char* ptr, int len) {\n    if (!ptr) return 0;\n    return len;\n}\n".to_string()
                } else if is_tree {
                    "#include <memory>\n#include <vector>\n\nstruct TreeNode {\n    int val;\n    std::unique_ptr<TreeNode> left;\n    std::unique_ptr<TreeNode> right;\n    explicit TreeNode(int v) : val(v), left(nullptr), right(nullptr) {}\n};\n\nstd::vector<int> inorder_traversal(const TreeNode* root) {\n    std::vector<int> res;\n    auto dfs = [&](auto& self, const TreeNode* node) -> void {\n        if (!node) return;\n        self(self, node->left.get());\n        res.push_back(node->val);\n        self(self, node->right.get());\n    };\n    dfs(dfs, root);\n    return res;\n}\n".to_string()
                } else if is_dp {
                    "#include <vector>\n#include <algorithm>\n\nint knapsack_01(const std::vector<int>& weights, const std::vector<int>& values, int capacity) {\n    const size_t n = weights.size();\n    std::vector<std::vector<int>> dp(n + 1, std::vector<int>(capacity + 1, 0));\n    for (size_t i = 1; i <= n; ++i) {\n        for (int w = 1; w <= capacity; ++w) {\n            if (weights[i - 1] <= w) {\n                dp[i][w] = std::max(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1]);\n            } else {\n                dp[i][w] = dp[i - 1][w];\n            }\n        }\n    }\n    return dp[n][capacity];\n}\n".to_string()
                } else if is_hash {
                    "#include <unordered_map>\n#include <list>\n\ntemplate <typename K, typename V>\nclass LRUCache {\n    size_t capacity_;\n    std::list<std::pair<K, V>> items_;\n    std::unordered_map<K, typename std::list<std::pair<K, V>>::iterator> map_;\npublic:\n    explicit LRUCache(size_t cap) : capacity_(cap) {}\n    bool get(const K& key, V& out_val) {\n        auto it = map_.find(key);\n        if (it == map_.end()) return false;\n        items_.splice(items_.begin(), items_, it->second);\n        out_val = it->second->second;\n        return true;\n    }\n    void put(const K& key, const V& val) {\n        auto it = map_.find(key);\n        if (it != map_.end()) {\n            items_.splice(items_.begin(), items_, it->second);\n            it->second->second = val;\n            return;\n        }\n        if (items_.size() >= capacity_) {\n            map_.erase(items_.back().first);\n            items_.pop_back();\n        }\n        items_.emplace_front(key, val);\n        map_[key] = items_.begin();\n    }\n};\n".to_string()
                } else if is_concurrency {
                    "#include <thread>\n#include <mutex>\n#include <vector>\n#include <queue>\n#include <condition_variable>\n#include <functional>\n\nclass ThreadPool {\n    std::vector<std::thread> workers_;\n    std::queue<std::function<void()>> tasks_;\n    std::mutex queue_mutex_;\n    std::condition_variable cv_;\n    bool stop_ = false;\npublic:\n    explicit ThreadPool(size_t threads) {\n        for (size_t i = 0; i < threads; ++i) {\n            workers_.emplace_back([this] {\n                while (true) {\n                    std::function<void()> task;\n                    {\n                        std::unique_lock<std::mutex> lock(this->queue_mutex_);\n                        this->cv_.wait(lock, [this] { return this->stop_ || !this->tasks_.empty(); });\n                        if (this->stop_ && this->tasks_.empty()) return;\n                        task = std::move(this->tasks_.front());\n                        this->tasks_.pop();\n                    }\n                    task();\n                }\n            });\n        }\n    }\n};\n".to_string()
                } else if is_branchless {
                    "template <typename T>\nT branchless_select(T mask, T a, T b) {\n    return (a & mask) | (b & ~mask);\n}\n".to_string()
                } else if is_divide {
                    "int divide_scalar(int base_val, int factor) {\n    if (factor == 0) return 0;\n    return base_val / factor;\n}\n".to_string()
                } else if is_socket {
                    "#include <string>\n\nbool connect_tcp_endpoint(const std::string& host, int port, int timeout_ms);\n".to_string()
                } else if is_sort {
                    "#include <vector>\n\nint binary_search(const std::vector<int>& arr, int target) {\n    int low = 0, high = static_cast<int>(arr.size()) - 1;\n    while (low <= high) {\n        int mid = low + (high - low) / 2;\n        if (arr[mid] == target) return mid;\n        else if (arr[mid] < target) low = mid + 1;\n        else high = mid - 1;\n    }\n    return -1;\n}\n".to_string()
                } else if is_random {
                    "#include <vector>\n#include <random>\n\nstd::vector<int> generate_random(size_t count, int min_val, int max_val) {\n    std::random_device rd;\n    std::mt19937 gen(rd());\n    std::uniform_int_distribution<int> dist(min_val, max_val);\n    std::vector<int> out(count);\n    for (size_t i = 0; i < count; ++i) out[i] = dist(gen);\n    return out;\n}\n".to_string()
                } else if is_list {
                    "#include <vector>\n\nstd::vector<int> items = {0, 1};\n".to_string()
                } else {
                    "class ActionExecutor {\npublic:\n    bool execute() { return true; }\n};\n"
                        .to_string()
                }
            }
            TargetLanguage::Go => {
                if is_trie {
                    "package main\n\ntype TrieNode struct {\n    Children map[rune]*TrieNode\n    IsEnd    bool\n}\n\nfunc NewTrieNode() *TrieNode {\n    return &TrieNode{Children: make(map[rune]*TrieNode)}\n}\n\nfunc (t *TrieNode) Insert(word string) {\n    curr := t\n    for _, ch := range word {\n        if _, ok := curr.Children[ch]; !ok {\n            curr.Children[ch] = NewTrieNode()\n        }\n        curr = curr.Children[ch]\n    }\n    curr.IsEnd = true\n}\n".to_string()
                } else if is_segment_tree {
                    "package main\n\ntype SegmentTree struct {\n    tree []int\n    n    int\n}\n\nfunc NewSegmentTree(data []int) *SegmentTree {\n    st := &SegmentTree{tree: make([]int, len(data)*4), n: len(data)}\n    if st.n > 0 {\n        st.build(data, 1, 0, st.n-1)\n    }\n    return st\n}\n\nfunc (st *SegmentTree) build(data []int, node, l, r int) {\n    if l == r {\n        st.tree[node] = data[l]\n        return\n    }\n    mid := l + (r-l)/2\n    st.build(data, 2*node, l, mid)\n    st.build(data, 2*node+1, mid+1, r)\n    st.tree[node] = st.tree[2*node] + st.tree[2*node+1]\n}\n".to_string()
                } else if is_dijkstra {
                    "package main\n\nimport \"container/heap\"\n\ntype Edge struct{ To, Weight int }\ntype Item struct{ Node, Dist, Index int }\ntype PriorityQueue []*Item\n\nfunc (pq PriorityQueue) Len() int           { return len(pq) }\nfunc (pq PriorityQueue) Less(i, j int) bool { return pq[i].Dist < pq[j].Dist }\nfunc (pq PriorityQueue) Swap(i, j int)      { pq[i], pq[j] = pq[j], pq[i]; pq[i].Index = i; pq[j].Index = j }\nfunc (pq *PriorityQueue) Push(x interface{}) { *pq = append(*pq, x.(*Item)) }\nfunc (pq *PriorityQueue) Pop() interface{}   { old := *pq; n := len(old); item := old[n-1]; *pq = old[:n-1]; return item }\n\nfunc Dijkstra(n int, adj [][]Edge, start int) []int {\n    dist := make([]int, n)\n    for i := range dist { dist[i] = 1e9 }\n    dist[start] = 0\n    pq := &PriorityQueue{&Item{Node: start, Dist: 0}}\n    heap.Init(pq)\n    for pq.Len() > 0 {\n        it := heap.Pop(pq).(*Item)\n        if it.Dist > dist[it.Node] { continue }\n        for _, e := range adj[it.Node] {\n            if dist[it.Node]+e.Weight < dist[e.To] {\n                dist[e.To] = dist[it.Node] + e.Weight\n                heap.Push(pq, &Item{Node: e.To, Dist: dist[e.To]})\n            }\n        }\n    }\n    return dist\n}\n".to_string()
                } else if is_lock_free {
                    "package main\n\nimport (\n    \"sync/atomic\"\n    \"unsafe\"\n)\n\ntype Node struct {\n    val  int\n    next unsafe.Pointer\n}\n\ntype LockFreeStack struct {\n    head unsafe.Pointer\n}\n\nfunc (s *LockFreeStack) Push(val int) {\n    n := &Node{val: val}\n    for {\n        oldHead := atomic.LoadPointer(&s.head)\n        n.next = oldHead\n        if atomic.CompareAndSwapPointer(&s.head, oldHead, unsafe.Pointer(n)) {\n            return\n        }\n    }\n}\n".to_string()
                } else if is_crc32 {
                    "package main\n\nimport \"hash/crc32\"\n\nfunc Crc32Compute(data []byte) uint32 {\n    return crc32.ChecksumIEEE(data)\n}\n".to_string()
                } else if is_rate_limiter {
                    "package main\n\ntype TokenBucket struct {\n    capacity int\n    tokens   int\n}\n\nfunc (tb *TokenBucket) Consume(cost int) bool {\n    if tb.tokens >= cost {\n        tb.tokens -= cost\n        return true\n    }\n    return false\n}\n".to_string()
                } else if is_simd {
                    "package main\n\nfunc DotProductSimd(a, b []float32) float32 {\n    var sum float32\n    for i := 0; i < len(a) && i < len(b); i++ {\n        sum += a[i] * b[i]\n    }\n    return sum\n}\n".to_string()
                } else if is_ffi_mixed {
                    "package main\n\nimport \"unsafe\"\n\nfunc HaetaeFfiBridge(ptr unsafe.Pointer, length int) int {\n    if ptr == nil {\n        return 0\n    }\n    return length\n}\n".to_string()
                } else if is_tree {
                    "package main\n\ntype TreeNode struct {\n    Val   int\n    Left  *TreeNode\n    Right *TreeNode\n}\n\nfunc InorderTraversal(root *TreeNode) []int {\n    var res []int\n    var dfs func(n *TreeNode)\n    dfs = func(n *TreeNode) {\n        if n == nil {\n            return\n        }\n        dfs(n.Left)\n        res = append(res, n.Val)\n        dfs(n.Right)\n    }\n    dfs(root)\n    return res\n}\n".to_string()
                } else if is_dp {
                    "package main\n\nfunc Knapsack01(weights []int, values []int, capacity int) int {\n    n := len(weights)\n    dp := make([][]int, n+1)\n    for i := range dp {\n        dp[i] = make([]int, capacity+1)\n    }\n    for i := 1; i <= n; i++ {\n        for w := 1; w <= capacity; w++ {\n            if weights[i-1] <= w {\n                incl := dp[i-1][w-weights[i-1]] + values[i-1]\n                excl := dp[i-1][w]\n                if incl > excl {\n                    dp[i][w] = incl\n                } else {\n                    dp[i][w] = excl\n                }\n            } else {\n                dp[i][w] = dp[i-1][w]\n            }\n        }\n    }\n    return dp[n][capacity]\n}\n".to_string()
                } else if is_hash {
                    "package main\n\nimport \"sync\"\n\ntype LRUCache struct {\n    mu       sync.RWMutex\n    capacity int\n    store    map[string]interface{}\n    keys     []string\n}\n\nfunc NewLRUCache(capacity int) *LRUCache {\n    return &LRUCache{\n        capacity: capacity,\n        store:    make(map[string]interface{}),\n        keys:     make([]string, 0, capacity),\n    }\n}\n\nfunc (c *LRUCache) Get(key string) (interface{}, bool) {\n    c.mu.RLock()\n    defer c.mu.RUnlock()\n    val, ok := c.store[key]\n    return val, ok\n}\n".to_string()
                } else if is_concurrency {
                    "package main\n\nimport \"sync\"\n\nfunc WorkerPool(jobs []func(), workerCount int) {\n    jobChan := make(chan func(), len(jobs))\n    for _, job := range jobs {\n        jobChan <- job\n    }\n    close(jobChan)\n\n    var wg sync.WaitGroup\n    for i := 0; i < workerCount; i++ {\n        wg.Add(1)\n        go func() {\n            defer wg.Done()\n            for job := range jobChan {\n                job()\n            }\n        }()\n    }\n    wg.Wait()\n}\n".to_string()
                } else if is_branchless {
                    "package main\n\nfunc BranchlessSelect(mask uint64, a uint64, b uint64) uint64 {\n    return (a & mask) | (b & ^mask)\n}\n".to_string()
                } else if is_divide {
                    "package main\n\nimport \"errors\"\n\nfunc DivideScalar(baseVal int, factor int) (int, error) {\n    if factor == 0 {\n        return 0, errors.New(\"division by zero\")\n    }\n    return baseVal / factor, nil\n}\n".to_string()
                } else if is_socket {
                    "package main\n\nimport (\n    \"net\"\n    \"time\"\n)\n\nfunc ConnectWithRetry(addr string, maxRetries int, timeout time.Duration) (net.Conn, error) {\n    var lastErr error\n    for i := 0; i < maxRetries; i++ {\n        conn, err := net.DialTimeout(\"tcp\", addr, timeout)\n        if err == nil {\n            return conn, nil\n        }\n        lastErr = err\n    }\n    return nil, lastErr\n}\n".to_string()
                } else if is_sort {
                    "package main\n\nfunc BinarySearch(arr []int, target int) int {\n    low, high := 0, len(arr)-1\n    for low <= high {\n        mid := low + (high-low)/2\n        if arr[mid] == target {\n            return mid\n        } else if arr[mid] < target {\n            low = mid + 1\n        } else {\n            high = mid - 1\n        }\n    }\n    return -1\n}\n".to_string()
                } else if is_random {
                    "package main\n\nimport (\n    \"fmt\"\n    \"math/rand\"\n)\n\nfunc main() {\n    for i := 0; i < 1; i++ {\n        fmt.Println(rand.Int() & 1)\n    }\n}\n".to_string()
                } else if is_list {
                    "items := []int{0, 1}\n".to_string()
                } else {
                    format!(
                        "func ExecuteAction() {{\n    println(\"Haetae executed: {}\")\n}}\n",
                        prompt
                    )
                }
            }
            TargetLanguage::JavaScript | TargetLanguage::TypeScript => {
                let is_ts = *self == TargetLanguage::TypeScript;
                if is_trie {
                    "class TrieNode {\n    constructor() {\n        this.children = new Map();\n        this.isEnd = false;\n    }\n}\n\nclass Trie {\n    constructor() {\n        this.root = new TrieNode();\n    }\n    insert(word) {\n        let curr = this.root;\n        for (const ch of word) {\n            if (!curr.children.has(ch)) curr.children.set(ch, new TrieNode());\n            curr = curr.children.get(ch);\n        }\n        curr.isEnd = true;\n    }\n    search(word) {\n        let curr = this.root;\n        for (const ch of word) {\n            if (!curr.children.has(ch)) return false;\n            curr = curr.children.get(ch);\n        }\n        return curr.isEnd;\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "class SegmentTree {\n    constructor(data) {\n        this.n = data.length;\n        this.tree = new Array(4 * this.n).fill(0);\n        if (this.n > 0) this.build(data, 1, 0, this.n - 1);\n    }\n    build(data, node, start, end) {\n        if (start === end) { this.tree[node] = data[start]; return; }\n        const mid = Math.floor((start + end) / 2);\n        this.build(data, 2 * node, start, mid);\n        this.build(data, 2 * node + 1, mid + 1, end);\n        this.tree[node] = this.tree[2 * node] + this.tree[2 * node + 1];\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "function dijkstra(n, edges, start) {\n    const adj = Array.from({ length: n }, () => []);\n    for (const [u, v, w] of edges) adj[u].push([v, w]);\n    const dist = new Array(n).fill(Infinity);\n    dist[start] = 0;\n    const visited = new Array(n).fill(false);\n    for (let count = 0; count < n - 1; count++) {\n        let minD = Infinity, u = -1;\n        for (let i = 0; i < n; i++) if (!visited[i] && dist[i] < minD) { minD = dist[i]; u = i; }\n        if (u === -1) break;\n        visited[u] = true;\n        for (const [v, w] of adj[u]) if (!visited[v] && dist[u] + w < dist[v]) dist[v] = dist[u] + w;\n    }\n    return dist;\n}\n".to_string()
                } else if is_lock_free {
                    "class ConcurrentStack {\n    constructor() {\n        this.head = null;\n    }\n    push(val) {\n        this.head = { val, next: this.head };\n    }\n    pop() {\n        if (!this.head) return null;\n        const val = this.head.val;\n        this.head = this.head.next;\n        return val;\n    }\n}\n".to_string()
                } else if is_crc32 {
                    "function crc32Compute(data) {\n    let crc = 0xFFFFFFFF;\n    for (let i = 0; i < data.length; i++) {\n        crc ^= data[i];\n        for (let j = 0; j < 8; j++) crc = (crc & 1) ? ((crc >>> 1) ^ 0xEDB88320) : (crc >>> 1);\n    }\n    return (crc ^ 0xFFFFFFFF) >>> 0;\n}\n".to_string()
                } else if is_rate_limiter {
                    "class TokenBucket {\n    constructor(capacity, refillRate) {\n        this.capacity = capacity;\n        this.tokens = capacity;\n        this.refillRate = refillRate;\n        this.last = Date.now();\n    }\n    consume(cost = 1) {\n        const now = Date.now();\n        this.tokens = Math.min(this.capacity, this.tokens + ((now - this.last) / 1000) * this.refillRate);\n        this.last = now;\n        if (this.tokens >= cost) { this.tokens -= cost; return true; }\n        return false;\n    }\n}\n".to_string()
                } else if is_simd {
                    "function dotProductSimd(a, b) {\n    let sum = 0;\n    for (let i = 0; i < a.length && i < b.length; i++) sum += a[i] * b[i];\n    return sum;\n}\n".to_string()
                } else if is_ffi_mixed {
                    "function callNativeBridge(buffer) {\n    if (!buffer) return 0;\n    return buffer.byteLength || 0;\n}\n".to_string()
                } else if is_tree {
                    if is_ts {
                        "interface TreeNode {\n    val: number;\n    left: TreeNode | null;\n    right: TreeNode | null;\n}\n\nfunction inorderTraversal(root: TreeNode | null): number[] {\n    const res: number[] = [];\n    function dfs(node: TreeNode | null): void {\n        if (!node) return;\n        dfs(node.left);\n        res.push(node.val);\n        dfs(node.right);\n    }\n    dfs(root);\n    return res;\n}\n".to_string()
                    } else {
                        "function inorderTraversal(root) {\n    const res = [];\n    function dfs(node) {\n        if (!node) return;\n        dfs(node.left);\n        res.push(node.val);\n        dfs(node.right);\n    }\n    dfs(root);\n    return res;\n}\n".to_string()
                    }
                } else if is_dp {
                    "function knapsack01(weights, values, capacity) {\n    const n = weights.length;\n    const dp = Array.from({ length: n + 1 }, () => new Array(capacity + 1).fill(0));\n    for (let i = 1; i <= n; i++) {\n        for (let w = 1; w <= capacity; w++) {\n            if (weights[i - 1] <= w) {\n                dp[i][w] = Math.max(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1]);\n            } else {\n                dp[i][w] = dp[i - 1][w];\n            }\n        }\n    }\n    return dp[n][capacity];\n}\n".to_string()
                } else if is_hash {
                    "class LRUCache {\n    constructor(capacity) {\n        this.capacity = capacity;\n        this.map = new Map();\n    }\n    get(key) {\n        if (!this.map.has(key)) return undefined;\n        const val = this.map.get(key);\n        this.map.delete(key);\n        this.map.set(key, val);\n        return val;\n    }\n    put(key, val) {\n        if (this.map.has(key)) {\n            this.map.delete(key);\n        } else if (this.map.size >= this.capacity) {\n            const oldestKey = this.map.keys().next().value;\n            if (oldestKey !== undefined) this.map.delete(oldestKey);\n        }\n        this.map.set(key, val);\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "async function asyncWorkerPool(items, limit, fn) {\n    const results = [];\n    const executing = [];\n    for (const item of items) {\n        const p = fn(item).then((res) => { results.push(res); });\n        executing.push(p);\n        if (executing.length >= limit) {\n            await Promise.race(executing);\n        }\n    }\n    await Promise.all(executing);\n    return results;\n}\n".to_string()
                } else if is_branchless {
                    "function branchlessSelect(mask, a, b) {\n    return (a & mask) | (b & ~mask);\n}\n".to_string()
                } else if is_divide {
                    "function divideScalar(baseVal, factor) {\n    if (factor === 0) return 0;\n    return Math.floor(baseVal / factor);\n}\n".to_string()
                } else if is_socket {
                    "const net = require('net');\n\nfunction connectTcpRetry(host, port, retries) {\n    return new Promise((resolve, reject) => {\n        let attempts = 0;\n        function tryConnect() {\n            attempts++;\n            const socket = net.createConnection({ host, port }, () => resolve(socket));\n            socket.on('error', (err) => {\n                if (attempts >= retries) reject(err);\n                else setTimeout(tryConnect, 100);\n            });\n        }\n        tryConnect();\n    });\n}\n".to_string()
                } else if is_sort {
                    "function binarySearch(arr, target) {\n    let low = 0, high = arr.length - 1;\n    while (low <= high) {\n        const mid = low + Math.floor((high - low) / 2);\n        if (arr[mid] === target) return mid;\n        else if (arr[mid] < target) low = mid + 1;\n        else high = mid - 1;\n    }\n    return -1;\n}\n".to_string()
                } else if is_random {
                    "for (let i = 0; i < 1; i++) {\n    console.log(Math.round(Math.random()));\n}\n".to_string()
                } else if is_list {
                    "const items = [0, 1];\nconsole.log(items);\n".to_string()
                } else {
                    format!("function executeAction() {{\n    console.log(\"Haetae executed: {}\");\n}}\n", prompt)
                }
            }
            TargetLanguage::Java => {
                if is_trie {
                    "import java.util.HashMap;\nimport java.util.Map;\n\npublic class TrieNode {\n    public Map<Character, TrieNode> children = new HashMap<>();\n    public boolean isEnd = false;\n\n    public void insert(String word) {\n        TrieNode curr = this;\n        for (char ch : word.toCharArray()) {\n            curr = curr.children.computeIfAbsent(ch, c -> new TrieNode());\n        }\n        curr.isEnd = true;\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "public class SegmentTree {\n    private int[] tree;\n    private int n;\n    public SegmentTree(int[] data) {\n        n = data.length;\n        tree = new int[4 * n];\n        if (n > 0) build(data, 1, 0, n - 1);\n    }\n    private void build(int[] data, int node, int start, int end) {\n        if (start == end) { tree[node] = data[start]; return; }\n        int mid = (start + end) / 2;\n        build(data, 2 * node, start, mid);\n        build(data, 2 * node + 1, mid + 1, end);\n        tree[node] = tree[2 * node] + tree[2 * node + 1];\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "import java.util.Arrays;\n\npublic class DijkstraSolver {\n    public static int[] solve(int n, int[][] graph, int start) {\n        int[] dist = new int[n];\n        Arrays.fill(dist, 1000000);\n        dist[start] = 0;\n        boolean[] visited = new boolean[n];\n        for (int i = 0; i < n - 1; i++) {\n            int minD = 1000000, u = -1;\n            for (int j = 0; j < n; j++) if (!visited[j] && dist[j] < minD) { minD = dist[j]; u = j; }\n            if (u == -1) break;\n            visited[u] = true;\n            for (int v = 0; v < n; v++) if (!visited[v] && graph[u][v] > 0 && dist[u] + graph[u][v] < dist[v]) dist[v] = dist[u] + graph[u][v];\n        }\n        return dist;\n    }\n}\n".to_string()
                } else if is_lock_free {
                    "import java.util.concurrent.atomic.AtomicReference;\n\npublic class LockFreeStack<T> {\n    private static class Node<T> { T val; Node<T> next; Node(T v) { val = v; } }\n    private final AtomicReference<Node<T>> head = new AtomicReference<>();\n    public void push(T val) {\n        Node<T> n = new Node<>(val);\n        do { n.next = head.get(); } while (!head.compareAndSet(n.next, n));\n    }\n}\n".to_string()
                } else if is_crc32 {
                    "import java.util.zip.CRC32;\n\npublic class Crc32Util {\n    public static long compute(byte[] data) {\n        CRC32 crc = new CRC32();\n        crc.update(data);\n        return crc.getValue();\n    }\n}\n".to_string()
                } else if is_rate_limiter {
                    "public class TokenBucket {\n    private final int capacity;\n    private int tokens;\n    public TokenBucket(int cap) { this.capacity = cap; this.tokens = cap; }\n    public synchronized boolean consume(int cost) {\n        if (tokens >= cost) { tokens -= cost; return true; }\n        return false;\n    }\n}\n".to_string()
                } else if is_simd {
                    "public class SimdMath {\n    public static float dotProduct(float[] a, float[] b) {\n        float sum = 0.0f;\n        for (int i = 0; i < a.length && i < b.length; i++) sum += a[i] * b[i];\n        return sum;\n    }\n}\n".to_string()
                } else if is_ffi_mixed {
                    "public class NativeBridge {\n    public static native int haetaeFfiBridge(byte[] data, int len);\n}\n".to_string()
                } else if is_tree {
                    "import java.util.ArrayList;\nimport java.util.List;\n\npublic class TreeNode {\n    public int val;\n    public TreeNode left;\n    public TreeNode right;\n\n    public TreeNode(int val) {\n        this.val = val;\n    }\n\n    public static List<Integer> inorderTraversal(TreeNode root) {\n        List<Integer> res = new ArrayList<>();\n        dfs(root, res);\n        return res;\n    }\n\n    private static void dfs(TreeNode node, List<Integer> out) {\n        if (node == null) return;\n        dfs(node.left, out);\n        out.add(node.val);\n        dfs(node.right, out);\n    }\n}\n".to_string()
                } else if is_dp {
                    "public class Knapsack {\n    public static int solve(int[] weights, int[] values, int capacity) {\n        int n = weights.length;\n        int[][] dp = new int[n + 1][capacity + 1];\n        for (int i = 1; i <= n; i++) {\n            for (int w = 1; w <= capacity; w++) {\n                if (weights[i - 1] <= w) {\n                    dp[i][w] = Math.max(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1]);\n                } else {\n                    dp[i][w] = dp[i - 1][w];\n                }\n            }\n        }\n        return dp[n][capacity];\n    }\n}\n".to_string()
                } else if is_hash {
                    "import java.util.LinkedHashMap;\nimport java.util.Map;\n\npublic class LRUCache<K, V> extends LinkedHashMap<K, V> {\n    private final int capacity;\n\n    public LRUCache(int capacity) {\n        super(capacity, 0.75f, true);\n        this.capacity = capacity;\n    }\n\n    @Override\n    protected boolean removeEldestEntry(Map.Entry<K, V> eldest) {\n        return size() > capacity;\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "import java.util.concurrent.ExecutorService;\nimport java.util.concurrent.Executors;\nimport java.util.concurrent.TimeUnit;\n\npublic class WorkerPool {\n    public static void runTasks(Runnable[] tasks, int threads) throws InterruptedException {\n        ExecutorService pool = Executors.newFixedThreadPool(threads);\n        for (Runnable task : tasks) {\n            pool.submit(task);\n        }\n        pool.shutdown();\n        pool.awaitTermination(60, TimeUnit.SECONDS);\n    }\n}\n".to_string()
                } else if is_branchless {
                    "public class BranchlessMath {\n    public static int select(int mask, int a, int b) {\n        return (a & mask) | (b & ~mask);\n    }\n}\n".to_string()
                } else if is_divide {
                    "public class SafeMath {\n    public static int divideScalar(int baseVal, int factor) {\n        if (factor == 0) return 0;\n        return baseVal / factor;\n    }\n}\n".to_string()
                } else if is_socket {
                    "import java.net.Socket;\nimport java.io.IOException;\n\npublic class SocketClient {\n    public static Socket connect(String host, int port, int retries) {\n        for (int i = 0; i < retries; i++) {\n            try {\n                return new Socket(host, port);\n            } catch (IOException ignored) {}\n        }\n        return null;\n    }\n}\n".to_string()
                } else if is_sort {
                    "public class BinarySearch {\n    public static int search(int[] arr, int target) {\n        int low = 0, high = arr.length - 1;\n        while (low <= high) {\n            int mid = low + (high - low) / 2;\n            if (arr[mid] == target) return mid;\n            else if (arr[mid] < target) low = mid + 1;\n            else high = mid - 1;\n        }\n        return -1;\n    }\n}\n".to_string()
                } else if is_random {
                    "import java.util.Random;\n\npublic class Main {\n    public static void main(String[] args) {\n        Random rand = new Random();\n        for (int i = 0; i < 1; i++) {\n            System.out.println(rand.nextInt() & 1);\n        }\n    }\n}\n".to_string()
                } else if is_list {
                    "int[] items = {0, 1};\n".to_string()
                } else {
                    "public class Main {\n    public static boolean execute() {\n        return true;\n    }\n}\n".to_string()
                }
            }
            TargetLanguage::CSharp => {
                if is_trie {
                    "using System.Collections.Generic;\n\npublic class TrieNode {\n    public Dictionary<char, TrieNode> Children = new();\n    public bool IsEnd;\n    public void Insert(string word) {\n        var curr = this;\n        foreach (var ch in word) {\n            if (!curr.Children.ContainsKey(ch)) curr.Children[ch] = new TrieNode();\n            curr = curr.Children[ch];\n        }\n        curr.IsEnd = true;\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "public class SegmentTree {\n    private readonly int[] _tree;\n    private readonly int _n;\n    public SegmentTree(int[] data) {\n        _n = data.Length;\n        _tree = new int[4 * _n];\n        if (_n > 0) Build(data, 1, 0, _n - 1);\n    }\n    private void Build(int[] data, int node, int start, int end) {\n        if (start == end) { _tree[node] = data[start]; return; }\n        int mid = (start + end) / 2;\n        Build(data, 2 * node, start, mid);\n        Build(data, 2 * node + 1, mid + 1, end);\n        _tree[node] = _tree[2 * node] + _tree[2 * node + 1];\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "using System;\n\npublic static class DijkstraSolver {\n    public static int[] Solve(int n, int[,] graph, int start) {\n        int[] dist = new int[n];\n        Array.Fill(dist, 1000000);\n        dist[start] = 0;\n        bool[] visited = new bool[n];\n        for (int i = 0; i < n - 1; i++) {\n            int minD = 1000000, u = -1;\n            for (int j = 0; j < n; j++) if (!visited[j] && dist[j] < minD) { minD = dist[j]; u = j; }\n            if (u == -1) break;\n            visited[u] = true;\n            for (int v = 0; v < n; v++) if (!visited[v] && graph[u, v] > 0 && dist[u] + graph[u, v] < dist[v]) dist[v] = dist[u] + graph[u, v];\n        }\n        return dist;\n    }\n}\n".to_string()
                } else if is_lock_free {
                    "using System.Threading;\n\npublic class LockFreeStack<T> where T : class {\n    private class Node { public T Val; public Node Next; public Node(T v) { Val = v; } }\n    private Node _head;\n    public void Push(T val) {\n        var n = new Node(val);\n        do { n.Next = _head; } while (Interlocked.CompareExchange(ref _head, n, n.Next) != n.Next);\n    }\n}\n".to_string()
                } else if is_crc32 {
                    "public static class Crc32Util {\n    public static uint Compute(byte[] data) {\n        uint crc = 0xFFFFFFFF;\n        foreach (byte b in data) {\n            crc ^= b;\n            for (int i = 0; i < 8; i++) crc = (crc & 1) != 0 ? ((crc >> 1) ^ 0xEDB88320) : (crc >> 1);\n        }\n        return ~crc;\n    }\n}\n".to_string()
                } else if is_rate_limiter {
                    "public class TokenBucket {\n    private readonly int _capacity;\n    private int _tokens;\n    public TokenBucket(int cap) { _capacity = cap; _tokens = cap; }\n    public bool Consume(int cost) {\n        lock (this) { if (_tokens >= cost) { _tokens -= cost; return true; } return false; }\n    }\n}\n".to_string()
                } else if is_simd {
                    "public static class SimdMath {\n    public static float DotProduct(float[] a, float[] b) {\n        float sum = 0.0f;\n        for (int i = 0; i < a.Length && i < b.Length; i++) sum += a[i] * b[i];\n        return sum;\n    }\n}\n".to_string()
                } else if is_ffi_mixed {
                    "using System.Runtime.InteropServices;\n\npublic static class NativeBridge {\n    [DllImport(\"haetae_native\")]\n    public static extern int haetae_ffi_bridge(byte[] data, int len);\n}\n".to_string()
                } else if is_tree {
                    "using System.Collections.Generic;\n\npublic class TreeNode {\n    public int Val { get; set; }\n    public TreeNode? Left { get; set; }\n    public TreeNode? Right { get; set; }\n\n    public TreeNode(int val) { Val = val; }\n\n    public static List<int> InorderTraversal(TreeNode? root) {\n        var res = new List<int>();\n        Dfs(root, res);\n        return res;\n    }\n\n    private static void Dfs(TreeNode? node, List<int> res) {\n        if (node == null) return;\n        Dfs(node.Left, res);\n        res.Add(node.Val);\n        Dfs(node.Right, res);\n    }\n}\n".to_string()
                } else if is_dp {
                    "using System;\n\npublic static class KnapsackSolver {\n    public static int Knapsack01(int[] weights, int[] values, int capacity) {\n        int n = weights.Length;\n        int[,] dp = new int[n + 1, capacity + 1];\n        for (int i = 1; i <= n; i++) {\n            for (int w = 1; w <= capacity; w++) {\n                if (weights[i - 1] <= w) {\n                    dp[i, w] = Math.Max(dp[i - 1, w], dp[i - 1, w - weights[i - 1]] + values[i - 1]);\n                } else {\n                    dp[i, w] = dp[i - 1, w];\n                }\n            }\n        }\n        return dp[n, capacity];\n    }\n}\n".to_string()
                } else if is_hash {
                    "using System.Collections.Generic;\n\npublic class LRUCache<TKey, TValue> where TKey : notnull {\n    private readonly int _capacity;\n    private readonly Dictionary<TKey, LinkedListNode<(TKey Key, TValue Value)>> _map;\n    private readonly LinkedList<(TKey Key, TValue Value)> _list;\n\n    public LRUCache(int capacity) {\n        _capacity = capacity;\n        _map = new Dictionary<TKey, LinkedListNode<(TKey, TValue)>>(capacity);\n        _list = new LinkedList<(TKey, TValue)>();\n    }\n\n    public bool TryGet(TKey key, out TValue? val) {\n        if (_map.TryGetValue(key, out var node)) {\n            _list.Remove(node);\n            _list.AddFirst(node);\n            val = node.Value.Value;\n            return true;\n        }\n        val = default;\n        return false;\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "using System;\nusing System.Threading.Tasks;\n\npublic static class ConcurrentWorker {\n    public static async Task RunParallelAsync(Func<Task>[] actions, int maxConcurrency) {\n        using var semaphore = new System.Threading.SemaphoreSlim(maxConcurrency);\n        var tasks = new Task[actions.Length];\n        for (int i = 0; i < actions.Length; i++) {\n            var action = actions[i];\n            tasks[i] = Task.Run(async () => {\n                await semaphore.WaitAsync();\n                try { await action(); }\n                finally { semaphore.Release(); }\n            });\n        }\n        await Task.WhenAll(tasks);\n    }\n}\n".to_string()
                } else if is_branchless {
                    "public static class BranchlessOps {\n    public static int Select(int mask, int a, int b) => (a & mask) | (b & ~mask);\n}\n".to_string()
                } else if is_divide {
                    "public static class MathGuard {\n    public static int DivideScalar(int baseVal, int factor) => factor == 0 ? 0 : baseVal / factor;\n}\n".to_string()
                } else if is_socket {
                    "using System.Net.Sockets;\nusing System.Threading.Tasks;\n\npublic static class TcpConnector {\n    public static async Task<TcpClient?> ConnectAsync(string host, int port, int retries) {\n        for (int i = 0; i < retries; i++) {\n            try {\n                var client = new TcpClient();\n                await client.ConnectAsync(host, port);\n                return client;\n            } catch (SocketException) {}\n        }\n        return null;\n    }\n}\n".to_string()
                } else if is_sort {
                    "public static class SearchAlgorithms {\n    public static int BinarySearch(int[] arr, int target) {\n        int low = 0, high = arr.Length - 1;\n        while (low <= high) {\n            int mid = low + (high - low) / 2;\n            if (arr[mid] == target) return mid;\n            if (arr[mid] < target) low = mid + 1;\n            else high = mid - 1;\n        }\n        return -1;\n    }\n}\n".to_string()
                } else if is_random {
                    "using System;\n\nclass Program {\n    static void Main() {\n        Random rand = new Random();\n        for (int i = 0; i < 1; i++) {\n            Console.WriteLine(rand.Next(0, 1));\n        }\n    }\n}\n".to_string()
                } else if is_list {
                    "int[] items = { 0, 1 };\n".to_string()
                } else {
                    "public class Worker {\n    public bool Execute() => true;\n}\n".to_string()
                }
            }
            TargetLanguage::Swift => {
                if is_trie {
                    "public final class TrieNode {\n    public var children: [Character: TrieNode] = [:]\n    public var isEnd: Bool = false\n    public init() {}\n    public func insert(_ word: String) {\n        var curr = self\n        for ch in word {\n            if curr.children[ch] == nil { curr.children[ch] = TrieNode() }\n            curr = curr.children[ch]!\n        }\n        curr.isEnd = true\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "public final class SegmentTree {\n    private var tree: [Int]\n    private let n: Int\n    public init(_ data: [Int]) {\n        n = data.count\n        tree = Array(repeating: 0, count: 4 * n)\n        if n > 0 { build(data, 1, 0, n - 1) }\n    }\n    private func build(_ data: [Int], _ node: Int, _ start: Int, _ end: Int) {\n        if start == end { tree[node] = data[start]; return }\n        let mid = (start + end) / 2\n        build(data, 2 * node, start, mid)\n        build(data, 2 * node + 1, mid + 1, end)\n        tree[node] = tree[2 * node] + tree[2 * node + 1]\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "public func dijkstra(n: Int, edges: [(u: Int, v: Int, w: Int)], start: Int) -> [Int] {\n    var dist = Array(repeating: 1_000_000, count: n)\n    dist[start] = 0\n    for _ in 0..<(n - 1) {\n        for e in edges {\n            if dist[e.u] + e.w < dist[e.v] { dist[e.v] = dist[e.u] + e.w }\n        }\n    }\n    return dist\n}\n".to_string()
                } else if is_lock_free {
                    "import Foundation\n\npublic final class AtomicStack<T> {\n    private let lock = NSLock()\n    private var storage: [T] = []\n    public init() {}\n    public func push(_ val: T) { lock.lock(); defer { lock.unlock() }; storage.append(val) }\n}\n".to_string()
                } else if is_crc32 {
                    "public func crc32Compute(_ data: [UInt8]) -> UInt32 {\n    var crc: UInt32 = 0xFFFFFFFF\n    for b in data {\n        crc ^= UInt32(b)\n        for _ in 0..<8 { crc = (crc & 1) != 0 ? ((crc >> 1) ^ 0xEDB88320) : (crc >> 1) }\n    }\n    return ~crc\n}\n".to_string()
                } else if is_rate_limiter {
                    "public actor TokenBucket {\n    private let capacity: Int\n    private var tokens: Int\n    public init(capacity: Int) { self.capacity = capacity; self.tokens = capacity }\n    public func consume(_ cost: Int) -> Bool {\n        if tokens >= cost { tokens -= cost; return true }\n        return false\n    }\n}\n".to_string()
                } else if is_simd {
                    "public func dotProductSimd(_ a: [Float], _ b: [Float]) -> Float {\n    var sum: Float = 0\n    for i in 0..<min(a.count, b.count) { sum += a[i] * b[i] }\n    return sum\n}\n".to_string()
                } else if is_ffi_mixed {
                    "@_cdecl(\"haetae_ffi_bridge\")\npublic func haetaeFfiBridge(ptr: UnsafePointer<UInt8>?, len: Int) -> Int32 {\n    guard let _ = ptr else { return 0 }\n    return Int32(len)\n}\n".to_string()
                } else if is_tree {
                    "public final class TreeNode {\n    public var val: Int\n    public var left: TreeNode?\n    public var right: TreeNode?\n\n    public init(_ val: Int, _ left: TreeNode? = nil, _ right: TreeNode? = nil) {\n        self.val = val\n        self.left = left\n        self.right = right\n    }\n}\n\npublic func inorderTraversal(_ root: TreeNode?) -> [Int] {\n    var res: [Int] = []\n    func dfs(_ node: TreeNode?) {\n        guard let node = node else { return }\n        dfs(node.left)\n        res.append(node.val)\n        dfs(node.right)\n    }\n    dfs(root)\n    return res\n}\n".to_string()
                } else if is_dp {
                    "public func knapsack01(weights: [Int], values: [Int], capacity: Int) -> Int {\n    let n = weights.count\n    var dp = Array(repeating: Array(repeating: 0, count: capacity + 1), count: n + 1)\n    for i in 1...n {\n        for w in 1...capacity {\n            if weights[i - 1] <= w {\n                dp[i][w] = max(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1])\n            } else {\n                dp[i][w] = dp[i - 1][w]\n            }\n        }\n    }\n    return dp[n][capacity]\n}\n".to_string()
                } else if is_hash {
                    "public final class SimpleCache<Key: Hashable, Value> {\n    private var storage: [Key: Value] = [:]\n    private let capacity: Int\n\n    public init(capacity: Int) {\n        self.capacity = capacity\n    }\n\n    public func get(_ key: Key) -> Value? {\n        storage[key]\n    }\n\n    public func put(_ key: Key, _ value: Value) {\n        if storage.count >= capacity {\n            if let firstKey = storage.keys.first {\n                storage.removeValue(forKey: firstKey)\n            }\n        }\n        storage[key] = value\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "public actor AsyncWorkerPool {\n    public init() {}\n    public func processItems<T: Sendable, R: Sendable>(_ items: [T], transform: @Sendable (T) async -> R) async -> [R] {\n        await withTaskGroup(of: R.self) { group in\n            for item in items {\n                group.addTask { await transform(item) }\n            }\n            var results: [R] = []\n            for await res in group { results.append(res) }\n            return results\n        }\n    }\n}\n".to_string()
                } else if is_branchless {
                    "public func branchlessSelect(mask: Int, a: Int, b: Int) -> Int {\n    (a & mask) | (b & ~mask)\n}\n".to_string()
                } else if is_divide {
                    "public func divideScalar(baseVal: Int, factor: Int) -> Int {\n    guard factor != 0 else { return 0 }\n    return baseVal / factor\n}\n".to_string()
                } else if is_socket {
                    "import Foundation\nimport Network\n\npublic final class TcpConnector {\n    public static func connect(host: String, port: UInt16) -> NWConnection {\n        let endpoint = NWEndpoint.hostPort(host: NWEndpoint.Host(host), port: NWEndpoint.Port(rawValue: port)!)\n        let conn = NWConnection(to: endpoint, using: .tcp)\n        conn.start(queue: .global())\n        return conn\n    }\n}\n".to_string()
                } else if is_sort {
                    "public func binarySearch(_ arr: [Int], _ target: Int) -> Int? {\n    var low = 0\n    var high = arr.count - 1\n    while low <= high {\n        let mid = low + (high - low) / 2\n        if arr[mid] == target { return mid }\n        else if arr[mid] < target { low = mid + 1 }\n        else { high = mid - 1 }\n    }\n    return nil\n}\n".to_string()
                } else if is_random {
                    "for i in 0...1 {\n    print(Int.random(in: 0...1))\n}\n".to_string()
                } else if is_list {
                    "let items = Array(0...1)\n".to_string()
                } else {
                    "public struct ActionModule {\n    public func execute() -> Bool { true }\n}\n"
                        .to_string()
                }
            }
            TargetLanguage::Kotlin => {
                if is_trie {
                    "class TrieNode {\n    val children = mutableMapOf<Char, TrieNode>()\n    var isEnd = false\n    fun insert(word: String) {\n        var curr = this\n        for (ch in word) {\n            curr = curr.children.computeIfAbsent(ch) { TrieNode() }\n        }\n        curr.isEnd = true\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "class SegmentTree(data: IntArray) {\n    val tree = IntArray(4 * data.size)\n    val n = data.size\n    init { if (n > 0) build(data, 1, 0, n - 1) }\n    private fun build(data: IntArray, node: Int, start: Int, end: Int) {\n        if (start == end) { tree[node] = data[start]; return }\n        val mid = (start + end) / 2\n        build(data, 2 * node, start, mid)\n        build(data, 2 * node + 1, mid + 1, end)\n        tree[node] = tree[2 * node] + tree[2 * node + 1]\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "fun dijkstra(n: Int, edges: List<Triple<Int, Int, Int>>, start: Int): IntArray {\n    val dist = IntArray(n) { 1_000_000 }\n    dist[start] = 0\n    for (i in 0 until n - 1) {\n        for ((u, v, w) in edges) {\n            if (dist[u] + w < dist[v]) dist[v] = dist[u] + w\n        }\n    }\n    return dist\n}\n".to_string()
                } else if is_lock_free {
                    "import java.util.concurrent.atomic.AtomicReference\n\nclass LockFreeStack<T> {\n    private class Node<T>(val v: T, var next: Node<T>?) \n    private val head = AtomicReference<Node<T>>()\n    fun push(v: T) {\n        val n = Node(v, null)\n        do { n.next = head.get() } while (!head.compareAndSet(n.next, n))\n    }\n}\n".to_string()
                } else if is_crc32 {
                    "fun crc32Compute(data: ByteArray): Long {\n    val crc = java.util.zip.CRC32()\n    crc.update(data)\n    return crc.value\n}\n".to_string()
                } else if is_rate_limiter {
                    "class TokenBucket(val capacity: Int, var tokens: Int) {\n    @Synchronized fun consume(cost: Int): Boolean {\n        if (tokens >= cost) { tokens -= cost; return true }\n        return false\n    }\n}\n".to_string()
                } else if is_simd {
                    "fun dotProductSimd(a: FloatArray, b: FloatArray): Float {\n    var sum = 0.0f\n    for (i in 0 until minOf(a.size, b.size)) sum += a[i] * b[i]\n    return sum\n}\n".to_string()
                } else if is_ffi_mixed {
                    "class NativeBridge {\n    external fun haetaeFfiBridge(data: ByteArray, len: Int): Int\n}\n".to_string()
                } else if is_tree {
                    "class TreeNode(var `val`: Int, var left: TreeNode? = null, var right: TreeNode? = null)\n\nfun inorderTraversal(root: TreeNode?): List<Int> {\n    val res = mutableListOf<Int>()\n    fun dfs(node: TreeNode?) {\n        if (node == null) return\n        dfs(node.left)\n        res.add(node.`val`)\n        dfs(node.right)\n    }\n    dfs(root)\n    return res\n}\n".to_string()
                } else if is_dp {
                    "fun knapsack01(weights: IntArray, values: IntArray, capacity: Int): Int {\n    val n = weights.size\n    val dp = Array(n + 1) { IntArray(capacity + 1) }\n    for (i in 1..n) {\n        for (w in 1..capacity) {\n            if (weights[i - 1] <= w) {\n                dp[i][w] = maxOf(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1])\n            } else {\n                dp[i][w] = dp[i - 1][w]\n            }\n        }\n    }\n    return dp[n][capacity]\n}\n".to_string()
                } else if is_hash {
                    "import java.util.LinkedHashMap\n\nclass LRUCache<K, V>(private val capacity: Int) : LinkedHashMap<K, V>(capacity, 0.75f, true) {\n    override fun removeEldestEntry(eldest: MutableMap.MutableEntry<K, V>?): Boolean {\n        return size > capacity\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "import kotlinx.coroutines.*\nimport kotlinx.coroutines.sync.Semaphore\nimport kotlinx.coroutines.sync.withPermit\n\nsuspend fun <T, R> parallelMap(items: List<T>, maxConcurrency: Int, transform: suspend (T) -> R): List<R> = coroutineScope {\n    val sem = Semaphore(maxConcurrency)\n    items.map { item ->\n        async {\n            sem.withPermit { transform(item) }\n        }\n    }.awaitAll()\n}\n".to_string()
                } else if is_branchless {
                    "fun branchlessSelect(mask: Int, a: Int, b: Int): Int = (a and mask) or (b and mask.inv())\n".to_string()
                } else if is_divide {
                    "fun divideScalar(baseVal: Int, factor: Int): Int = if (factor == 0) 0 else baseVal / factor\n".to_string()
                } else if is_socket {
                    "import java.net.Socket\n\nfun connectTcp(host: String, port: Int, retries: Int): Socket? {\n    for (i in 0 until retries) {\n        try {\n            return Socket(host, port)\n        } catch (_: Exception) {}\n    }\n    return null\n}\n".to_string()
                } else if is_sort {
                    "fun binarySearch(arr: IntArray, target: Int): Int {\n    var low = 0\n    var high = arr.size - 1\n    while (low <= high) {\n        val mid = low + (high - low) / 2\n        if (arr[mid] == target) return mid\n        else if (arr[mid] < target) low = mid + 1\n        else high = mid - 1\n    }\n    return -1\n}\n".to_string()
                } else if is_random {
                    "fun main() {\n    for (i in 0..1) {\n        println((0..1).random())\n    }\n}\n".to_string()
                } else if is_list {
                    "val items = (0..1).toList()\n".to_string()
                } else {
                    "class ActionHandler {\n    fun execute(): Boolean = true\n}\n".to_string()
                }
            }
            TargetLanguage::Php => {
                if is_trie {
                    "<?php\nclass TrieNode {\n    public array $children = [];\n    public bool $isEnd = false;\n    public function insert(string $word): void {\n        $curr = $this;\n        for ($i = 0; $i < strlen($word); $i++) {\n            $ch = $word[$i];\n            if (!isset($curr->children[$ch])) $curr->children[$ch] = new TrieNode();\n            $curr = $curr->children[$ch];\n        }\n        $curr->isEnd = true;\n    }\n}\n".to_string()
                } else if is_segment_tree {
                    "<?php\nclass SegmentTree {\n    public array $tree;\n    public int $n;\n    public function __construct(array $data) {\n        $this->n = count($data);\n        $this->tree = array_fill(0, 4 * $this->n, 0);\n        if ($this->n > 0) $this->build($data, 1, 0, $this->n - 1);\n    }\n    private function build(array $data, int $node, int $l, int $r): void {\n        if ($l === $r) { $this->tree[$node] = $data[$l]; return; }\n        $mid = intdiv($l + $r, 2);\n        $this->build($data, 2 * $node, $l, $mid);\n        $this->build($data, 2 * $node + 1, $mid + 1, $r);\n        $this->tree[$node] = $this->tree[2 * $node] + $this->tree[2 * $node + 1];\n    }\n}\n".to_string()
                } else if is_dijkstra {
                    "<?php\nfunction dijkstra(int $n, array $edges, int $start): array {\n    $dist = array_fill(0, $n, 1000000);\n    $dist[$start] = 0;\n    for ($i = 0; $i < $n - 1; $i++) {\n        foreach ($edges as [$u, $v, $w]) {\n            if ($dist[$u] + $w < $dist[$v]) $dist[$v] = $dist[$u] + $w;\n        }\n    }\n    return $dist;\n}\n".to_string()
                } else if is_lock_free {
                    "<?php\nclass AtomicStack {\n    private array $items = [];\n    public function push($val): void { array_unshift($this->items, $val); }\n    public function pop() { return array_shift($this->items); }\n}\n".to_string()
                } else if is_crc32 {
                    "<?php\nfunction crc32Compute(string $data): int {\n    return crc32($data);\n}\n".to_string()
                } else if is_rate_limiter {
                    "<?php\nclass TokenBucket {\n    public int $capacity;\n    public int $tokens;\n    public function __construct(int $cap) { $this->capacity = $cap; $this->tokens = $cap; }\n    public function consume(int $cost): bool {\n        if ($this->tokens >= $cost) { $this->tokens -= $cost; return true; }\n        return false;\n    }\n}\n".to_string()
                } else if is_simd {
                    "<?php\nfunction dotProductSimd(array $a, array $b): float {\n    $sum = 0.0;\n    for ($i = 0; $i < count($a) && $i < count($b); $i++) $sum += $a[$i] * $b[$i];\n    return $sum;\n}\n".to_string()
                } else if is_ffi_mixed {
                    "<?php\nfunction callNativeBridge(string $data): int {\n    return strlen($data);\n}\n".to_string()
                } else if is_tree {
                    "<?php\nclass TreeNode {\n    public int $val;\n    public ?TreeNode $left;\n    public ?TreeNode $right;\n\n    public function __construct(int $val = 0, ?TreeNode $left = null, ?TreeNode $right = null) {\n        $this->val = $val;\n        $this->left = $left;\n        $this->right = $right;\n    }\n}\n\nfunction inorderTraversal(?TreeNode $root): array {\n    $res = [];\n    $dfs = function(?TreeNode $node) use (&$dfs, &$res) {\n        if ($node === null) return;\n        $dfs($node->left);\n        $res[] = $node->val;\n        $dfs($node->right);\n    };\n    $dfs($root);\n    return $res;\n}\n".to_string()
                } else if is_dp {
                    "<?php\nfunction knapsack01(array $weights, array $values, int $capacity): int {\n    $n = count($weights);\n    $dp = array_fill(0, $n + 1, array_fill(0, $capacity + 1, 0));\n    for ($i = 1; $i <= $n; $i++) {\n        for ($w = 1; $w <= capacity; $w++) {\n            if ($weights[$i - 1] <= w) {\n                $dp[$i][$w] = max($dp[$i - 1][$w], dp[$i - 1][$w - weights[$i - 1]] + values[$i - 1]);\n            } else {\n                $dp[$i][$w] = dp[$i - 1][w];\n            }\n        }\n    }\n    return dp[n][capacity];\n}\n".to_string()
                } else if is_hash {
                    "<?php\nclass SimpleCache {\n    private array $data = [];\n    private int $capacity;\n\n    public function __construct(int $capacity) {\n        $this->capacity = $capacity;\n    }\n\n    public function get(string $key) {\n        return $this->data[$key] ?? null;\n    }\n\n    public function put(string $key, $val): void {\n        if (count($this->data) >= $this->capacity) {\n            array_shift($this->data);\n        }\n        $this->data[$key] = $val;\n    }\n}\n".to_string()
                } else if is_concurrency {
                    "<?php\nfunction executeConcurrentHttp(array $urls): array {\n    $mh = curl_multi_init();\n    $handles = [];\n    foreach ($urls as $url) {\n        $ch = curl_init($url);\n        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);\n        curl_multi_add_handle($mh, $ch);\n        $handles[] = $ch;\n    }\n    $running = null;\n    do {\n        curl_multi_exec($mh, $running);\n    } while ($running > 0);\n    $results = [];\n    foreach ($handles as $ch) {\n        $results[] = curl_multi_getcontent($ch);\n        curl_multi_remove_handle($mh, $ch);\n    }\n    curl_multi_close($mh);\n    return $results;\n}\n".to_string()
                } else if is_branchless {
                    "<?php\nfunction branchlessSelect(int $mask, int $a, int $b): int {\n    return ($a & $mask) | ($b & ~$mask);\n}\n".to_string()
                } else if is_divide {
                    "<?php\nfunction divideScalar(int $baseVal, int $factor): int {\n    if ($factor === 0) return 0;\n    return intdiv($baseVal, $factor);\n}\n".to_string()
                } else if is_socket {
                    "<?php\nfunction connectSocketRetry(string $host, int $port, int $retries) {\n    for ($i = 0; $i < $retries; $i++) {\n        $sock = @fsockopen($host, $port, $errno, $errstr, 2);\n        if ($sock) return $sock;\n    }\n    return false;\n}\n".to_string()
                } else if is_sort {
                    "<?php\nfunction binarySearch(array $arr, int $target): int {\n    $low = 0;\n    $high = count($arr) - 1;\n    while ($low <= high) {\n        $mid = intdiv($low + $high, 2);\n        if ($arr[$mid] === $target) return $mid;\n        if ($arr[$mid] < target) $low = $mid + 1;\n        else $high = mid - 1;\n    }\n    return -1;\n}\n".to_string()
                } else if is_random {
                    "<?php\nfor ($i = 0; $i < 1; $i++) {\n    echo rand(0, 1) . \"\\n\";\n}\n"
                        .to_string()
                } else if is_list {
                    "<?php\n$items = range(0, 1);\n".to_string()
                } else {
                    "<?php\nfunction executeAction(): bool {\n    return true;\n}\n".to_string()
                }
            }
            TargetLanguage::Sql => {
                if is_trie {
                    "WITH RECURSIVE TriePaths AS (\n    SELECT id, parent_id, char_val, is_end FROM trie_nodes WHERE parent_id IS NULL\n    UNION ALL\n    SELECT t.id, t.parent_id, t.char_val, t.is_end FROM trie_nodes t INNER JOIN TriePaths tp ON t.parent_id = tp.id\n)\nSELECT * FROM TriePaths;\n".to_string()
                } else if is_segment_tree {
                    "WITH RECURSIVE SegTree AS (SELECT 1 as node, 0 as l, 100 as r UNION ALL SELECT node * 2, l, (l + r) / 2 FROM SegTree WHERE l < r)\nSELECT * FROM SegTree LIMIT 10;\n".to_string()
                } else if is_dijkstra {
                    "WITH RECURSIVE ShortestPath AS (\n    SELECT src, dest, cost FROM network_graph WHERE src = :start\n    UNION ALL\n    SELECT sp.src, g.dest, sp.cost + g.cost FROM ShortestPath sp JOIN network_graph g ON sp.dest = g.src WHERE sp.cost + g.cost < 1000\n)\nSELECT dest, MIN(cost) FROM ShortestPath GROUP BY dest;\n".to_string()
                } else if is_lock_free {
                    "SELECT id, payload FROM atomic_queue WHERE status = 'READY' ORDER BY id ASC LIMIT 1 FOR UPDATE SKIP LOCKED;\n".to_string()
                } else if is_crc32 {
                    "SELECT id, hashtext(payload) AS crc_checksum FROM payloads;\n".to_string()
                } else if is_rate_limiter {
                    "UPDATE client_rate_limits SET tokens = tokens - 1 WHERE client_id = :id AND tokens > 0;\n".to_string()
                } else if is_simd {
                    "SELECT SUM(a.val * b.val) AS dot_product FROM vec_a a JOIN vec_b b ON a.idx = b.idx;\n".to_string()
                } else if is_ffi_mixed {
                    "SELECT haetae_ffi_bridge(payload_bytes, LENGTH(payload_bytes)) FROM records;\n"
                        .to_string()
                } else if is_tree {
                    "WITH RECURSIVE TreeTraversal AS (\n    SELECT id, parent_id, val, 1 AS depth\n    FROM tree_nodes\n    WHERE parent_id IS NULL\n    UNION ALL\n    SELECT t.id, t.parent_id, t.val, tt.depth + 1\n    FROM tree_nodes t\n    INNER JOIN TreeTraversal tt ON t.parent_id = tt.id\n)\nSELECT * FROM TreeTraversal ORDER BY depth, id;\n".to_string()
                } else if is_dp {
                    "SELECT\n    item_id,\n    weight,\n    value,\n    SUM(value) OVER (ORDER BY weight ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS cumulative_value\nFROM knapsack_candidates;\n".to_string()
                } else if is_hash {
                    "CREATE TABLE IF NOT EXISTS kv_cache (\n    cache_key VARCHAR(255) PRIMARY KEY,\n    cache_value TEXT NOT NULL,\n    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n);\n\nINSERT INTO kv_cache (cache_key, cache_value, updated_at)\nVALUES (:key, :val, CURRENT_TIMESTAMP)\nON CONFLICT (cache_key) DO UPDATE\nSET cache_value = EXCLUDED.cache_value, updated_at = CURRENT_TIMESTAMP;\n".to_string()
                } else if is_concurrency {
                    "BEGIN TRANSACTION;\nSELECT task_id, payload\nFROM task_queue\nWHERE status = 'PENDING'\nORDER BY priority DESC, created_at ASC\nLIMIT 10\nFOR UPDATE SKIP LOCKED;\n\nUPDATE task_queue\nSET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP\nWHERE task_id IN (SELECT task_id FROM task_queue WHERE status = 'PENDING' LIMIT 10);\nCOMMIT;\n".to_string()
                } else if is_branchless {
                    "SELECT id, (a & mask) | (b & ~mask) AS branchless_val FROM bitwise_table;\n"
                        .to_string()
                } else if is_divide {
                    "SELECT id, CASE WHEN factor = 0 THEN 0 ELSE base_val / factor END AS safe_division FROM computation_data;\n".to_string()
                } else if is_socket {
                    "SELECT host, port, timeout_ms, max_retries FROM network_endpoints WHERE is_active = 1;\n".to_string()
                } else if is_sort {
                    "SELECT id, val FROM indexed_records WHERE val >= :target ORDER BY val ASC LIMIT 1;\n".to_string()
                } else if is_random {
                    "SELECT FLOOR(RAND()) AS random_num;\n".to_string()
                } else if is_list {
                    "SELECT unnest(ARRAY[0, 1]) AS item;\n".to_string()
                } else {
                    "SELECT 1 AS status;\n".to_string()
                }
            }
            TargetLanguage::Bash => {
                if is_trie {
                    "trie_insert() {\n    local word=\"$1\"\n    echo \"inserting $word into bash prefix store\"\n}\n".to_string()
                } else if is_segment_tree {
                    "segtree_build() {\n    local -a arr=(\"$@\")\n    echo \"building tree of size ${#arr[@]}\"\n}\n".to_string()
                } else if is_dijkstra {
                    "dijkstra_path() {\n    local start=\"$1\"\n    echo \"computing shortest path from $start\"\n}\n".to_string()
                } else if is_lock_free {
                    "atomic_push() {\n    local val=\"$1\"\n    mkdir /tmp/haetae_lock 2>/dev/null && echo \"$val\" >> /tmp/stack && rmdir /tmp/haetae_lock\n}\n".to_string()
                } else if is_crc32 {
                    "crc32_calc() {\n    cksum \"$1\" | awk '{print $1}'\n}\n".to_string()
                } else if is_rate_limiter {
                    "token_bucket() {\n    local cost=\"$1\"\n    [ \"$TOKENS\" -ge \"$cost\" ] && TOKENS=$((TOKENS - cost)) && return 0 || return 1\n}\n".to_string()
                } else if is_simd {
                    "dot_product() {\n    local sum=0\n    echo \"$sum\"\n}\n".to_string()
                } else if is_ffi_mixed {
                    "haetae_ffi_bridge() {\n    [ -z \"$1\" ] && return 1 || echo ${#1}\n}\n"
                        .to_string()
                } else if is_tree {
                    "traverse_tree() {\n    local dir=\"$1\"\n    for item in \"$dir\"/[*]; do\n        if [ -d \"$item\" ]; then\n            traverse_tree \"$item\"\n        elif [ -f \"$item\" ]; then\n            echo \"$item\"\n        fi\n    done\n}\n".to_string()
                } else if is_dp {
                    "fib_dp() {\n    local n=$1\n    local -a dp=(0 1)\n    for ((i=2; i<=n; i++)); do\n        dp[i]=$((dp[i-1] + dp[i-2]))\n    done\n    echo \"${dp[n]}\"\n}\n".to_string()
                } else if is_hash {
                    "declare -A LRU_CACHE\n\ncache_put() {\n    local key=\"$1\"\n    local val=\"$2\"\n    LRU_CACHE[\"$key\"]=\"$val\"\n}\n\ncache_get() {\n    local key=\"$1\"\n    echo \"${LRU_CACHE[$key]}\"\n}\n".to_string()
                } else if is_concurrency {
                    "run_parallel_pool() {\n    local max_jobs=4\n    for job in \"$@\"; do\n        while [ $(jobs -r | wc -l) -ge $max_jobs ]; do\n            sleep 0.1\n        done\n        eval \"$job\" &\n    done\n    wait\n}\n".to_string()
                } else if is_branchless {
                    "branchless_select() {\n    local mask=$1\n    local a=$2\n    local b=$3\n    echo \"$(( (a & mask) | (b & ~mask) ))\"\n}\n".to_string()
                } else if is_divide {
                    "divide_scalar() {\n    local base_val=$1\n    local factor=$2\n    if [ \"$factor\" -eq 0 ]; then\n        echo 0\n    else\n        echo \"$((base_val / factor))\"\n    fi\n}\n".to_string()
                } else if is_socket {
                    "check_tcp_port() {\n    local host=\"$1\"\n    local port=\"$2\"\n    nc -z -w 2 \"$host\" \"$port\"\n    return $?\n}\n".to_string()
                } else if is_sort {
                    "binary_search() {\n    local target=$1\n    shift\n    local arr=(\"$@\")\n    local low=0\n    local high=$((${#arr[@]} - 1))\n    while [ $low -le $high ]; do\n        local mid=$((low + (high - low) / 2))\n        if [ \"${arr[mid]}\" -eq \"$target\" ]; then\n            echo \"$mid\"\n            return 0\n        elif [ \"${arr[mid]}\" -lt \"$target\" ]; then\n            low=$((mid + 1))\n        else\n            high=$((mid - 1))\n        fi\n    done\n    echo \"-1\"\n    return 1\n}\n".to_string()
                } else if is_random {
                    "for i in $(seq 0 1); do\n    echo $((RANDOM & 1))\ndone\n".to_string()
                } else if is_list {
                    "items=(0 1)\n".to_string()
                } else {
                    "execute_action() {\n    return 0\n}\n".to_string()
                }
            }
            TargetLanguage::HtmlCss => {
                if is_trie {
                    "<div class=\"trie-visualization\"><div class=\"trie-node\" data-char=\"root\"></div></div>\n<style>\n.trie-visualization { display: flex; flex-direction: column; gap: 8px; }\n.trie-node { padding: 4px 8px; border: 1px solid #333; border-radius: 4px; }\n</style>\n".to_string()
                } else if is_segment_tree {
                    "<div class=\"segment-tree-layout\"><div class=\"tree-interval\">0..100</div></div>\n<style>\n.segment-tree-layout { display: grid; gap: 4px; }\n.tree-interval { background: #eee; padding: 6px; }\n</style>\n".to_string()
                } else if is_dijkstra {
                    "<div class=\"graph-routing-card\"><span class=\"shortest-path-dist\">Distance: 0</span></div>\n<style>\n.graph-routing-card { padding: 12px; border: 1px solid #aaa; border-radius: 6px; }\n</style>\n".to_string()
                } else if is_lock_free {
                    "<div class=\"atomic-stack-monitor\"><span class=\"cas-status\">SYNC</span></div>\n<style>\n.atomic-stack-monitor { font-family: monospace; padding: 8px; background: #222; color: #0f0; }\n</style>\n".to_string()
                } else if is_crc32 {
                    "<div class=\"checksum-badge\"><span class=\"crc-val\">0xEDB88320</span></div>\n<style>\n.checksum-badge { padding: 4px 8px; background: #f0f0f0; border: 1px solid #ccc; font-family: monospace; }\n</style>\n".to_string()
                } else if is_rate_limiter {
                    "<div class=\"rate-limiter-gauge\"><span class=\"token-level\">100% Capacity</span></div>\n<style>\n.rate-limiter-gauge { width: 100%; background: #e0e0e0; border-radius: 4px; padding: 4px; }\n</style>\n".to_string()
                } else if is_simd {
                    "<div class=\"simd-vector-view\"><span class=\"lane\">[1.0, 2.0, 3.0, 4.0]</span></div>\n<style>\n.simd-vector-view { display: flex; gap: 4px; font-family: monospace; }\n</style>\n".to_string()
                } else if is_ffi_mixed {
                    "<div class=\"ffi-boundary\"><span class=\"bridge-status\">C-ABI Connected</span></div>\n<style>\n.ffi-boundary { border: 2px dashed #666; padding: 8px; text-align: center; }\n</style>\n".to_string()
                } else if is_tree {
                    "<div class=\"tree-container\">\n    <ul class=\"tree-root\">\n        <li class=\"tree-node\">\n            <span class=\"node-label\">Root</span>\n            <ul class=\"tree-children\">\n                <li class=\"tree-node\"><span class=\"node-label\">Left</span></li>\n                <li class=\"tree-node\"><span class=\"node-label\">Right</span></li>\n            </ul>\n        </li>\n    </ul>\n</div>\n<style>\n.tree-container { font-family: monospace; padding: 16px; }\n.tree-root, .tree-children { list-style: none; padding-left: 20px; }\n.tree-node { position: relative; margin: 4px 0; }\n.node-label { background: #f0f0f0; padding: 2px 6px; border-radius: 4px; }\n</style>\n".to_string()
                } else if is_dp {
                    "<div class=\"dp-matrix-container\">\n    <table class=\"dp-table\">\n        <thead>\n            <tr><th>i \\ w</th><th>0</th><th>1</th><th>2</th><th>3</th></tr>\n        </thead>\n        <tbody>\n            <tr><td>0</td><td>0</td><td>0</td><td>0</td><td>0</td></tr>\n            <tr><td>1</td><td>0</td><td>2</td><td>2</td><td>2</td></tr>\n        </tbody>\n    </table>\n</div>\n<style>\n.dp-table { border-collapse: collapse; width: 100%; }\n.dp-table th, .dp-table td { border: 1px solid #ccc; padding: 8px; text-align: center; }\n</style>\n".to_string()
                } else if is_hash {
                    "<div class=\"kv-store-card\">\n    <div class=\"kv-header\">Cache Entries</div>\n    <div class=\"kv-row\"><span class=\"kv-key\">session_token</span><span class=\"kv-val\">0xAF91B2</span></div>\n</div>\n<style>\n.kv-store-card { border: 1px solid #ddd; border-radius: 8px; padding: 12px; }\n.kv-row { display: flex; justify-content: space-between; padding: 4px 0; }\n</style>\n".to_string()
                } else if is_concurrency {
                    "<div class=\"worker-dashboard\">\n    <div class=\"worker-status active\">Worker 1: Running</div>\n    <div class=\"worker-status idle\">Worker 2: Idle</div>\n</div>\n<style>\n.worker-dashboard { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }\n.worker-status { padding: 8px; border-radius: 4px; font-weight: bold; }\n.worker-status.active { background: #d4edda; color: #155724; }\n.worker-status.idle { background: #f8f9fa; color: #6c757d; }\n</style>\n".to_string()
                } else if is_branchless {
                    "<div class=\"branchless-toggle\">\n    <input type=\"checkbox\" id=\"mask-toggle\" />\n    <label for=\"mask-toggle\">Bitmask State Selector</label>\n</div>\n".to_string()
                } else if is_divide {
                    "<div class=\"division-guard\">\n    <span class=\"val-display\">Division Guard Active</span>\n</div>\n".to_string()
                } else if is_socket {
                    "<div class=\"network-connection-card\">\n    <span class=\"status-indicator online\"></span>\n    <span class=\"endpoint\">tcp://127.0.0.1:8080</span>\n</div>\n".to_string()
                } else if is_sort {
                    "<div class=\"search-widget\">\n    <input type=\"text\" class=\"search-field\" placeholder=\"Binary Search Query...\" />\n</div>\n".to_string()
                } else if is_random {
                    "<div id=\"random-output\">\n    <span class=\"rand-val\">1</span>\n</div>\n"
                        .to_string()
                } else if is_list {
                    "<ul class=\"item-list\">\n    <li>Item 0</li>\n    <li>Item 1</li>\n</ul>\n"
                        .to_string()
                } else {
                    "<div class=\"haetae-container\">\n    <span class=\"status-ready\">Ready</span>\n</div>\n".to_string()
                }
            }
        }
    }

    pub fn refactor_code(&self, target_code: &str, prompt: &str) -> String {
        let p_lower = prompt.to_lowercase();
        let is_branchless = p_lower.contains("branchless")
            || p_lower.contains("브랜치리스")
            || p_lower.contains("비트")
            || p_lower.contains("bitwise")
            || p_lower.contains("optimize")
            || p_lower.contains("zero alloc");
        let is_result = p_lower.contains("result")
            || p_lower.contains("option")
            || p_lower.contains("모나드")
            || p_lower.contains("에러")
            || p_lower.contains("error")
            || p_lower.contains("try")
            || p_lower.contains("functional")
            || p_lower.contains("monad");
        let is_async = p_lower.contains("async")
            || p_lower.contains("비동기")
            || p_lower.contains("concurren")
            || p_lower.contains("동시성");
        let is_stream = p_lower.contains("stream")
            || p_lower.contains("iterator")
            || p_lower.contains("함수형")
            || p_lower.contains("functional")
            || p_lower.contains("comprehension");
        let is_guard = p_lower.contains("guard")
            || p_lower.contains("가드")
            || p_lower.contains("null")
            || p_lower.contains("널");
        let is_crc =
            p_lower.contains("crc") || p_lower.contains("checksum") || p_lower.contains("체크섬");
        let is_bucket = p_lower.contains("bucket")
            || p_lower.contains("rate limit")
            || p_lower.contains("속도 제한");
        let is_cas = p_lower.contains("cas")
            || p_lower.contains("atomic")
            || p_lower.contains("compare and swap")
            || p_lower.contains("원자적");
        let is_div_guard = p_lower.contains("div")
            || p_lower.contains("나눗셈")
            || p_lower.contains("invariant")
            || p_lower.contains("불변식");
        let is_list = p_lower.contains("리스트")
            || p_lower.contains("배열")
            || p_lower.contains("list")
            || p_lower.contains("array");

        match self {
            TargetLanguage::Python => {
                if is_branchless {
                    "val = (a & mask) | (b & ~mask)\n".to_string()
                } else if is_result {
                    "try:\n    val = int(target)\nexcept (ValueError, TypeError):\n    val = None\n"
                        .to_string()
                } else if is_async {
                    "async def async_process(item):\n    await asyncio.sleep(0)\n    return item\n"
                        .to_string()
                } else if is_stream {
                    "result = [x * 2 for x in items if x > 0]\n".to_string()
                } else if is_guard {
                    "if not target:\n    return None\n".to_string()
                } else if is_crc {
                    "crc = zlib.crc32(payload) & 0xFFFFFFFF\n".to_string()
                } else if is_bucket {
                    "if not token_bucket.consume(1):\n    raise RuntimeError('Rate limited')\n"
                        .to_string()
                } else if is_cas {
                    "while not atomic_ref.compare_and_set(cur, new_val):\n    cur = atomic_ref.get()\n".to_string()
                } else if is_div_guard {
                    "if factor == 0:\n    return 0\nval = base_val // factor\n".to_string()
                } else if is_list {
                    "numbers = [random.randint(0, 1) for _ in range(0, 1)]\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Rust => {
                if is_branchless {
                    "let val = (a & mask) | (b & !mask);\n".to_string()
                } else if is_result {
                    "fn parse_val(input: &str) -> Result<i32, &'static str> {\n    input.parse::<i32>().map_err(|_| \"invalid number\")\n}\n".to_string()
                } else if is_async {
                    "async fn async_process(item: i32) -> Result<i32, Box<dyn std::error::Error>> {\n    Ok(item)\n}\n".to_string()
                } else if is_stream {
                    "let result: Vec<_> = items.iter().filter(|&&x| x > 0).map(|&x| x * 2).collect();\n".to_string()
                } else if is_guard {
                    "let target = target.as_ref()?;\n".to_string()
                } else if is_crc {
                    "let crc = crc32_compute(&payload);\n".to_string()
                } else if is_bucket {
                    "if !token_bucket.consume(1) { return Err(\"rate limit exceeded\"); }\n"
                        .to_string()
                } else if is_cas {
                    "while atomic_val.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed).is_err() { cur = atomic_val.load(Ordering::Relaxed); }\n".to_string()
                } else if is_div_guard {
                    "if factor == 0 { return Err(\"division by zero\"); } Ok(base_val / factor)\n"
                        .to_string()
                } else if is_list {
                    "let numbers: Vec<i32> = (0..1).collect();\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::C => {
                if is_branchless {
                    "val = (a & mask) | (b & ~mask);\n".to_string()
                } else if is_result {
                    "int parse_val(const char* input, int* out_val) {\n    if (!input || !out_val) return 0;\n    *out_val = atoi(input);\n    return 1;\n}\n".to_string()
                } else if is_async {
                    "void* thread_worker(void* arg) { return arg; }\n".to_string()
                } else if is_stream {
                    "for (size_t i = 0; i < len; ++i) { if (arr[i] > 0) out[count++] = arr[i] * 2; }\n".to_string()
                } else if is_guard {
                    "if (!ptr) return 0;\n".to_string()
                } else if is_crc {
                    "unsigned int crc = crc32_compute(data, len);\n".to_string()
                } else if is_bucket {
                    "if (!token_bucket_consume(tb, 1)) return 0;\n".to_string()
                } else if is_cas {
                    "while (!__sync_bool_compare_and_swap(&state, old_val, new_val)) { old_val = state; }\n".to_string()
                } else if is_div_guard {
                    "if (factor == 0) return 0; return base_val / factor;\n".to_string()
                } else if is_list {
                    "int numbers[] = {0, 1};\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Cpp => {
                if is_branchless {
                    "val = (a & mask) | (b & ~mask);\n".to_string()
                } else if is_result {
                    "#include <optional>\n#include <string>\n\nstd::optional<int> parse_val(const std::string& input) {\n    try { return std::stoi(input); } catch (...) { return std::nullopt; }\n}\n".to_string()
                } else if is_async {
                    "std::future<int> async_process(int val) { return std::async(std::launch::async, [val] { return val; }); }\n".to_string()
                } else if is_stream {
                    "std::vector<int> res;\nfor (int x : items) if (x > 0) res.push_back(x * 2);\n"
                        .to_string()
                } else if is_guard {
                    "if (!ptr) return nullptr;\n".to_string()
                } else if is_crc {
                    "uint32_t crc = crc32_compute(data);\n".to_string()
                } else if is_bucket {
                    "if (!tb.consume(1)) return false;\n".to_string()
                } else if is_cas {
                    "while (!head.compare_exchange_weak(cur, next));\n".to_string()
                } else if is_div_guard {
                    "if (factor == 0) return 0; return base_val / factor;\n".to_string()
                } else if is_list {
                    "std::vector<int> numbers = {0, 1};\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Go => {
                if is_branchless {
                    "val := (a & mask) | (b & ^mask)\n".to_string()
                } else if is_result {
                    "func ParseVal(input string) (int, error) {\n    return strconv.Atoi(input)\n}\n".to_string()
                } else if is_async {
                    "go func(item int) { results <- item }(val)\n".to_string()
                } else if is_stream {
                    "var res []int\nfor _, x := range items { if x > 0 { res = append(res, x*2) } }\n".to_string()
                } else if is_guard {
                    "if ptr == nil {\n    return nil\n}\n".to_string()
                } else if is_crc {
                    "crc := crc32.ChecksumIEEE(payload)\n".to_string()
                } else if is_bucket {
                    "if !tb.Consume(1) { return errors.New(\"rate limit\") }\n".to_string()
                } else if is_cas {
                    "for !atomic.CompareAndSwapPointer(&head, cur, next) { cur = atomic.LoadPointer(&head) }\n".to_string()
                } else if is_div_guard {
                    "if factor == 0 { return 0, errors.New(\"division by zero\") }\nreturn baseVal / factor, nil\n".to_string()
                } else if is_list {
                    "numbers := []int{0, 1}\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::JavaScript | TargetLanguage::TypeScript => {
                let is_ts = *self == TargetLanguage::TypeScript;
                if is_branchless {
                    "const val = (a & mask) | (b & ~mask);\n".to_string()
                } else if is_result {
                    if is_ts {
                        "function parseVal(input: string): number | null {\n    const num = Number(input);\n    return isNaN(num) ? null : num;\n}\n".to_string()
                    } else {
                        "function parseVal(input) {\n    const num = Number(input);\n    return isNaN(num) ? null : num;\n}\n".to_string()
                    }
                } else if is_async {
                    "async function asyncProcess(item) { return Promise.resolve(item); }\n"
                        .to_string()
                } else if is_stream {
                    "const res = items.filter(x => x > 0).map(x => x * 2);\n".to_string()
                } else if is_guard {
                    "if (!target) return null;\n".to_string()
                } else if is_crc {
                    "const crc = crc32Compute(payload);\n".to_string()
                } else if is_bucket {
                    "if (!bucket.consume(1)) throw new Error('Rate limit');\n".to_string()
                } else if is_cas {
                    "while (!atomicCompareSwap(cur, next)) { cur = getCur(); }\n".to_string()
                } else if is_div_guard {
                    "if (factor === 0) return 0; return Math.floor(baseVal / factor);\n".to_string()
                } else if is_list {
                    "const items = [0, 1];\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Java => {
                if is_branchless {
                    "int val = (a & mask) | (b & ~mask);\n".to_string()
                } else if is_result {
                    "public static Optional<Integer> parseVal(String input) {\n    try { return Optional.of(Integer.parseInt(input)); } catch (NumberFormatException e) { return Optional.empty(); }\n}\n".to_string()
                } else if is_async {
                    "CompletableFuture.supplyAsync(() -> item);\n".to_string()
                } else if is_stream {
                    "List<Integer> res = items.stream().filter(x -> x > 0).map(x -> x * 2).toList();\n".to_string()
                } else if is_guard {
                    "if (target == null) return Optional.empty();\n".to_string()
                } else if is_crc {
                    "long crc = Crc32Util.compute(payload);\n".to_string()
                } else if is_bucket {
                    "if (!tb.consume(1)) throw new RuntimeException(\"Rate limit\");\n".to_string()
                } else if is_cas {
                    "while (!head.compareAndSet(cur, next)) { cur = head.get(); }\n".to_string()
                } else if is_div_guard {
                    "if (factor == 0) return 0; return baseVal / factor;\n".to_string()
                } else if is_list {
                    "int[] numbers = {0, 1};\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::CSharp => {
                if is_branchless {
                    "int val = (a & mask) | (b & ~mask);\n".to_string()
                } else if is_result {
                    "public static bool TryParseVal(string input, out int result) => int.TryParse(input, out result);\n".to_string()
                } else if is_async {
                    "await Task.Run(() => item);\n".to_string()
                } else if is_stream {
                    "var res = items.Where(x => x > 0).Select(x => x * 2).ToList();\n".to_string()
                } else if is_guard {
                    "if (target is null) return null;\n".to_string()
                } else if is_crc {
                    "uint crc = Crc32Util.Compute(payload);\n".to_string()
                } else if is_bucket {
                    "if (!tb.Consume(1)) throw new InvalidOperationException(\"Rate limit\");\n"
                        .to_string()
                } else if is_cas {
                    "while (Interlocked.CompareExchange(ref head, next, cur) != cur) { cur = head; }\n".to_string()
                } else if is_div_guard {
                    "if (factor == 0) return 0; return baseVal / factor;\n".to_string()
                } else if is_list {
                    "int[] items = { 0, 1 };\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Swift => {
                if is_branchless {
                    "let val = (a & mask) | (b & ~mask)\n".to_string()
                } else if is_result {
                    "func parseVal(_ input: String) -> Int? { Int(input) }\n".to_string()
                } else if is_async {
                    "Task { await process(item) }\n".to_string()
                } else if is_stream {
                    "let res = items.filter { $0 > 0 }.map { $0 * 2 }\n".to_string()
                } else if is_guard {
                    "guard let target = target else { return nil }\n".to_string()
                } else if is_crc {
                    "let crc = crc32Compute(payload)\n".to_string()
                } else if is_bucket {
                    "guard await tb.consume(1) else { throw RateLimitError() }\n".to_string()
                } else if is_cas {
                    "while !compareAndSwap(&head, cur, next) { cur = head }\n".to_string()
                } else if is_div_guard {
                    "guard factor != 0 else { return 0 }; return baseVal / factor\n".to_string()
                } else if is_list {
                    "let items = Array(0...1)\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Kotlin => {
                if is_branchless {
                    "val result = (a and mask) or (b and mask.inv())\n".to_string()
                } else if is_result {
                    "fun parseVal(input: String): Int? = input.toIntOrNull()\n".to_string()
                } else if is_async {
                    "coroutineScope { launch { process(item) } }\n".to_string()
                } else if is_stream {
                    "val res = items.filter { it > 0 }.map { it * 2 }\n".to_string()
                } else if is_guard {
                    "val nonNullTarget = target ?: return null\n".to_string()
                } else if is_crc {
                    "val crc = crc32Compute(payload)\n".to_string()
                } else if is_bucket {
                    "if (!tb.consume(1)) error(\"Rate limit\")\n".to_string()
                } else if is_cas {
                    "while (!head.compareAndSet(cur, next)) { cur = head.get() }\n".to_string()
                } else if is_div_guard {
                    "if (factor == 0) return 0 else baseVal / factor\n".to_string()
                } else if is_list {
                    "val items = (0..1).toList()\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Php => {
                if is_branchless {
                    "$val = ($a & $mask) | ($b & ~$mask);\n".to_string()
                } else if is_result {
                    "function parseVal(string $input): ?int { return is_numeric($input) ? (int)$input : null; }\n".to_string()
                } else if is_async {
                    "$fiber = new Fiber(function() use ($item) { Fiber::suspend($item); });\n"
                        .to_string()
                } else if is_stream {
                    "$res = array_map(fn($x) => $x * 2, array_filter($items, fn($x) => $x > 0));\n"
                        .to_string()
                } else if is_guard {
                    "if ($target === null) return null;\n".to_string()
                } else if is_crc {
                    "$crc = crc32($payload);\n".to_string()
                } else if is_bucket {
                    "if (!$tb->consume(1)) throw new Exception(\"Rate limit\");\n".to_string()
                } else if is_cas {
                    "while (!atomic_cas($head, $cur, $next)) { $cur = $head; }\n".to_string()
                } else if is_div_guard {
                    "if ($factor === 0) return 0; return intdiv($baseVal, $factor);\n".to_string()
                } else if is_list {
                    "$items = range(0, 1);\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Sql => {
                if is_branchless {
                    "SELECT (a & mask) | (b & ~mask) AS val FROM table_name;\n".to_string()
                } else if is_result {
                    "SELECT CASE WHEN val ~ '^[0-9]+$' THEN CAST(val AS INTEGER) ELSE NULL END FROM dataset;\n".to_string()
                } else if is_async {
                    "SELECT pg_notify('task_channel', 'payload');\n".to_string()
                } else if is_stream {
                    "SELECT val * 2 FROM dataset WHERE val > 0;\n".to_string()
                } else if is_guard {
                    "WHERE target IS NOT NULL\n".to_string()
                } else if is_crc {
                    "SELECT hashtext(payload) AS crc FROM table_name;\n".to_string()
                } else if is_bucket {
                    "UPDATE limits SET tokens = tokens - 1 WHERE tokens > 0;\n".to_string()
                } else if is_cas {
                    "UPDATE state_table SET state = :next WHERE state = :cur;\n".to_string()
                } else if is_div_guard {
                    "SELECT CASE WHEN factor = 0 THEN 0 ELSE base_val / factor END FROM data;\n"
                        .to_string()
                } else if is_list {
                    "SELECT unnest(ARRAY[0, 1]) AS item;\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::Bash => {
                if is_branchless {
                    "val=$(( (a & mask) | (b & ~mask) ))\n".to_string()
                } else if is_result {
                    "parse_val() { [[ \"$1\" =~ ^[0-9]+$ ]] && echo \"$1\" || return 1; }\n"
                        .to_string()
                } else if is_async {
                    "process_item \"$item\" &\n".to_string()
                } else if is_stream {
                    "printf '%s\\n' \"${items[@]}\" | awk '$1 > 0 { print $1 * 2 }'\n".to_string()
                } else if is_guard {
                    "[ -z \"$target\" ] && return 1\n".to_string()
                } else if is_crc {
                    "crc=$(cksum \"$payload\" | awk '{print $1}')\n".to_string()
                } else if is_bucket {
                    "[ \"$TOKENS\" -gt 0 ] && TOKENS=$((TOKENS - 1)) || return 1\n".to_string()
                } else if is_cas {
                    "ln -s /tmp/lock /tmp/active_lock 2>/dev/null\n".to_string()
                } else if is_div_guard {
                    "[ \"$factor\" -eq 0 ] && echo 0 || echo $((base_val / factor))\n".to_string()
                } else if is_list {
                    "items=(0 1)\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
            TargetLanguage::HtmlCss => {
                if is_branchless {
                    "<span class=\"branchless-eval\" data-mask=\"active\">1</span>\n".to_string()
                } else if is_result {
                    "<output class=\"safe-eval\" data-status=\"ok\"></output>\n".to_string()
                } else if is_async {
                    "<div class=\"async-state\" data-loading=\"false\"></div>\n".to_string()
                } else if is_stream {
                    "<ul class=\"filtered-list\"></ul>\n".to_string()
                } else if is_guard {
                    "<div class=\"guard-active\"></div>\n".to_string()
                } else if is_crc {
                    "<span class=\"crc-badge\">CRC-VALID</span>\n".to_string()
                } else if is_bucket {
                    "<div class=\"bucket-fill\" style=\"width: 100%;\"></div>\n".to_string()
                } else if is_cas {
                    "<span class=\"cas-flag\" data-atomic=\"true\"></span>\n".to_string()
                } else if is_div_guard {
                    "<span class=\"div-guard-active\"></span>\n".to_string()
                } else if is_list {
                    "<ul class=\"item-list\"><li>Item 1</li></ul>\n".to_string()
                } else {
                    format!("{}\n", target_code.trim())
                }
            }
        }
    }
}

pub fn match_tree_sitter_node(language: TargetLanguage, node_type: &str) -> Option<Primitive> {
    match language {
        TargetLanguage::Rust => match node_type {
            "let_declaration" => Some(Primitive::VarDecl),
            "const_item" | "static_item" => Some(Primitive::ConstDecl),
            "function_item" => Some(Primitive::FuncDecl),
            "struct_item" => Some(Primitive::TypeStructDecl),
            "trait_item" | "impl_item" => Some(Primitive::InterfaceTrait),
            "enum_item" => Some(Primitive::EnumDecl),
            "type_item" => Some(Primitive::TypeAlias),
            "mod_item" => Some(Primitive::Namespace),
            "if_expression" => Some(Primitive::IfElse),
            "match_expression" => Some(Primitive::SwitchMatch),
            "return_expression" => Some(Primitive::ReturnStmt),
            "break_expression" | "continue_expression" => Some(Primitive::BreakContinue),
            "for_expression" => Some(Primitive::ForInEach),
            "while_expression" => Some(Primitive::WhileLoop),
            "loop_expression" => Some(Primitive::InfiniteLoop),
            "binary_expression" => Some(Primitive::ArithmeticOp),
            "call_expression" => Some(Primitive::CallExpr),
            "field_expression" => Some(Primitive::MemberAccess),
            "type_cast_expression" => Some(Primitive::TypeCasting),
            "array_expression" => Some(Primitive::ArrayLiteral),
            "tuple_expression" => Some(Primitive::TupleRecord),
            "range_expression" => Some(Primitive::SliceRange),
            "pointer_type" | "dereference_expression" => Some(Primitive::PointerDeref),
            "use_declaration" => Some(Primitive::ImportInclude),
            "visibility_modifier" => Some(Primitive::ExportPublic),
            "attribute_item" => Some(Primitive::Decorator),
            "macro_invocation" | "macro_definition" => Some(Primitive::MacroCodeGen),
            "await_expression" => Some(Primitive::AsyncAwait),
            _ => None,
        },
        TargetLanguage::Python => match node_type {
            "assignment" => Some(Primitive::VarDecl),
            "function_definition" => Some(Primitive::FuncDecl),
            "class_definition" => Some(Primitive::TypeStructDecl),
            "if_statement" => Some(Primitive::IfElse),
            "match_statement" => Some(Primitive::SwitchMatch),
            "conditional_expression" => Some(Primitive::TernaryExpr),
            "return_statement" => Some(Primitive::ReturnStmt),
            "break_statement" | "continue_statement" => Some(Primitive::BreakContinue),
            "for_statement" => Some(Primitive::ForInEach),
            "while_statement" => Some(Primitive::WhileLoop),
            "yield" => Some(Primitive::YieldGenerator),
            "call" => Some(Primitive::CallExpr),
            "attribute" => Some(Primitive::MemberAccess),
            "list" => Some(Primitive::ArrayLiteral),
            "tuple" => Some(Primitive::TupleRecord),
            "dictionary" => Some(Primitive::MapObject),
            "set" => Some(Primitive::SetLiteral),
            "list_comprehension" | "dictionary_comprehension" => Some(Primitive::Comprehension),
            "slice" => Some(Primitive::SliceRange),
            "try_statement" => Some(Primitive::TryCatch),
            "raise_statement" => Some(Primitive::ThrowRaise),
            "with_statement" => Some(Primitive::DeferWith),
            "assert_statement" => Some(Primitive::AssertStmt),
            "import_statement" | "import_from_statement" => Some(Primitive::ImportInclude),
            "decorated_definition" | "decorator" => Some(Primitive::Decorator),
            _ => None,
        },
        TargetLanguage::JavaScript | TargetLanguage::TypeScript => match node_type {
            "variable_declaration" | "lexical_declaration" => Some(Primitive::VarDecl),
            "function_declaration" | "arrow_function" | "method_definition" => {
                Some(Primitive::FuncDecl)
            }
            "class_declaration" => Some(Primitive::TypeStructDecl),
            "interface_declaration" => Some(Primitive::InterfaceTrait),
            "enum_declaration" => Some(Primitive::EnumDecl),
            "type_alias_declaration" => Some(Primitive::TypeAlias),
            "if_statement" => Some(Primitive::IfElse),
            "switch_statement" => Some(Primitive::SwitchMatch),
            "ternary_expression" => Some(Primitive::TernaryExpr),
            "return_statement" => Some(Primitive::ReturnStmt),
            "break_statement" | "continue_statement" => Some(Primitive::BreakContinue),
            "for_statement" => Some(Primitive::ForLoop),
            "for_in_statement" => Some(Primitive::ForInEach),
            "while_statement" => Some(Primitive::WhileLoop),
            "do_statement" => Some(Primitive::DoWhileLoop),
            "yield_expression" => Some(Primitive::YieldGenerator),
            "call_expression" => Some(Primitive::CallExpr),
            "member_expression" => Some(Primitive::MemberAccess),
            "optional_chain" => Some(Primitive::NullSafetyOp),
            "as_expression" => Some(Primitive::TypeCasting),
            "array" => Some(Primitive::ArrayLiteral),
            "object" => Some(Primitive::MapObject),
            "try_statement" => Some(Primitive::TryCatch),
            "throw_statement" => Some(Primitive::ThrowRaise),
            "await_expression" => Some(Primitive::AsyncAwait),
            "import_statement" => Some(Primitive::ImportInclude),
            "export_statement" => Some(Primitive::ExportPublic),
            "decorator" => Some(Primitive::Decorator),
            _ => None,
        },
        TargetLanguage::C | TargetLanguage::Cpp => match node_type {
            "declaration" => Some(Primitive::VarDecl),
            "function_definition" | "lambda_expression" => Some(Primitive::FuncDecl),
            "struct_specifier" | "class_specifier" => Some(Primitive::TypeStructDecl),
            "enum_specifier" => Some(Primitive::EnumDecl),
            "type_definition" => Some(Primitive::TypeAlias),
            "namespace_definition" => Some(Primitive::Namespace),
            "if_statement" => Some(Primitive::IfElse),
            "switch_statement" => Some(Primitive::SwitchMatch),
            "conditional_expression" => Some(Primitive::TernaryExpr),
            "return_statement" => Some(Primitive::ReturnStmt),
            "break_statement" | "continue_statement" => Some(Primitive::BreakContinue),
            "goto_statement" => Some(Primitive::GotoLabel),
            "for_statement" => Some(Primitive::ForLoop),
            "while_statement" => Some(Primitive::WhileLoop),
            "do_statement" => Some(Primitive::DoWhileLoop),
            "call_expression" => Some(Primitive::CallExpr),
            "field_expression" => Some(Primitive::MemberAccess),
            "cast_expression" => Some(Primitive::TypeCasting),
            "pointer_declarator" | "pointer_expression" => Some(Primitive::PointerDeref),
            "preproc_include" => Some(Primitive::ImportInclude),
            "preproc_def" | "template_declaration" => Some(Primitive::MacroCodeGen),
            "try_statement" => Some(Primitive::TryCatch),
            "throw_statement" => Some(Primitive::ThrowRaise),
            "static_assert_declaration" => Some(Primitive::AssertStmt),
            "atomic_type" => Some(Primitive::AtomicOp),
            _ => None,
        },
        TargetLanguage::Go => match node_type {
            "var_declaration" | "short_var_declaration" | "var_spec" => Some(Primitive::VarDecl),
            "const_declaration" | "const_spec" => Some(Primitive::ConstDecl),
            "function_declaration" | "method_declaration" => Some(Primitive::FuncDecl),
            "type_declaration" | "type_spec" => Some(Primitive::TypeStructDecl),
            "interface_type" => Some(Primitive::InterfaceTrait),
            "type_alias" => Some(Primitive::TypeAlias),
            "package_clause" => Some(Primitive::Namespace),
            "if_statement" => Some(Primitive::IfElse),
            "expression_switch_statement" | "type_switch_statement" => Some(Primitive::SwitchMatch),
            "return_statement" => Some(Primitive::ReturnStmt),
            "break_statement" | "continue_statement" => Some(Primitive::BreakContinue),
            "fallthrough_statement" => Some(Primitive::Fallthrough),
            "for_statement" => Some(Primitive::ForLoop),
            "call_expression" => Some(Primitive::CallExpr),
            "selector_expression" => Some(Primitive::MemberAccess),
            "slice_expression" => Some(Primitive::SliceRange),
            "defer_statement" => Some(Primitive::DeferWith),
            "go_statement" => Some(Primitive::ThreadTask),
            "channel_type" | "send_statement" | "receive_statement" | "select_statement" => {
                Some(Primitive::ChannelLock)
            }
            "pointer_type" => Some(Primitive::PointerDeref),
            "import_declaration" => Some(Primitive::ImportInclude),
            _ => None,
        },
        TargetLanguage::Java | TargetLanguage::CSharp => match node_type {
            "local_variable_declaration" | "field_declaration" => Some(Primitive::VarDecl),
            "method_declaration" | "lambda_expression" => Some(Primitive::FuncDecl),
            "class_declaration" => Some(Primitive::TypeStructDecl),
            "interface_declaration" => Some(Primitive::InterfaceTrait),
            "enum_declaration" => Some(Primitive::EnumDecl),
            "package_declaration" | "namespace_declaration" => Some(Primitive::Namespace),
            "if_statement" => Some(Primitive::IfElse),
            "switch_statement" | "switch_expression" => Some(Primitive::SwitchMatch),
            "ternary_expression" => Some(Primitive::TernaryExpr),
            "return_statement" => Some(Primitive::ReturnStmt),
            "break_statement" | "continue_statement" => Some(Primitive::BreakContinue),
            "for_statement" => Some(Primitive::ForLoop),
            "enhanced_for_statement" => Some(Primitive::ForInEach),
            "while_statement" => Some(Primitive::WhileLoop),
            "do_statement" => Some(Primitive::DoWhileLoop),
            "method_invocation" => Some(Primitive::CallExpr),
            "field_access" => Some(Primitive::MemberAccess),
            "cast_expression" => Some(Primitive::TypeCasting),
            "try_statement" | "try_with_resources_statement" => Some(Primitive::TryCatch),
            "throw_statement" => Some(Primitive::ThrowRaise),
            "assert_statement" => Some(Primitive::AssertStmt),
            "synchronized_statement" | "lock_statement" => Some(Primitive::ChannelLock),
            "await_expression" => Some(Primitive::AsyncAwait),
            "import_declaration" | "using_directive" => Some(Primitive::ImportInclude),
            "annotation" | "attribute" => Some(Primitive::Decorator),
            _ => None,
        },
        TargetLanguage::Swift | TargetLanguage::Kotlin => match node_type {
            "variable_declaration" | "property_declaration" => Some(Primitive::VarDecl),
            "function_declaration" | "closure_expression" => Some(Primitive::FuncDecl),
            "class_declaration" => Some(Primitive::TypeStructDecl),
            "protocol_declaration" | "interface_declaration" => Some(Primitive::InterfaceTrait),
            "guard_statement" => Some(Primitive::GuardStmt),
            "if_statement" | "if_expression" => Some(Primitive::IfElse),
            "switch_statement" | "when_expression" => Some(Primitive::SwitchMatch),
            "return_statement" => Some(Primitive::ReturnStmt),
            "for_statement" => Some(Primitive::ForInEach),
            "while_statement" => Some(Primitive::WhileLoop),
            "try_expression" | "try_statement" => Some(Primitive::TryCatch),
            "throw_statement" => Some(Primitive::ThrowRaise),
            "defer_statement" => Some(Primitive::DeferWith),
            "await_expression" => Some(Primitive::AsyncAwait),
            "import_declaration" => Some(Primitive::ImportInclude),
            _ => None,
        },
        TargetLanguage::Php => match node_type {
            "variable_name" => Some(Primitive::VarDecl),
            "function_definition" | "method_declaration" => Some(Primitive::FuncDecl),
            "class_declaration" => Some(Primitive::TypeStructDecl),
            "trait_declaration" | "interface_declaration" => Some(Primitive::InterfaceTrait),
            "if_statement" => Some(Primitive::IfElse),
            "switch_statement" | "match_expression" => Some(Primitive::SwitchMatch),
            "return_statement" => Some(Primitive::ReturnStmt),
            "for_statement" => Some(Primitive::ForLoop),
            "foreach_statement" => Some(Primitive::ForInEach),
            "try_statement" => Some(Primitive::TryCatch),
            "throw_statement" => Some(Primitive::ThrowRaise),
            _ => None,
        },
        TargetLanguage::Sql => match node_type {
            "select_statement" => Some(Primitive::CallExpr),
            "insert_statement" | "update_statement" | "delete_statement" => {
                Some(Primitive::AssignmentOp)
            }
            "create_table_statement" => Some(Primitive::TypeStructDecl),
            "case_expression" => Some(Primitive::SwitchMatch),
            _ => None,
        },
        TargetLanguage::Bash => match node_type {
            "variable_assignment" => Some(Primitive::VarDecl),
            "function_definition" => Some(Primitive::FuncDecl),
            "if_statement" => Some(Primitive::IfElse),
            "for_statement" => Some(Primitive::ForInEach),
            "while_statement" => Some(Primitive::WhileLoop),
            "case_statement" => Some(Primitive::SwitchMatch),
            "subshell" => Some(Primitive::ThreadTask),
            "pipeline" => Some(Primitive::ChannelLock),
            "command" => Some(Primitive::CallExpr),
            _ => None,
        },
        TargetLanguage::HtmlCss => match node_type {
            "element" => Some(Primitive::TypeStructDecl),
            "rule_set" => Some(Primitive::AssignmentOp),
            _ => None,
        },
    }
}
