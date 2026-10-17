use crate::tcc_engine::VerifyError;
use haetae_core::config::HEX_RADIX;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LowerToken {
    Ident(String),
    Number(i64),
    Str(String),
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    DoubleSlash,
    Percent,
    // Bitwise & Logic
    Amp,
    Pipe,
    Caret,
    Tilde,
    Bang,
    AmpAmp,
    PipePipe,
    Shl,
    Shr,
    // Comparison
    Eq,
    EqEq,
    TripleEq,
    NotEq,
    NotEqEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    // Assignment & Compound Assignment
    ColonEq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpEq,
    PipeEq,
    CaretEq,
    ShlEq,
    ShrEq,
    PlusPlus,
    MinusMinus,
    // Arrows & Delimiters
    Arrow,
    FatArrow,
    Question,
    Colon,
    Dot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semi,
    // Directives
    Preprocessor(String),
    // Universal Operational Keywords
    KeywordDef,
    KeywordFn,
    KeywordPub,
    KeywordFunc,
    KeywordFunction,
    KeywordReturn,
    KeywordIf,
    KeywordElif,
    KeywordElse,
    KeywordWhile,
    KeywordFor,
    KeywordBreak,
    KeywordContinue,
    KeywordAnd,
    KeywordOr,
    KeywordNot,
    KeywordLet,
    KeywordConst,
    KeywordVar,
    KeywordMut,
    KeywordStruct,
    KeywordClass,
    KeywordMatch,
    KeywordSwitch,
    KeywordCase,
    KeywordDefault,
    KeywordType,
    KeywordTrue,
    KeywordFalse,
    KeywordNone,
    KeywordTry,
    KeywordCatch,
    KeywordThrow,
    KeywordAsync,
    KeywordAwait,
    KeywordYield,
    KeywordDo,
    KeywordLoop,
}

pub struct LowerLexer<'a> {
    chars: Vec<char>,
    pos: usize,
    _lang_hint: &'a str,
}

impl<'a> LowerLexer<'a> {
    pub fn new(source: &'a str, lang_hint: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            _lang_hint: lang_hint,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<LowerToken>, VerifyError> {
        let mut tokens = Vec::new();

        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];

            // 1. Skip whitespace
            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }

            // 2. Preprocessor & Comments (#)
            if c == '#' {
                let mut line_buf = String::new();
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                    line_buf.push(self.chars[self.pos]);
                    self.pos += 1;
                }
                let trimmed = line_buf.trim();
                if trimmed.starts_with("#include")
                    || trimmed.starts_with("#define")
                    || trimmed.starts_with("#ifdef")
                    || trimmed.starts_with("#ifndef")
                    || trimmed.starts_with("#endif")
                    || trimmed.starts_with("#pragma")
                    || trimmed.starts_with("#error")
                    || trimmed.starts_with("#undef")
                {
                    tokens.push(LowerToken::Preprocessor(line_buf));
                }
                continue;
            }

            // 3. Comments (//) vs DoubleSlash Operator (//)
            if c == '/' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                let is_prev_operand = matches!(
                    tokens.last(),
                    Some(LowerToken::Ident(_))
                        | Some(LowerToken::Number(_))
                        | Some(LowerToken::RParen)
                        | Some(LowerToken::RBracket)
                );

                let mut peek_idx = self.pos + 1;
                peek_idx += 1;
                while peek_idx < self.chars.len()
                    && (self.chars[peek_idx] == ' ' || self.chars[peek_idx] == '\t')
                {
                    peek_idx += 1;
                }

                let is_next_operand = peek_idx < self.chars.len()
                    && self.chars[peek_idx] != '\n'
                    && self.chars[peek_idx] != '\r'
                    && (self.chars[peek_idx].is_alphanumeric()
                        || self.chars[peek_idx] == '_'
                        || self.chars[peek_idx] == '('
                        || self.chars[peek_idx] == '$');

                if is_prev_operand && is_next_operand {
                    self.pos += 1;
                    self.pos += 1;
                    tokens.push(LowerToken::DoubleSlash);
                    continue;
                } else {
                    while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                        self.pos += 1;
                    }
                    continue;
                }
            }

            // 4. Multi-line block comments (/* ... */)
            if c == '/' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '*' {
                self.pos += 1;
                self.pos += 1;
                while self.pos + 1 < self.chars.len() {
                    if self.chars[self.pos] == '*' && self.chars[self.pos + 1] == '/' {
                        self.pos += 1;
                        self.pos += 1;
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }

            // 5. Numbers
            if c.is_ascii_digit() {
                let mut num_str = String::new();
                if c == '0'
                    && self.pos + 1 < self.chars.len()
                    && (self.chars[self.pos + 1] == 'x' || self.chars[self.pos + 1] == 'X')
                {
                    self.pos += 1;
                    self.pos += 1;
                    while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_hexdigit() {
                        num_str.push(self.chars[self.pos]);
                        self.pos += 1;
                    }
                    while self.pos < self.chars.len()
                        && (self.chars[self.pos] == 'u'
                            || self.chars[self.pos] == 'U'
                            || self.chars[self.pos] == 'l'
                            || self.chars[self.pos] == 'L')
                    {
                        self.pos += 1;
                    }
                    let val = i64::from_str_radix(&num_str, HEX_RADIX).unwrap_or(0);
                    tokens.push(LowerToken::Number(val));
                    continue;
                }

                while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                    num_str.push(self.chars[self.pos]);
                    self.pos += 1;
                }
                while self.pos < self.chars.len()
                    && (self.chars[self.pos] == 'u'
                        || self.chars[self.pos] == 'U'
                        || self.chars[self.pos] == 'l'
                        || self.chars[self.pos] == 'L'
                        || self.chars[self.pos] == 'f'
                        || self.chars[self.pos] == 'F')
                {
                    self.pos += 1;
                }
                let val = num_str.parse::<i64>().unwrap_or(0);
                tokens.push(LowerToken::Number(val));
                continue;
            }

            // 6. Identifiers & Keywords
            if c.is_ascii_alphabetic() || c == '_' || c == '$' {
                let mut id = String::new();
                while self.pos < self.chars.len()
                    && (self.chars[self.pos].is_ascii_alphanumeric()
                        || self.chars[self.pos] == '_'
                        || self.chars[self.pos] == '$')
                {
                    id.push(self.chars[self.pos]);
                    self.pos += 1;
                }

                let tok = match id.as_str() {
                    "def" => LowerToken::KeywordDef,
                    "fn" => LowerToken::KeywordFn,
                    "pub" => LowerToken::KeywordPub,
                    "func" | "fun" => LowerToken::KeywordFunc,
                    "function" => LowerToken::KeywordFunction,
                    "return" => LowerToken::KeywordReturn,
                    "if" => LowerToken::KeywordIf,
                    "elif" | "elsif" => LowerToken::KeywordElif,
                    "else" => LowerToken::KeywordElse,
                    "while" => LowerToken::KeywordWhile,
                    "for" => LowerToken::KeywordFor,
                    "do" => LowerToken::KeywordDo,
                    "loop" => LowerToken::KeywordLoop,
                    "break" => LowerToken::KeywordBreak,
                    "continue" => LowerToken::KeywordContinue,
                    "and" => LowerToken::KeywordAnd,
                    "or" => LowerToken::KeywordOr,
                    "not" => LowerToken::KeywordNot,
                    "let" => LowerToken::KeywordLet,
                    "const" => LowerToken::KeywordConst,
                    "var" => LowerToken::KeywordVar,
                    "mut" => LowerToken::KeywordMut,
                    "struct" => LowerToken::KeywordStruct,
                    "class" => LowerToken::KeywordClass,
                    "match" => LowerToken::KeywordMatch,
                    "switch" => LowerToken::KeywordSwitch,
                    "case" => LowerToken::KeywordCase,
                    "default" => LowerToken::KeywordDefault,
                    "type" => LowerToken::KeywordType,
                    "try" => LowerToken::KeywordTry,
                    "catch" | "except" => LowerToken::KeywordCatch,
                    "throw" | "raise" => LowerToken::KeywordThrow,
                    "async" => LowerToken::KeywordAsync,
                    "await" => LowerToken::KeywordAwait,
                    "yield" => LowerToken::KeywordYield,
                    "True" | "true" => LowerToken::KeywordTrue,
                    "False" | "false" => LowerToken::KeywordFalse,
                    "None" | "null" | "nil" | "undefined" => LowerToken::KeywordNone,
                    _ => LowerToken::Ident(id.trim_start_matches('$').to_string()),
                };
                tokens.push(tok);
                continue;
            }

            // 7. String Literals
            if c == '"' || c == '\'' {
                let quote = c;
                self.pos += 1;
                let mut s = String::new();
                while self.pos < self.chars.len() && self.chars[self.pos] != quote {
                    if self.chars[self.pos] == '\\' && self.pos + 1 < self.chars.len() {
                        self.pos += 1;
                    }
                    s.push(self.chars[self.pos]);
                    self.pos += 1;
                }
                if self.pos < self.chars.len() {
                    self.pos += 1;
                }
                tokens.push(LowerToken::Str(s));
                continue;
            }

            // 8. Operators & Punctuation
            match c {
                '.' => tokens.push(LowerToken::Dot),
                '+' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '+' {
                        self.pos += 1;
                        tokens.push(LowerToken::PlusPlus);
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::PlusEq);
                    } else {
                        tokens.push(LowerToken::Plus);
                    }
                }
                '-' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '>' {
                        self.pos += 1;
                        tokens.push(LowerToken::Arrow);
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '-' {
                        self.pos += 1;
                        tokens.push(LowerToken::MinusMinus);
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::MinusEq);
                    } else {
                        tokens.push(LowerToken::Minus);
                    }
                }
                '*' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::StarEq);
                    } else {
                        tokens.push(LowerToken::Star);
                    }
                }
                '/' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::SlashEq);
                    } else {
                        tokens.push(LowerToken::Slash);
                    }
                }
                '%' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::PercentEq);
                    } else {
                        tokens.push(LowerToken::Percent);
                    }
                }
                '&' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '&' {
                        self.pos += 1;
                        tokens.push(LowerToken::AmpAmp);
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::AmpEq);
                    } else {
                        tokens.push(LowerToken::Amp);
                    }
                }
                '|' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '|' {
                        self.pos += 1;
                        tokens.push(LowerToken::PipePipe);
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::PipeEq);
                    } else {
                        tokens.push(LowerToken::Pipe);
                    }
                }
                '^' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::CaretEq);
                    } else {
                        tokens.push(LowerToken::Caret);
                    }
                }
                '~' => tokens.push(LowerToken::Tilde),
                '?' => tokens.push(LowerToken::Question),
                ':' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::ColonEq);
                    } else {
                        tokens.push(LowerToken::Colon);
                    }
                }
                '=' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                            self.pos += 1;
                            tokens.push(LowerToken::TripleEq);
                        } else {
                            tokens.push(LowerToken::EqEq);
                        }
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '>' {
                        self.pos += 1;
                        tokens.push(LowerToken::FatArrow);
                    } else {
                        tokens.push(LowerToken::Eq);
                    }
                }
                '!' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                            self.pos += 1;
                            tokens.push(LowerToken::NotEqEq);
                        } else {
                            tokens.push(LowerToken::NotEq);
                        }
                    } else {
                        tokens.push(LowerToken::Bang);
                    }
                }
                '<' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '<' {
                        self.pos += 1;
                        if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                            self.pos += 1;
                            tokens.push(LowerToken::ShlEq);
                        } else {
                            tokens.push(LowerToken::Shl);
                        }
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::LtEq);
                    } else {
                        tokens.push(LowerToken::Lt);
                    }
                }
                '>' => {
                    if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '>' {
                        self.pos += 1;
                        if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                            self.pos += 1;
                            tokens.push(LowerToken::ShrEq);
                        } else {
                            tokens.push(LowerToken::Shr);
                        }
                    } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                        self.pos += 1;
                        tokens.push(LowerToken::GtEq);
                    } else {
                        tokens.push(LowerToken::Gt);
                    }
                }
                '(' => tokens.push(LowerToken::LParen),
                ')' => tokens.push(LowerToken::RParen),
                '[' => tokens.push(LowerToken::LBracket),
                ']' => tokens.push(LowerToken::RBracket),
                '{' => tokens.push(LowerToken::LBrace),
                '}' => tokens.push(LowerToken::RBrace),
                ',' => tokens.push(LowerToken::Comma),
                ';' => tokens.push(LowerToken::Semi),
                _ => {
                    return Err(VerifyError::UnsupportedSyntax(format!(
                        "Unexpected character in source: '{}'",
                        c
                    )));
                }
            }
            self.pos += 1;
        }

        Ok(tokens)
    }
}

pub fn is_known_type(s: &str) -> bool {
    matches!(
        s,
        "int"
            | "i32"
            | "i64"
            | "long"
            | "double"
            | "float"
            | "char"
            | "bool"
            | "boolean"
            | "size_t"
            | "int32_t"
            | "int64_t"
            | "uint32_t"
            | "uint64_t"
            | "short"
            | "byte"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "i8"
            | "i16"
            | "isize"
            | "usize"
            | "number"
            | "Int"
            | "string"
            | "String"
            | "void"
            | "auto"
    )
}

pub fn canonicalize_type(ty: &str) -> String {
    let t = ty.trim();
    match t {
        "void" => "void".to_string(),
        "int" | "i32" | "number" | "Int" | "short" | "byte" | "int32_t" | "i16" | "i8" => {
            "int".to_string()
        }
        "unsigned int" | "u32" | "uint32_t" | "u16" | "u8" | "uint" => "unsigned int".to_string(),
        "long" | "i64" | "long long" | "int64_t" | "isize" | "usize" | "size_t" => {
            "long long".to_string()
        }
        "unsigned long" | "u64" | "uint64_t" | "unsigned long long" | "ulong" => {
            "unsigned long long".to_string()
        }
        "float" | "f32" => "float".to_string(),
        "double" | "f64" => "double".to_string(),
        "char*" | "&str" | "string" | "String" => "const char*".to_string(),
        "void*" | "*mut" | "*const" | "ptr" => "void*".to_string(),
        "int*" => "int*".to_string(),
        "bool" | "boolean" => "int".to_string(),
        _ => "int".to_string(),
    }
}

pub fn normalize_parameter(tokens: &[LowerToken]) -> Option<(String, String)> {
    let filtered: Vec<&LowerToken> = tokens
        .iter()
        .filter(|t| match t {
            LowerToken::KeywordMut => false,
            LowerToken::Amp | LowerToken::Star => false,
            LowerToken::Ident(s) => {
                s != "_" && s != "final" && s != "const" && s != "var" && s != "val"
            }
            _ => true,
        })
        .collect();

    if filtered.is_empty() {
        return None;
    }

    // Pattern 1: name: type (Rust, TS, Python, Kotlin, Swift)
    if let Some(colon_pos) = filtered.iter().position(|t| **t == LowerToken::Colon) {
        let name_toks = &filtered[..colon_pos];
        let type_toks = &filtered[colon_pos + 1..];

        let name = match name_toks.last() {
            Some(LowerToken::Ident(n)) => n.clone(),
            _ => return None,
        };

        let ty_str = if let Some(LowerToken::Ident(ty)) = type_toks.first() {
            canonicalize_type(ty)
        } else {
            "int".to_string()
        };

        return Some((name, ty_str));
    }

    // Pattern 2: Two identifiers (C/Java/C# "int a" or Go "a int")
    let idents: Vec<String> = filtered
        .iter()
        .filter_map(|t| match t {
            LowerToken::Ident(s) => Some(s.clone()),
            _ => None,
        })
        .collect();

    if idents.len() > 1 {
        let first = &idents[0];
        let second = &idents[1];
        if is_known_type(first) {
            // C / Java / C#: type name
            return Some((second.clone(), canonicalize_type(first)));
        } else if is_known_type(second) {
            // Go: name type
            return Some((first.clone(), canonicalize_type(second)));
        } else {
            return Some((second.clone(), canonicalize_type(first)));
        }
    }

    // Pattern 3: Single identifier (Python / JS / PHP: a)
    if idents.len() == 1 {
        let name = idents[0].clone();
        return Some((name, "int".to_string()));
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstUnaryOp {
    Neg,
    Not,
    BitNot,
    Deref,
}

#[derive(Debug, Clone)]
pub enum AstPattern {
    Wildcard,
    Literal(i64),
    Ident(String),
}

#[derive(Debug, Clone)]
pub enum AstExpr {
    Number(i64),
    Str(String),
    Bool(bool),
    NoneVal,
    Ident(String),
    Binary {
        op: AstBinOp,
        lhs: Box<AstExpr>,
        rhs: Box<AstExpr>,
    },
    Unary {
        op: AstUnaryOp,
        operand: Box<AstExpr>,
    },
    Index {
        array: Box<AstExpr>,
        index: Box<AstExpr>,
    },
    Call {
        callee: Box<AstExpr>,
        args: Vec<AstExpr>,
    },
    MemberAccess {
        object: Box<AstExpr>,
        member: String,
        is_arrow: bool,
    },
    Ternary {
        cond: Box<AstExpr>,
        true_expr: Box<AstExpr>,
        false_expr: Box<AstExpr>,
    },
    MatchExpr {
        target: Box<AstExpr>,
        arms: Vec<(AstPattern, AstExpr)>,
    },
}

#[derive(Debug, Clone)]
pub enum AstStmt {
    Return(Option<AstExpr>),
    If {
        condition: AstExpr,
        then_branch: Vec<AstStmt>,
        elif_branches: Vec<(AstExpr, Vec<AstStmt>)>,
        else_branch: Option<Vec<AstStmt>>,
    },
    While {
        condition: AstExpr,
        body: Vec<AstStmt>,
    },
    For {
        init: Option<String>,
        condition: Option<AstExpr>,
        step: Option<String>,
        body: Vec<AstStmt>,
    },
    VarDecl {
        name: String,
        ty: String,
        init: Option<AstExpr>,
        is_array: bool,
        array_size: Option<usize>,
        array_elements: Vec<AstExpr>,
    },
    Assign {
        target: String,
        op: String,
        value: AstExpr,
    },
    Match {
        target: AstExpr,
        arms: Vec<AstMatchArm>,
    },
    Expr(AstExpr),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct AstMatchArm {
    pub pattern: AstPattern,
    pub body: AstMatchBody,
}

#[derive(Debug, Clone)]
pub enum AstMatchBody {
    Expr(AstExpr),
    Block(Vec<AstStmt>),
}

#[derive(Debug, Clone)]
pub struct AstStruct {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct AstFunction {
    pub name: String,
    pub return_type: String,
    pub params: Vec<(String, String)>,
    pub body: Vec<AstStmt>,
}

#[derive(Debug, Clone)]
pub enum AstItem {
    Preprocessor(String),
    Struct(AstStruct),
    Function(AstFunction),
}

pub type ParsedFunction = (String, String, Vec<(String, String)>);

pub struct StructuredCLowerer<'a> {
    pub tokens: &'a [LowerToken],
    pub pos: usize,
    known_arrays: HashMap<String, String>,
    declared_vars: HashSet<String>,
}

impl<'a> StructuredCLowerer<'a> {
    pub fn new(tokens: &'a [LowerToken]) -> Self {
        Self {
            tokens,
            pos: 0,
            known_arrays: HashMap::new(),
            declared_vars: HashSet::new(),
        }
    }

    fn peek(&self) -> Option<&'a LowerToken> {
        self.tokens.get(self.pos)
    }

    fn next_tok(&mut self) -> Option<&'a LowerToken> {
        if self.pos < self.tokens.len() {
            let t = &self.tokens[self.pos];
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn match_tok(&mut self, expected: &LowerToken) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn parse_all(&mut self) -> Result<Vec<AstItem>, VerifyError> {
        let mut items = Vec::new();

        while self.pos < self.tokens.len() {
            if self.match_tok(&LowerToken::Semi) {
                continue;
            }

            if let Some(LowerToken::Preprocessor(p)) = self.peek() {
                items.push(AstItem::Preprocessor(p.clone()));
                self.pos += 1;
                continue;
            }

            if self.peek() == Some(&LowerToken::KeywordStruct)
                || self.peek() == Some(&LowerToken::KeywordClass)
            {
                let st = self.parse_struct()?;
                items.push(AstItem::Struct(st));
                continue;
            }

            if let [LowerToken::KeywordType, LowerToken::KeywordStruct, ..] = &self.tokens[self.pos..] {
                self.pos += 1; // skip type
                let st = self.parse_struct()?;
                items.push(AstItem::Struct(st));
                continue;
            }

            let (fn_name, _, params) = self.parse_function()?;
            items.push(AstItem::Function(AstFunction {
                name: fn_name,
                return_type: "int".to_string(),
                params,
                body: Vec::new(),
            }));
        }

        Ok(items)
    }

    pub fn parse_struct(&mut self) -> Result<AstStruct, VerifyError> {
        if self.peek() == Some(&LowerToken::KeywordStruct)
            || self.peek() == Some(&LowerToken::KeywordClass)
        {
            self.pos += 1;
        }

        let name = match self.next_tok() {
            Some(LowerToken::Ident(s)) => s.clone(),
            other => {
                return Err(VerifyError::UnsupportedSyntax(format!(
                    "Expected struct/class name, got {:?}",
                    other
                )));
            }
        };

        if self.peek() == Some(&LowerToken::KeywordStruct)
            || self.peek() == Some(&LowerToken::KeywordClass)
        {
            self.pos += 1;
        }

        if !self.match_tok(&LowerToken::LBrace) {
            return Err(VerifyError::UnsupportedSyntax(
                "Expected '{' after struct name".to_string(),
            ));
        }

        let mut fields = Vec::new();
        while self.pos < self.tokens.len() {
            if self.match_tok(&LowerToken::RBrace) {
                break;
            }
            if self.match_tok(&LowerToken::Semi) || self.match_tok(&LowerToken::Comma) {
                continue;
            }
            if self.match_tok(&LowerToken::KeywordPub) {
                // skip pub
            }

            match self.next_tok() {
                Some(LowerToken::Ident(first)) => {
                    let mut field_name = first.clone();
                    let mut field_type = "int".to_string();

                    if self.match_tok(&LowerToken::Colon) {
                        // Rust / TS: name: type
                        if let Some(LowerToken::Ident(ty)) = self.next_tok() {
                            field_type = canonicalize_type(ty);
                        }
                    } else if let Some(LowerToken::Ident(second)) = self.peek() {
                        if is_known_type(first) {
                            field_type = canonicalize_type(first);
                            field_name = second.clone();
                        } else {
                            field_name = first.clone();
                            field_type = canonicalize_type(second);
                        }
                        self.pos += 1;
                    }

                    fields.push((field_name, field_type));
                }
                Some(LowerToken::RBrace) => {
                    break;
                }
                other => {
                    return Err(VerifyError::UnsupportedSyntax(format!(
                        "Unexpected token in struct body: {:?}",
                        other
                    )));
                }
            }
        }

        self.match_tok(&LowerToken::Semi);

        Ok(AstStruct { name, fields })
    }

    pub fn parse_function(&mut self) -> Result<ParsedFunction, VerifyError> {
        let mut is_async = false;
        // Skip qualifiers: pub / async / export / public / private / protected / static / inline / virtual / override / final / constexpr / extern
        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::KeywordPub => {
                    self.pos += 1;
                    if let Some(LowerToken::LParen) = self.peek() {
                        while let Some(t) = self.next_tok() {
                            if *t == LowerToken::RParen {
                                break;
                            }
                        }
                    }
                }
                LowerToken::KeywordAsync => {
                    self.pos += 1;
                    is_async = true;
                }
                LowerToken::Ident(s)
                    if s == "export"
                        || s == "async"
                        || s == "public"
                        || s == "private"
                        || s == "protected"
                        || s == "static"
                        || s == "inline"
                        || s == "virtual"
                        || s == "override"
                        || s == "final"
                        || s == "constexpr"
                        || s == "extern" =>
                {
                    if s == "async" {
                        is_async = true;
                    }
                    self.pos += 1;
                }
                _ => break,
            }
        }

        let mut declared_ret_type = "int".to_string();

        // Header keyword: def / fn / func / function / or C/C++/Java/C# return type
        let has_fn_kw = match self.peek() {
            Some(LowerToken::KeywordDef)
            | Some(LowerToken::KeywordFn)
            | Some(LowerToken::KeywordFunc)
            | Some(LowerToken::KeywordFunction) => {
                self.pos += 1;
                true
            }
            Some(LowerToken::Ident(ret_type)) if is_known_type(ret_type) => {
                declared_ret_type = canonicalize_type(ret_type);
                self.pos += 1;
                true
            }
            _ => false,
        };

        if !has_fn_kw {
            return Err(VerifyError::UnsupportedSyntax(
                "Missing function definition keyword or return type".to_string(),
            ));
        }

        // Pointer return type: int* func()
        if self.match_tok(&LowerToken::Star) {
            declared_ret_type.push('*');
        }

        // Go Receiver skip: func (r *Receiver) Name(...)
        if self.peek() == Some(&LowerToken::LParen) {
            self.pos += 1;
            while let Some(t) = self.next_tok() {
                if *t == LowerToken::RParen {
                    break;
                }
            }
        }

        // Function Name
        let fn_name = match self.next_tok() {
            Some(LowerToken::Ident(name)) => name.clone(),
            other => {
                return Err(VerifyError::UnsupportedSyntax(format!(
                    "Expected function name identifier, got {:?}",
                    other
                )));
            }
        };

        // Parameters in ( ... )
        if !self.match_tok(&LowerToken::LParen) {
            return Err(VerifyError::UnsupportedSyntax(
                "Expected '(' after function name".to_string(),
            ));
        }

        let mut params = Vec::new();
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::RParen {
                self.pos += 1;
                break;
            }
            if *tok == LowerToken::Comma {
                self.pos += 1;
                continue;
            }

            let mut param_tokens = Vec::new();
            while let Some(t) = self.peek() {
                if *t == LowerToken::Comma || *t == LowerToken::RParen {
                    break;
                }
                param_tokens.push(t.clone());
                self.pos += 1;
            }

            if let Some((pname, ptype)) = normalize_parameter(&param_tokens) {
                if pname != "self" && pname != "cls" && pname != "this" && pname != "void" {
                    params.push((pname, ptype));
                }
            }
        }

        // Return type annotation: -> type, : type, or Go trailing type
        if self.match_tok(&LowerToken::Arrow) {
            if let Some(LowerToken::Ident(ty)) = self.peek() {
                declared_ret_type = canonicalize_type(ty);
                self.pos += 1;
            }
        } else if self.match_tok(&LowerToken::Colon) {
            if let Some(LowerToken::Ident(ty)) = self.peek() {
                if is_known_type(ty) {
                    declared_ret_type = canonicalize_type(ty);
                    self.pos += 1;
                }
            }
        } else if let Some(LowerToken::Ident(ty)) = self.peek() {
            if is_known_type(ty) {
                declared_ret_type = canonicalize_type(ty);
                self.pos += 1;
            }
        } else if self.peek() == Some(&LowerToken::LParen) {
            // Go multiple return or parenthesized: (int)
            self.pos += 1;
            if let Some(LowerToken::Ident(ty)) = self.peek() {
                declared_ret_type = canonicalize_type(ty);
                self.pos += 1;
            }
            self.match_tok(&LowerToken::RParen);
        }

        let is_brace_block = if self.match_tok(&LowerToken::LBrace) {
            true
        } else {
            self.match_tok(&LowerToken::Colon);
            false
        };

        // Reset scope for this function
        self.declared_vars.clear();
        for (pname, _) in &params {
            self.declared_vars.insert(pname.clone());
        }

        // Parse Body Statements
        let mut body_statements = Vec::new();
        while self.pos < self.tokens.len() {
            if is_brace_block && self.peek() == Some(&LowerToken::RBrace) {
                self.pos += 1;
                break;
            }

            if self.peek() == Some(&LowerToken::KeywordDef)
                || self.peek() == Some(&LowerToken::KeywordFn)
                || self.peek() == Some(&LowerToken::KeywordFunc)
                || self.peek() == Some(&LowerToken::KeywordFunction)
            {
                break;
            }

            let stmt = self.parse_statement()?;
            if !stmt.is_empty() {
                body_statements.push(stmt);
            }
        }

        if body_statements.is_empty() {
            return Err(VerifyError::UnsupportedSyntax(format!(
                "Function '{}' has an empty or unparseable body",
                fn_name
            )));
        }

        let c_ret = canonicalize_type(&declared_ret_type);
        let mut c_func = format!("{} {}(", c_ret, fn_name);
        let mut c_params = Vec::new();
        for (pname, ptype) in &params {
            c_params.push(format!("{} {}", ptype, pname));
        }
        c_func.push_str(&c_params.join(", "));
        c_func.push_str(") {\n");

        let uses_async = is_async || body_statements.iter().any(|s| s.contains("haetae_pt"));
        if uses_async {
            c_func.push_str("  struct pt haetae_pt;\n");
            c_func.push_str("  PT_INIT(&haetae_pt);\n");
            c_func.push_str("  PT_BEGIN(&haetae_pt);\n");
        }

        for stmt in body_statements {
            c_func.push_str("  ");
            c_func.push_str(&stmt);
            c_func.push('\n');
        }

        if uses_async {
            c_func.push_str("  PT_END(&haetae_pt);\n");
        }
        c_func.push_str("}\n");

        Ok((fn_name, c_func, params))
    }

    pub fn parse_block_or_statement(&mut self) -> Result<String, VerifyError> {
        if self.match_tok(&LowerToken::LBrace) {
            let mut stmts = Vec::new();
            while self.pos < self.tokens.len() {
                if self.match_tok(&LowerToken::RBrace) {
                    break;
                }
                let s = self.parse_statement()?;
                if !s.is_empty() {
                    stmts.push(s);
                }
            }
            Ok(stmts.join(" "))
        } else {
            self.parse_statement()
        }
    }

    pub fn parse_statement(&mut self) -> Result<String, VerifyError> {
        let tok = match self.peek() {
            Some(t) => t.clone(),
            None => return Ok(String::new()),
        };

        match tok {
            LowerToken::Semi => {
                self.pos += 1;
                Ok(String::new())
            }
            LowerToken::KeywordReturn => {
                self.pos += 1;
                if self.match_tok(&LowerToken::Semi) {
                    return Ok("return 0;".to_string());
                }
                let expr = self.parse_expr()?;
                self.match_tok(&LowerToken::Semi);
                Ok(format!("return {};", expr))
            }
            LowerToken::KeywordIf => {
                self.pos += 1;
                let has_paren = self.match_tok(&LowerToken::LParen);
                let cond = self.parse_expr()?;
                if has_paren {
                    self.match_tok(&LowerToken::RParen);
                }
                self.match_tok(&LowerToken::Colon);
                let sub_stmt = self.parse_block_or_statement()?;

                let mut out = format!("if ({}) {{ {} }}", cond, sub_stmt);

                while self.peek() == Some(&LowerToken::KeywordElif) {
                    self.pos += 1;
                    let elif_paren = self.match_tok(&LowerToken::LParen);
                    let elif_cond = self.parse_expr()?;
                    if elif_paren {
                        self.match_tok(&LowerToken::RParen);
                    }
                    self.match_tok(&LowerToken::Colon);
                    let elif_stmt = self.parse_block_or_statement()?;
                    out.push_str(&format!(" else if ({}) {{ {} }}", elif_cond, elif_stmt));
                }

                if self.match_tok(&LowerToken::KeywordElse) {
                    self.match_tok(&LowerToken::Colon);
                    if self.peek() == Some(&LowerToken::KeywordIf) {
                        let else_if_stmt = self.parse_statement()?;
                        out.push_str(&format!(" else {}", else_if_stmt));
                    } else {
                        let else_stmt = self.parse_block_or_statement()?;
                        out.push_str(&format!(" else {{ {} }}", else_stmt));
                    }
                }

                Ok(out)
            }
            LowerToken::KeywordWhile => {
                self.pos += 1;
                let has_paren = self.match_tok(&LowerToken::LParen);
                let cond = self.parse_expr()?;
                if has_paren {
                    self.match_tok(&LowerToken::RParen);
                }
                self.match_tok(&LowerToken::Colon);
                let body = self.parse_block_or_statement()?;
                Ok(format!("while ({}) {{ {} }}", cond, body))
            }
            LowerToken::KeywordFor => {
                self.pos += 1;
                if self.match_tok(&LowerToken::LParen) {
                    let init_stmt = self.parse_statement()?;
                    let cond_expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    let post_expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::RParen);
                    let body = self.parse_block_or_statement()?;
                    Ok(format!(
                        "for ({}; {}; {}) {{ {} }}",
                        init_stmt.trim_end_matches(';'),
                        cond_expr,
                        post_expr,
                        body
                    ))
                } else {
                    let mut loop_var = "i".to_string();
                    if let Some(LowerToken::Ident(v)) = self.peek() {
                        loop_var = v.clone();
                        self.pos += 1;
                    }
                    if let Some(LowerToken::Ident(in_kw)) = self.peek() {
                        if in_kw == "in" {
                            self.pos += 1;
                        }
                    }
                    let iter_expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Colon);
                    let body = self.parse_block_or_statement()?;
                    let limit = if iter_expr.starts_with("range(") && iter_expr.ends_with(')') {
                        iter_expr
                            .trim_start_matches("range(")
                            .trim_end_matches(')')
                            .to_string()
                    } else {
                        iter_expr
                    };
                    Ok(format!(
                        "for (int {} = 0; {} < {}; {}++) {{ {} }}",
                        loop_var, loop_var, limit, loop_var, body
                    ))
                }
            }
            LowerToken::KeywordLoop => {
                self.pos += 1;
                let body = self.parse_block_or_statement()?;
                Ok(format!("while (1) {{ {} }}", body))
            }
            LowerToken::KeywordDo => {
                self.pos += 1;
                let body = self.parse_block_or_statement()?;
                self.match_tok(&LowerToken::KeywordWhile);
                self.match_tok(&LowerToken::LParen);
                let cond = self.parse_expr()?;
                self.match_tok(&LowerToken::RParen);
                self.match_tok(&LowerToken::Semi);
                Ok(format!("do {{ {} }} while ({});", body, cond))
            }
            LowerToken::KeywordMatch | LowerToken::KeywordSwitch => {
                self.pos += 1;
                let has_paren = self.match_tok(&LowerToken::LParen);
                let target = self.parse_expr()?;
                if has_paren {
                    self.match_tok(&LowerToken::RParen);
                }
                if !self.match_tok(&LowerToken::LBrace) {
                    return Err(VerifyError::UnsupportedSyntax(
                        "Expected '{' after match/switch target".to_string(),
                    ));
                }

                let mut cases = Vec::new();
                while self.pos < self.tokens.len() {
                    if self.match_tok(&LowerToken::RBrace) {
                        break;
                    }
                    self.match_tok(&LowerToken::KeywordCase);

                    let pat = match self.next_tok() {
                        Some(LowerToken::Number(n)) => format!("case {}:", n),
                        Some(LowerToken::Ident(s)) if s == "_" || s == "default" => {
                            "default:".to_string()
                        }
                        Some(LowerToken::KeywordDefault) => "default:".to_string(),
                        Some(LowerToken::Ident(s)) => format!("case {}:", s),
                        other => {
                            return Err(VerifyError::UnsupportedSyntax(format!(
                                "Expected match pattern, got {:?}",
                                other
                            )));
                        }
                    };

                    if self.match_tok(&LowerToken::FatArrow) {
                        let arm_body = if self.match_tok(&LowerToken::LBrace) {
                            let mut stmts = Vec::new();
                            while self.pos < self.tokens.len() {
                                if self.match_tok(&LowerToken::RBrace) {
                                    break;
                                }
                                let s = self.parse_statement()?;
                                if !s.is_empty() {
                                    stmts.push(s);
                                }
                            }
                            stmts.join(" ")
                        } else {
                            let expr = self.parse_expr()?;
                            self.match_tok(&LowerToken::Comma);
                            format!("return {};", expr)
                        };
                        self.match_tok(&LowerToken::Comma);
                        cases.push(format!("{} {{ {} }}", pat, arm_body));
                    } else if self.match_tok(&LowerToken::Colon) {
                        let mut stmts = Vec::new();
                        while self.pos < self.tokens.len() {
                            if let Some(tok) = self.peek() {
                                if *tok == LowerToken::KeywordCase
                                    || *tok == LowerToken::KeywordDefault
                                    || *tok == LowerToken::RBrace
                                {
                                    break;
                                }
                            }
                            let s = self.parse_statement()?;
                            if !s.is_empty() {
                                stmts.push(s);
                            }
                        }
                        cases.push(format!("{} {{ {} }}", pat, stmts.join(" ")));
                    }
                }

                Ok(format!("switch ({}) {{ {} }}", target, cases.join(" ")))
            }
            LowerToken::KeywordBreak => {
                self.pos += 1;
                self.match_tok(&LowerToken::Semi);
                Ok("break;".to_string())
            }
            LowerToken::KeywordContinue => {
                self.pos += 1;
                self.match_tok(&LowerToken::Semi);
                Ok("continue;".to_string())
            }
            // Verification Tool 1: CEXCEPTION_H (Exception Handling)
            LowerToken::KeywordTry => {
                self.pos += 1;
                self.match_tok(&LowerToken::Colon);
                let try_body = self.parse_block_or_statement()?;
                let mut catch_part = String::new();
                if self.match_tok(&LowerToken::KeywordCatch) {
                    let mut err_var = "haetae_err".to_string();
                    if self.match_tok(&LowerToken::LParen) {
                        while let Some(t) = self.next_tok() {
                            if *t == LowerToken::RParen {
                                break;
                            }
                            if let LowerToken::Ident(name) = t {
                                err_var = name.clone();
                            }
                        }
                    }
                    self.match_tok(&LowerToken::Colon);
                    let catch_body = self.parse_block_or_statement()?;
                    catch_part = format!(
                        " else if (({} = CException_CurrentFrame->id)) {{ {} }}",
                        err_var, catch_body
                    );
                }
                Ok(format!(
                    "int {} = 0; if (!setjmp(CException_CurrentFrame->buf)) {{ {} }}{}",
                    "haetae_err", try_body, catch_part
                ))
            }
            LowerToken::KeywordThrow => {
                self.pos += 1;
                let expr = if self.peek() == Some(&LowerToken::Semi) {
                    "1".to_string()
                } else {
                    self.parse_expr()?
                };
                self.match_tok(&LowerToken::Semi);
                Ok(format!("Throw({});", expr))
            }
            // Verification Tool 2: PT_H (Protothreads Async/Await & Coroutines)
            LowerToken::KeywordAwait => {
                self.pos += 1;
                let expr = self.parse_expr()?;
                self.match_tok(&LowerToken::Semi);
                Ok(format!("PT_WAIT_UNTIL(&haetae_pt, ({}));", expr))
            }
            LowerToken::KeywordYield => {
                self.pos += 1;
                if self.peek() == Some(&LowerToken::Semi) {
                    self.pos += 1;
                    Ok("PT_YIELD(&haetae_pt);".to_string())
                } else {
                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    Ok(format!("PT_YIELD(&haetae_pt); (void)({});", expr))
                }
            }
            LowerToken::KeywordLet | LowerToken::KeywordConst | LowerToken::KeywordVar => {
                self.pos += 1;
                self.match_tok(&LowerToken::KeywordMut);

                let var_name = match self.next_tok() {
                    Some(LowerToken::Ident(name)) => name.clone(),
                    other => {
                        return Err(VerifyError::UnsupportedSyntax(format!(
                            "Expected variable name, got {:?}",
                            other
                        )));
                    }
                };

                let mut var_type = "int".to_string();
                if self.match_tok(&LowerToken::Colon) {
                    if let Some(LowerToken::Ident(ty)) = self.peek() {
                        var_type = canonicalize_type(ty);
                        self.pos += 1;
                    }
                } else if let Some(LowerToken::Ident(ty)) = self.peek() {
                    if is_known_type(ty) {
                        var_type = canonicalize_type(ty);
                        self.pos += 1;
                    }
                }

                self.match_tok(&LowerToken::Eq);

                if self.peek() == Some(&LowerToken::LBracket) {
                    return self.parse_array_decl(&var_name);
                }

                if let [LowerToken::Ident(id), LowerToken::Bang, LowerToken::LBracket, ..] = &self.tokens[self.pos..] {
                    if id == "vec" {
                        self.pos += 1;
                        self.pos += 1;
                        self.pos += 1;
                        return self.parse_array_decl_after_lbracket(&var_name);
                    }
                }

                let expr = self.parse_expr()?;
                self.match_tok(&LowerToken::Semi);

                self.declared_vars.insert(var_name.clone());
                Ok(format!("{} {} = {};", var_type, var_name, expr))
            }
            LowerToken::Ident(ref name) => {
                // vec![1, 2, 3]
                if let [LowerToken::Ident(id), LowerToken::Bang, LowerToken::LBracket, ..] = &self.tokens[self.pos..] {
                    if id == "vec" {
                        self.pos += 1;
                        self.pos += 1;
                        self.pos += 1;
                        return self.parse_array_items("arr");
                    }
                }

                // C pointer declaration: int* p = 0;
                if is_known_type(name) {
                    if let [LowerToken::Ident(_), LowerToken::Star, LowerToken::Ident(var), ..] = &self.tokens[self.pos..] {
                        let ty = format!("{}*", name);
                        let var_name = var.clone();
                        self.pos += 1;
                        self.pos += 1;
                        self.pos += 1;
                        self.declared_vars.insert(var_name.clone());
                        if self.match_tok(&LowerToken::Eq) {
                            let init_expr = self.parse_expr()?;
                            self.match_tok(&LowerToken::Semi);
                            return Ok(format!("{} {} = {};", ty, var_name, init_expr));
                        } else {
                            self.match_tok(&LowerToken::Semi);
                            return Ok(format!("{} {};", ty, var_name));
                        }
                    }
                }

                // C fixed array: int a[5] = {1, 2, 3, 4, 5};
                if is_known_type(name) {
                    if let [LowerToken::Ident(_), LowerToken::Ident(var), LowerToken::LBracket, ..] = &self.tokens[self.pos..] {
                        let var_name = var.clone();
                        self.pos += 1; // skip type
                        self.pos += 1; // skip var
                        self.pos += 1; // skip [
                        let mut arr_sz_str = String::new();
                        if let Some(LowerToken::Number(sz)) = self.peek() {
                            arr_sz_str = format!("{}", sz);
                            self.pos += 1;
                        }
                        self.match_tok(&LowerToken::RBracket);
                        self.match_tok(&LowerToken::Eq);

                        if !arr_sz_str.is_empty() {
                            self.known_arrays
                                .insert(var_name.clone(), arr_sz_str.clone());
                        }

                        if self.match_tok(&LowerToken::LBrace) {
                            let mut elems = Vec::new();
                            while self.pos < self.tokens.len() {
                                if self.match_tok(&LowerToken::RBrace) {
                                    break;
                                }
                                if self.match_tok(&LowerToken::Comma) {
                                    continue;
                                }
                                let item = self.parse_expr()?;
                                elems.push(item);
                            }
                            self.match_tok(&LowerToken::Semi);
                            if arr_sz_str.is_empty() {
                                arr_sz_str = format!("{}", elems.len());
                                self.known_arrays
                                    .insert(var_name.clone(), arr_sz_str.clone());
                            }
                            self.declared_vars.insert(var_name.clone());
                            return Ok(format!(
                                "int {}[{}] = {{{}}};",
                                var_name,
                                arr_sz_str,
                                elems.join(", ")
                            ));
                        }
                    }
                }

                // Typed variable declaration: int x = 5;
                if is_known_type(name)
                    && self.pos + 1 < self.tokens.len()
                    && matches!(self.tokens.get(self.pos + 1), Some(LowerToken::Ident(_)))
                {
                    let ty = canonicalize_type(name);
                    self.pos += 1;
                    if let Some(LowerToken::Ident(vname)) = self.next_tok() {
                        let var_name = vname.clone();
                        self.declared_vars.insert(var_name.clone());
                        if self.match_tok(&LowerToken::Eq) {
                            let init_expr = self.parse_expr()?;
                            self.match_tok(&LowerToken::Semi);
                            return Ok(format!("{} {} = {};", ty, var_name, init_expr));
                        } else {
                            self.match_tok(&LowerToken::Semi);
                            return Ok(format!("{} {};", ty, var_name));
                        }
                    }
                }

                let next_tok = self.tokens.get(self.pos + 1);

                // Simple assignment: var = expr OR var := expr
                if next_tok == Some(&LowerToken::Eq) || next_tok == Some(&LowerToken::ColonEq) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;

                    if self.peek() == Some(&LowerToken::LBracket) {
                        return self.parse_array_decl(&var_name);
                    }

                    if let [LowerToken::Ident(id), LowerToken::Bang, LowerToken::LBracket, ..] = &self.tokens[self.pos..] {
                        if id == "vec" {
                            self.pos += 1;
                            self.pos += 1;
                            self.pos += 1;
                            return self.parse_array_decl_after_lbracket(&var_name);
                        }
                    }

                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);

                    if self.declared_vars.contains(&var_name) {
                        return Ok(format!("{} = {};", var_name, expr));
                    } else {
                        self.declared_vars.insert(var_name.clone());
                        return Ok(format!("int {} = {};", var_name, expr));
                    }
                }

                // Compound assignments
                if next_tok == Some(&LowerToken::PlusEq) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!("{} = ({} + ({}));", var_name, var_name, expr));
                }
                if next_tok == Some(&LowerToken::MinusEq) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!("{} = ({} - ({}));", var_name, var_name, expr));
                }
                if next_tok == Some(&LowerToken::StarEq) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!(
                        "{} = haetae_checked_mul({}, ({}));",
                        var_name, var_name, expr
                    ));
                }
                if next_tok == Some(&LowerToken::SlashEq) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!(
                        "{} = haetae_checked_div({}, ({}));",
                        var_name, var_name, expr
                    ));
                }
                if next_tok == Some(&LowerToken::PercentEq) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    let expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!(
                        "{} = haetae_checked_mod({}, ({}));",
                        var_name, var_name, expr
                    ));
                }
                if next_tok == Some(&LowerToken::PlusPlus) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!("{} = ({} + 1);", var_name, var_name));
                }
                if next_tok == Some(&LowerToken::MinusMinus) {
                    let var_name = name.clone();
                    self.pos += 1;
                    self.pos += 1;
                    self.match_tok(&LowerToken::Semi);
                    return Ok(format!("{} = ({} - 1);", var_name, var_name));
                }

                // Expression or implicit return
                let expr = self.parse_expr()?;
                let is_semi = self.match_tok(&LowerToken::Semi);
                if is_semi {
                    Ok(format!("{};", expr))
                } else {
                    Ok(format!("return {};", expr))
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                let is_semi = self.match_tok(&LowerToken::Semi);
                if is_semi {
                    Ok(format!("{};", expr))
                } else {
                    Ok(format!("return {};", expr))
                }
            }
        }
    }

    fn parse_array_decl(&mut self, var_name: &str) -> Result<String, VerifyError> {
        if self.peek() == Some(&LowerToken::LBracket) {
            self.pos += 1;
        }
        self.parse_array_items_inner(var_name)
    }

    fn parse_array_decl_after_lbracket(&mut self, var_name: &str) -> Result<String, VerifyError> {
        self.parse_array_items_inner(var_name)
    }

    fn parse_array_items_inner(&mut self, var_name: &str) -> Result<String, VerifyError> {
        let mut items = Vec::new();
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::RBracket {
                self.pos += 1;
                break;
            }
            if *tok == LowerToken::Comma {
                self.pos += 1;
                continue;
            }
            let item_expr = self.parse_expr()?;
            items.push(item_expr);
        }

        self.match_tok(&LowerToken::Semi);

        let len_val = items.len();
        let len_var = format!("{}_len", var_name);
        self.known_arrays
            .insert(var_name.to_string(), len_var.clone());
        self.declared_vars.insert(var_name.to_string());
        self.declared_vars.insert(len_var.clone());

        if items.is_empty() {
            Ok(format!(
                "int {} = {}; int {}[1] = {{0}};",
                len_var, len_val, var_name
            ))
        } else {
            Ok(format!(
                "int {} = {}; int {}[] = {{{}}};",
                len_var,
                len_val,
                var_name,
                items.join(", ")
            ))
        }
    }

    fn parse_array_items(&mut self, var_name: &str) -> Result<String, VerifyError> {
        self.parse_array_items_inner(var_name)
    }

    pub fn parse_expr(&mut self) -> Result<String, VerifyError> {
        if self.peek() == Some(&LowerToken::KeywordMatch) {
            self.pos += 1;
            let target = self.parse_expr()?;
            if !self.match_tok(&LowerToken::LBrace) {
                return Err(VerifyError::UnsupportedSyntax(
                    "Expected '{' after match target".to_string(),
                ));
            }
            let mut arms = Vec::new();
            while self.pos < self.tokens.len() {
                if self.match_tok(&LowerToken::RBrace) {
                    break;
                }
                let pat_str = match self.next_tok() {
                    Some(LowerToken::Number(n)) => format!("{}", n),
                    Some(LowerToken::Ident(s)) => s.clone(),
                    other => {
                        return Err(VerifyError::UnsupportedSyntax(format!(
                            "Expected pattern in match expr, got {:?}",
                            other
                        )));
                    }
                };
                if !self.match_tok(&LowerToken::FatArrow) {
                    return Err(VerifyError::UnsupportedSyntax(
                        "Expected '=>' in match expr".to_string(),
                    ));
                }
                let body_expr = self.parse_expr()?;
                self.match_tok(&LowerToken::Comma);
                arms.push((pat_str, body_expr));
            }

            let mut chain = "0".to_string();
            for (pat, body) in arms.into_iter().rev() {
                if pat == "_" {
                    chain = body;
                } else {
                    chain = format!("(({} == {}) ? ({}) : ({}))", target, pat, body, chain);
                }
            }
            return Ok(chain);
        }

        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_logical_and()?;
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::PipePipe || *tok == LowerToken::KeywordOr {
                self.pos += 1;
                let rhs = self.parse_logical_and()?;
                lhs = format!("({} || {})", lhs, rhs);
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_logical_and(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_bitwise_or()?;
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::AmpAmp || *tok == LowerToken::KeywordAnd {
                self.pos += 1;
                let rhs = self.parse_bitwise_or()?;
                lhs = format!("({} && {})", lhs, rhs);
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_bitwise_or(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_bitwise_xor()?;
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::Pipe {
                self.pos += 1;
                let rhs = self.parse_bitwise_xor()?;
                lhs = format!("({} | {})", lhs, rhs);
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_bitwise_xor(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_bitwise_and()?;
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::Caret {
                self.pos += 1;
                let rhs = self.parse_bitwise_and()?;
                lhs = format!("({} ^ {})", lhs, rhs);
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_bitwise_and(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_equality()?;
        while let Some(tok) = self.peek() {
            if *tok == LowerToken::Amp {
                self.pos += 1;
                let rhs = self.parse_equality()?;
                lhs = format!("({} & {})", lhs, rhs);
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_equality(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_relational()?;
        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::EqEq | LowerToken::TripleEq => {
                    self.pos += 1;
                    let rhs = self.parse_relational()?;
                    lhs = format!("({} == {})", lhs, rhs);
                }
                LowerToken::NotEq | LowerToken::NotEqEq => {
                    self.pos += 1;
                    let rhs = self.parse_relational()?;
                    lhs = format!("({} != {})", lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_relational(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_shift()?;
        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::Lt => {
                    self.pos += 1;
                    let rhs = self.parse_shift()?;
                    lhs = format!("({} < {})", lhs, rhs);
                }
                LowerToken::LtEq => {
                    self.pos += 1;
                    let rhs = self.parse_shift()?;
                    lhs = format!("({} <= {})", lhs, rhs);
                }
                LowerToken::Gt => {
                    self.pos += 1;
                    let rhs = self.parse_shift()?;
                    lhs = format!("({} > {})", lhs, rhs);
                }
                LowerToken::GtEq => {
                    self.pos += 1;
                    let rhs = self.parse_shift()?;
                    lhs = format!("({} >= {})", lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_shift(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_additive()?;
        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::Shl => {
                    self.pos += 1;
                    let rhs = self.parse_additive()?;
                    lhs = format!("({} << {})", lhs, rhs);
                }
                LowerToken::Shr => {
                    self.pos += 1;
                    let rhs = self.parse_additive()?;
                    lhs = format!("({} >> {})", lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_additive(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_multiplicative()?;
        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::Plus => {
                    self.pos += 1;
                    let rhs = self.parse_multiplicative()?;
                    lhs = format!("({} + {})", lhs, rhs);
                }
                LowerToken::Minus => {
                    self.pos += 1;
                    let rhs = self.parse_multiplicative()?;
                    lhs = format!("({} - {})", lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_multiplicative(&mut self) -> Result<String, VerifyError> {
        let mut lhs = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::Star => {
                    self.pos += 1;
                    let rhs = self.parse_unary()?;
                    lhs = format!("haetae_checked_mul({}, {})", lhs, rhs);
                }
                LowerToken::Slash | LowerToken::DoubleSlash => {
                    self.pos += 1;
                    let rhs = self.parse_unary()?;
                    lhs = format!("haetae_checked_div({}, {})", lhs, rhs);
                }
                LowerToken::Percent => {
                    self.pos += 1;
                    let rhs = self.parse_unary()?;
                    lhs = format!("haetae_checked_mod({}, {})", lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<String, VerifyError> {
        match self.peek() {
            Some(LowerToken::Minus) => {
                self.pos += 1;
                let sub = self.parse_unary()?;
                Ok(format!("(-{})", sub))
            }
            Some(LowerToken::PlusPlus) => {
                self.pos += 1;
                let sub = self.parse_unary()?;
                Ok(format!("(++{})", sub))
            }
            Some(LowerToken::MinusMinus) => {
                self.pos += 1;
                let sub = self.parse_unary()?;
                Ok(format!("(--{})", sub))
            }
            Some(LowerToken::Bang) | Some(LowerToken::KeywordNot) => {
                self.pos += 1;
                let sub = self.parse_unary()?;
                Ok(format!("(!{})", sub))
            }
            Some(LowerToken::Tilde) => {
                self.pos += 1;
                let sub = self.parse_unary()?;
                Ok(format!("(~{})", sub))
            }
            Some(LowerToken::Star) => {
                self.pos += 1;
                let sub = self.parse_unary()?;
                Ok(format!("(*{})", sub))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<String, VerifyError> {
        let mut expr = self.parse_primary()?;

        while let Some(tok) = self.peek() {
            match tok {
                LowerToken::PlusPlus => {
                    self.pos += 1;
                    expr = format!("{}++", expr);
                }
                LowerToken::MinusMinus => {
                    self.pos += 1;
                    expr = format!("{}--", expr);
                }
                LowerToken::LBracket => {
                    self.pos += 1;
                    let idx_expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::RBracket);
                    if let Some(len_name) = self.known_arrays.get(&expr) {
                        expr =
                            format!("{}[haetae_checked_index({}, {})]", expr, len_name, idx_expr);
                    } else {
                        expr = format!("{}[{}]", expr, idx_expr);
                    }
                }
                LowerToken::LParen => {
                    self.pos += 1;
                    let mut args = Vec::new();
                    while let Some(t) = self.peek() {
                        if *t == LowerToken::RParen {
                            self.pos += 1;
                            break;
                        }
                        if *t == LowerToken::Comma {
                            self.pos += 1;
                            continue;
                        }
                        let arg = self.parse_expr()?;
                        args.push(arg);
                    }
                    expr = format!("{}({})", expr, args.join(", "));
                }
                LowerToken::Dot => {
                    self.pos += 1;
                    match self.next_tok() {
                        Some(LowerToken::Ident(field)) => {
                            expr = format!("{}.{}", expr, field);
                        }
                        other => {
                            return Err(VerifyError::UnsupportedSyntax(format!(
                                "Expected identifier after '.', got {:?}",
                                other
                            )));
                        }
                    }
                }
                LowerToken::Arrow => {
                    self.pos += 1;
                    match self.next_tok() {
                        Some(LowerToken::Ident(field)) => {
                            expr = format!("{}->{}", expr, field);
                        }
                        other => {
                            return Err(VerifyError::UnsupportedSyntax(format!(
                                "Expected identifier after '->', got {:?}",
                                other
                            )));
                        }
                    }
                }
                LowerToken::Question => {
                    self.pos += 1;
                    let true_expr = self.parse_expr()?;
                    self.match_tok(&LowerToken::Colon);
                    let false_expr = self.parse_expr()?;
                    expr = format!("(({}) ? ({}) : ({}))", expr, true_expr, false_expr);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<String, VerifyError> {
        let tok = match self.peek() {
            Some(t) => t.clone(),
            None => {
                return Err(VerifyError::UnsupportedSyntax(
                    "Unexpected EOF in primary expression".to_string(),
                ))
            }
        };
        self.pos += 1;
        match tok {
            LowerToken::Number(n) => Ok(format!("{}", n)),
            LowerToken::Ident(s) => {
                if s == "Math" {
                    if let [LowerToken::Dot, LowerToken::Ident(m), ..] = &self.tokens[self.pos..] {
                        if m == "floor" {
                            self.pos += 1;
                            self.pos += 1;
                            return self.parse_primary();
                        }
                    }
                }
                // Verification Tool 3: GENERIC_TYPE_H (type_of introspection)
                if (s == "type_of" || s == "typeof") && self.peek() == Some(&LowerToken::LParen) {
                    self.pos += 1;
                    let inner = self.parse_expr()?;
                    self.match_tok(&LowerToken::RParen);
                    return Ok(format!("type_of({})", inner));
                }
                Ok(s)
            }
            LowerToken::Str(s) => Ok(format!("\"{}\"", s)),
            LowerToken::KeywordTrue => Ok("1".to_string()),
            LowerToken::KeywordFalse => Ok("0".to_string()),
            LowerToken::KeywordNone => Ok("0".to_string()),
            LowerToken::LParen => {
                let inner = self.parse_expr()?;
                self.match_tok(&LowerToken::RParen);
                Ok(format!("({})", inner))
            }
            other => Err(VerifyError::UnsupportedSyntax(format!(
                "Unexpected token in expression: {:?}",
                other
            ))),
        }
    }
}

pub fn generate_c_fuzz_harness(func_name: &str, param_types: &[&str]) -> String {
    generate_multi_c_fuzz_harness(&[(func_name, param_types)])
}

pub fn generate_multi_c_fuzz_harness(funcs: &[(&str, &[&str])]) -> String {
    let count = funcs.len();
    if count == 0 {
        return "int haetae_fuzz_entry(long long payload) {\n  (void)payload;\n  return 0;\n}\n"
            .to_string();
    }

    let is_multi = count > 1;
    let mut out = String::new();

    // 64-bit Payload Derivation Helper (avalanche bit mixer with boundary mutations)
    out.push_str(
        r#"
static inline unsigned long long haetae_derive_payload(unsigned long long seed, unsigned int idx) {
    unsigned long long x = seed ^ (((unsigned long long)(idx + 1)) * 0x9e3779b97f4a7c15ULL);
    x ^= x >> 30;
    x *= 0xbf58476d1ce4e5b9ULL;
    x ^= x >> 27;
    x *= 0x94d049bb133111ebULL;
    x ^= x >> 31;
    unsigned int mode = (unsigned int)(x & 15);
    if (mode == 0) return 0ULL;
    if (mode == 1) return 1ULL;
    if (mode == 2) return ~0ULL;
    if (mode == 3) return 0x7fffffffffffffffULL;
    if (mode == 4) return 0x8000000000000000ULL;
    if (mode == 5) return 0x7fffffffULL;
    if (mode == 6) return (unsigned long long)(int)0x80000000;
    return x;
}
"#,
    );

    // Single-payload Entry Point
    out.push_str("int haetae_fuzz_entry(long long payload) {\n");
    if is_multi {
        out.push_str("  unsigned long long up = (unsigned long long)payload;\n");
        out.push_str(&format!(
            "  unsigned int sel = (unsigned int)(up % {});\n",
            count
        ));
        out.push_str(&format!("  long long p = (long long)(up / {});\n", count));
        out.push_str("  switch (sel) {\n");
    }

    for (idx, (fn_name, param_types)) in funcs.iter().enumerate() {
        let indent = if is_multi { "      " } else { "  " };
        if is_multi {
            out.push_str(&format!("    case {}: {{\n", idx));
        }

        let mut declarations = Vec::new();
        let mut call_args = Vec::new();

        for (p_idx, ptype) in param_types.iter().enumerate() {
            let var_name = format!("p{}", p_idx);
            let c_type = match *ptype {
                "int" | "i32" | "number" | "Int" => "int",
                "unsigned int" | "u32" => "unsigned int",
                "long" | "i64" => "long long",
                "unsigned long" | "u64" => "unsigned long long",
                "float" | "f32" => "float",
                "double" | "f64" => "double",
                "char*" | "&str" | "string" | "String" => "const char*",
                "void*" | "*mut" | "*const" | "ptr" => "void*",
                _ => "int",
            };

            let src_var = if is_multi { "p" } else { "payload" };

            if p_idx == 0 {
                match c_type {
                    "float" | "double" => declarations
                        .push(format!("{} {} = ({}){}", c_type, var_name, c_type, src_var)),
                    "const char*" => declarations.push(format!(
                        "{} {} = ({} == 0) ? (const char*)0 : \"test_payload\"",
                        c_type, var_name, src_var
                    )),
                    "void*" => declarations.push(format!(
                        "{} {} = ({} == 0) ? (void*)0 : (void*)1",
                        c_type, var_name, src_var
                    )),
                    _ => declarations
                        .push(format!("{} {} = ({}){}", c_type, var_name, c_type, src_var)),
                }
            } else {
                match c_type {
                    "float" | "double" => declarations.push(format!(
                        "{} {} = (double)((long long)haetae_derive_payload((unsigned long long){}, {}))",
                        c_type, var_name, src_var, p_idx
                    )),
                    "const char*" => declarations.push(format!(
                        "{} {} = (haetae_derive_payload((unsigned long long){}, {}) % 2 == 0) ? (const char*)0 : \"test_payload\"",
                        c_type, var_name, src_var, p_idx
                    )),
                    "void*" => declarations.push(format!(
                        "{} {} = (haetae_derive_payload((unsigned long long){}, {}) % 2 == 0) ? (void*)0 : (void*)1",
                        c_type, var_name, src_var, p_idx
                    )),
                    _ => declarations.push(format!(
                        "{} {} = ({})haetae_derive_payload((unsigned long long){}, {})",
                        c_type, var_name, c_type, src_var, p_idx
                    )),
                }
            }

            call_args.push(var_name);
        }

        if !declarations.is_empty() {
            out.push_str(indent);
            out.push_str(&declarations.join("; "));
            out.push_str(";\n");
        } else if !is_multi {
            out.push_str("  (void)payload;\n");
        } else {
            out.push_str("      (void)p;\n");
        }

        out.push_str(&format!(
            "{}volatile long long res = (long long){}({});\n",
            indent,
            fn_name,
            call_args.join(", ")
        ));
        out.push_str(&format!("{}return (int)(res & 1);\n", indent));

        if is_multi {
            out.push_str("    }\n");
        }
    }

    if is_multi {
        out.push_str("    default:\n      return 0;\n  }\n");
    }
    out.push_str("}\n\n");

    // Multi-payload Entry Point
    out.push_str("int haetae_fuzz_multi_entry(const long long* payloads, int count) {\n");
    if funcs.is_empty() {
        out.push_str("  (void)payloads; (void)count; return 0;\n}\n");
        return out;
    }

    out.push_str("  if (count <= 0 || payloads == 0) return 0;\n");
    if is_multi {
        out.push_str(&format!(
            "  unsigned int sel = (unsigned int)((unsigned long long)payloads[0] % {});\n",
            count
        ));
        out.push_str("  switch (sel) {\n");
    }

    for (idx, (fn_name, param_types)) in funcs.iter().enumerate() {
        let indent = if is_multi { "      " } else { "  " };
        if is_multi {
            out.push_str(&format!("    case {}: {{\n", idx));
        }

        let mut declarations = Vec::new();
        let mut call_args = Vec::new();

        for (i, ptype) in param_types.iter().enumerate() {
            let var_name = format!("p{}", i);
            let c_type = match *ptype {
                "int" | "i32" | "number" | "Int" => "int",
                "unsigned int" | "u32" => "unsigned int",
                "long" | "i64" => "long long",
                "unsigned long" | "u64" => "unsigned long long",
                "float" | "f32" => "float",
                "double" | "f64" => "double",
                "char*" | "&str" | "string" | "String" => "const char*",
                "void*" | "*mut" | "*const" | "ptr" => "void*",
                _ => "int",
            };

            if i == 0 {
                declarations.push(format!(
                    "{} {} = (count > 0 && payloads != 0) ? ({})payloads[0] : 0",
                    c_type, var_name, c_type
                ));
            } else {
                declarations.push(format!(
                    "{} {} = (count > {} && payloads != 0) ? ({})payloads[{}] : ({})haetae_derive_payload((unsigned long long)payloads[0], {})",
                    c_type, var_name, i, c_type, i, c_type, i
                ));
            }
            call_args.push(var_name);
        }

        if !declarations.is_empty() {
            out.push_str(indent);
            out.push_str(&declarations.join("; "));
            out.push_str(";\n");
        }

        out.push_str(&format!(
            "{}volatile long long res = (long long){}({});\n",
            indent,
            fn_name,
            call_args.join(", ")
        ));
        out.push_str(&format!("{}return (int)(res & 1);\n", indent));

        if is_multi {
            out.push_str("    }\n");
        }
    }

    if is_multi {
        out.push_str("    default:\n      return 0;\n  }\n");
    }
    out.push_str("}\n");

    out
}

pub fn generate_fuzz_payload_assignment(param_index: usize, c_type: &str, val: i64) -> String {
    let var_name = format!("p{}", param_index);
    match c_type {
        "float" | "double" => format!("{} = (double){};", var_name, val),
        "const char*" => {
            if val == 0 {
                format!("{} = 0;", var_name)
            } else {
                format!("{} = \"test_payload\";", var_name)
            }
        }
        "void*" => {
            if val == 0 {
                format!("{} = 0;", var_name)
            } else {
                format!("{} = (void*)1;", var_name)
            }
        }
        _ => format!("{} = {};", var_name, val),
    }
}

pub fn rewrite_c_divisions(source: &str) -> String {
    let mut lexer = LowerLexer::new(source, "c");
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return source.to_string(),
    };

    let mut lowerer = StructuredCLowerer::new(&tokens);
    let mut rewritten = Vec::new();
    while lowerer.pos < lowerer.tokens.len() {
        if let Ok((_, c_code, _)) = lowerer.parse_function() {
            rewritten.push(c_code);
        } else {
            lowerer.pos += 1;
        }
    }

    if rewritten.is_empty() {
        source.to_string()
    } else {
        rewritten.join("\n\n")
    }
}

pub fn lower_source_to_c_ir(source: &str, lang_or_ext: &str) -> String {
    match try_lower_source_to_c_ir(source, lang_or_ext) {
        Ok(c) => c,
        Err(e) => format!("#error \"Haetae Lowering Error: {}\"\n", e),
    }
}

pub fn try_lower_source_to_c_ir(source: &str, lang_or_ext: &str) -> Result<String, VerifyError> {
    let guard_prefix = crate::headers::bundle_headers();

    if source.contains("haetae_fuzz_entry") {
        return Ok(source.to_string());
    }

    let lang_lower = lang_or_ext.to_lowercase();
    let mut lexer = LowerLexer::new(source, &lang_lower);
    let tokens = lexer.tokenize()?;

    let mut lowerer = StructuredCLowerer::new(&tokens);
    let mut lowered_items = Vec::new();
    let mut fn_descriptors: Vec<(String, Vec<String>)> = Vec::new();

    while lowerer.pos < lowerer.tokens.len() {
        if lowerer.match_tok(&LowerToken::Semi) {
            continue;
        }

        if let Some(LowerToken::Preprocessor(p)) = lowerer.peek() {
            lowered_items.push(p.clone());
            lowerer.pos += 1;
            continue;
        }

        if lowerer.peek() == Some(&LowerToken::KeywordStruct)
            || lowerer.peek() == Some(&LowerToken::KeywordClass)
        {
            match lowerer.parse_struct() {
                Ok(st) => {
                    let mut s_out = format!("typedef struct {} {{\n", st.name);
                    for (fn_field, ft) in st.fields {
                        s_out.push_str(&format!("  {} {};\n", ft, fn_field));
                    }
                    s_out.push_str(&format!("}} {};\n", st.name));
                    lowered_items.push(s_out);
                    continue;
                }
                Err(e) => {
                    if lowered_items.is_empty() {
                        return Err(e);
                    }
                    break;
                }
            }
        }

        if let [LowerToken::KeywordType, LowerToken::KeywordStruct, ..] = &lowerer.tokens[lowerer.pos..] {
            lowerer.pos += 1; // skip type
            match lowerer.parse_struct() {
                Ok(st) => {
                    let mut s_out = format!("typedef struct {} {{\n", st.name);
                    for (fn_field, ft) in st.fields {
                        s_out.push_str(&format!("  {} {};\n", ft, fn_field));
                    }
                    s_out.push_str(&format!("}} {};\n", st.name));
                    lowered_items.push(s_out);
                    continue;
                }
                Err(e) => {
                    if lowered_items.is_empty() {
                        return Err(e);
                    }
                    break;
                }
            }
        }

        match lowerer.parse_function() {
            Ok((fn_name, c_code, params)) => {
                lowered_items.push(c_code);
                let p_types = params.iter().map(|(_, ty)| ty.clone()).collect();
                fn_descriptors.push((fn_name, p_types));
            }
            Err(e) => {
                if lowered_items.is_empty() {
                    return Err(e);
                }
                break;
            }
        }
    }

    if fn_descriptors.is_empty() {
        return Err(VerifyError::UnsupportedSyntax(
            "Failed to parse any valid functions in source code".to_string(),
        ));
    }

    let all_c_code = lowered_items.join("\n\n");
    let targets: Vec<(&str, Vec<&str>)> = fn_descriptors
        .iter()
        .map(|(n, p)| (n.as_str(), p.iter().map(|s| s.as_str()).collect()))
        .collect();
    let slice_targets: Vec<(&str, &[&str])> =
        targets.iter().map(|(n, p)| (*n, p.as_slice())).collect();
    let harness = generate_multi_c_fuzz_harness(&slice_targets);

    Ok(format!("{}\n{}\n{}\n", guard_prefix, all_c_code, harness))
}


