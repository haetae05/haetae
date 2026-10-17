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
