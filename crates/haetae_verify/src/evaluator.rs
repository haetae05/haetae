use crate::tcc_engine::VerifyError;
use haetae_core::config::{
    MAX_CALL_STACK_DEPTH, MAX_STACK_FRAME_BYTES, NUM_EIGHT, NUM_FOUR, NUM_ONE, NUM_TWO, NUM_ZERO,
    POSIX_SIGFPE, POSIX_SIGSEGV,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Exclamation,
    Shl,
    Shr,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Question,
    Colon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semi,
    Assign,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = NUM_ZERO;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += NUM_ONE;
            continue;
        }

        if c.is_ascii_digit() {
            let mut num_str = String::new();
            if c == '0'
                && i + NUM_ONE < chars.len()
                && (chars[i + NUM_ONE] == 'x' || chars[i + NUM_ONE] == 'X')
            {
                i += NUM_TWO;
                while i < chars.len() && chars[i].is_ascii_hexdigit() {
                    num_str.push(chars[i]);
                    i += NUM_ONE;
                }
                while i < chars.len()
                    && (chars[i] == 'u' || chars[i] == 'U' || chars[i] == 'l' || chars[i] == 'L')
                {
                    i += NUM_ONE;
                }
                let val = i64::from_str_radix(&num_str, haetae_core::config::HEX_RADIX)
                    .unwrap_or(NUM_ZERO as i64);
                tokens.push(Token::Number(val));
                continue;
            }

            while i < chars.len() && chars[i].is_ascii_digit() {
                num_str.push(chars[i]);
                i += NUM_ONE;
            }
            while i < chars.len()
                && (chars[i] == 'u'
                    || chars[i] == 'U'
                    || chars[i] == 'l'
                    || chars[i] == 'L'
                    || chars[i] == 'f'
                    || chars[i] == '.')
            {
                i += NUM_ONE;
            }
            let val = num_str.parse::<i64>().unwrap_or(NUM_ZERO as i64);
            tokens.push(Token::Number(val));
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let mut id = String::new();
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                id.push(chars[i]);
                i += NUM_ONE;
            }
            tokens.push(Token::Ident(id));
            continue;
        }

        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Percent),
            '~' => tokens.push(Token::Tilde),
            '?' => tokens.push(Token::Question),
            ':' => tokens.push(Token::Colon),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semi),
            '&' => {
                if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '&' {
                    i += NUM_ONE;
                    tokens.push(Token::Amp);
                } else {
                    tokens.push(Token::Amp);
                }
            }
            '|' => {
                if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '|' {
                    i += NUM_ONE;
                    tokens.push(Token::Pipe);
                } else {
                    tokens.push(Token::Pipe);
                }
            }
            '^' => tokens.push(Token::Caret),
            '!' => {
                if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '=' {
                    i += NUM_ONE;
                    tokens.push(Token::NotEq);
                } else {
                    tokens.push(Token::Exclamation);
                }
            }
            '=' => {
                if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '=' {
                    i += NUM_ONE;
                    tokens.push(Token::EqEq);
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '<' => {
                if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '<' {
                    i += NUM_ONE;
                    tokens.push(Token::Shl);
                } else if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '=' {
                    i += NUM_ONE;
                    tokens.push(Token::LtEq);
                } else {
                    tokens.push(Token::Lt);
                }
            }
            '>' => {
                if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '>' {
                    i += NUM_ONE;
                    tokens.push(Token::Shr);
                } else if i + NUM_ONE < chars.len() && chars[i + NUM_ONE] == '=' {
                    i += NUM_ONE;
                    tokens.push(Token::GtEq);
                } else {
                    tokens.push(Token::Gt);
                }
            }
            _ => {}
        }
        i += NUM_ONE;
    }

    tokens
}

pub struct ExprParser<'a> {
    tokens: &'a [Token],
    pos: usize,
    env: &'a mut HashMap<String, i64>,
    source: Option<&'a str>,
    call_depth: usize,
}

impl<'a> ExprParser<'a> {
    pub fn new(tokens: &'a [Token], env: &'a mut HashMap<String, i64>) -> Self {
        Self {
            tokens,
            pos: NUM_ZERO,
            env,
            source: None,
            call_depth: NUM_ZERO,
        }
    }

    pub fn with_context(
        tokens: &'a [Token],
        env: &'a mut HashMap<String, i64>,
        source: Option<&'a str>,
        call_depth: usize,
    ) -> Self {
        Self {
            tokens,
            pos: NUM_ZERO,
            env,
            source,
            call_depth,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += NUM_ONE;
        }
        tok
    }

    pub fn parse_expression(&mut self) -> Result<i64, VerifyError> {
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> Result<i64, VerifyError> {
        let mut cond = self.parse_logical_or()?;
        if let Some(Token::Question) = self.peek() {
            self.next();
            let true_val = self.parse_expression()?;
            if let Some(Token::Colon) = self.peek() {
                self.next();
            }
            let false_val = self.parse_expression()?;
            cond = if cond != NUM_ZERO as i64 {
                true_val
            } else {
                false_val
            };
        }
        Ok(cond)
    }

    fn parse_logical_or(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_logical_and()?;
        while let Some(Token::Pipe) = self.peek() {
            self.next();
            let rhs = self.parse_logical_and()?;
            lhs |= rhs;
        }
        Ok(lhs)
    }

    fn parse_logical_and(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_bitwise_xor()?;
        while let Some(Token::Amp) = self.peek() {
            self.next();
            let rhs = self.parse_bitwise_xor()?;
            lhs &= rhs;
        }
        Ok(lhs)
    }

    fn parse_bitwise_xor(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_equality()?;
        while let Some(Token::Caret) = self.peek() {
            self.next();
            let rhs = self.parse_equality()?;
            lhs ^= rhs;
        }
        Ok(lhs)
    }

    fn parse_equality(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_relational()?;
        while let Some(tok) = self.peek().cloned() {
            match tok {
                Token::EqEq => {
                    self.next();
                    let rhs = self.parse_relational()?;
                    lhs = if lhs == rhs {
                        NUM_ONE as i64
                    } else {
                        NUM_ZERO as i64
                    };
                }
                Token::NotEq => {
                    self.next();
                    let rhs = self.parse_relational()?;
                    lhs = if lhs != rhs {
                        NUM_ONE as i64
                    } else {
                        NUM_ZERO as i64
                    };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_relational(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_shift()?;
        while let Some(tok) = self.peek().cloned() {
            match tok {
                Token::Lt => {
                    self.next();
                    let rhs = self.parse_shift()?;
                    lhs = if lhs < rhs {
                        NUM_ONE as i64
                    } else {
                        NUM_ZERO as i64
                    };
                }
                Token::LtEq => {
                    self.next();
                    let rhs = self.parse_shift()?;
                    lhs = if lhs <= rhs {
                        NUM_ONE as i64
                    } else {
                        NUM_ZERO as i64
                    };
                }
                Token::Gt => {
                    self.next();
                    let rhs = self.parse_shift()?;
                    lhs = if lhs > rhs {
                        NUM_ONE as i64
                    } else {
                        NUM_ZERO as i64
                    };
                }
                Token::GtEq => {
                    self.next();
                    let rhs = self.parse_shift()?;
                    lhs = if lhs >= rhs {
                        NUM_ONE as i64
                    } else {
                        NUM_ZERO as i64
                    };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_shift(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_additive()?;
        while let Some(tok) = self.peek().cloned() {
            match tok {
                Token::Shl => {
                    self.next();
                    let rhs = self.parse_additive()?;
                    let safe_shift = (rhs & (haetae_core::config::NUM_THIRTY_ONE as i64)) as u32;
                    lhs = lhs.wrapping_shl(safe_shift);
                }
                Token::Shr => {
                    self.next();
                    let rhs = self.parse_additive()?;
                    let safe_shift = (rhs & (haetae_core::config::NUM_THIRTY_ONE as i64)) as u32;
                    lhs = lhs.wrapping_shr(safe_shift);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_additive(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_multiplicative()?;
        while let Some(tok) = self.peek().cloned() {
            match tok {
                Token::Plus => {
                    self.next();
                    let rhs = self.parse_multiplicative()?;
                    lhs = lhs.wrapping_add(rhs);
                }
                Token::Minus => {
                    self.next();
                    let rhs = self.parse_multiplicative()?;
                    lhs = lhs.wrapping_sub(rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_multiplicative(&mut self) -> Result<i64, VerifyError> {
        let mut lhs = self.parse_unary()?;
        while let Some(tok) = self.peek().cloned() {
            match tok {
                Token::Star => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    match lhs.checked_mul(rhs) {
                        Some(val) => lhs = val,
                        None => return Err(VerifyError::CrashSignal(POSIX_SIGFPE)),
                    }
                }
                Token::Slash => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    if rhs == NUM_ZERO as i64 {
                        return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                    }
                    lhs = lhs.wrapping_div(rhs);
                }
                Token::Percent => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    if rhs == NUM_ZERO as i64 {
                        return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                    }
                    lhs = lhs.wrapping_rem(rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<i64, VerifyError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.next();
                let val = self.parse_unary()?;
                Ok(val.wrapping_neg())
            }
            Some(Token::Tilde) => {
                self.next();
                let val = self.parse_unary()?;
                Ok(!val)
            }
            Some(Token::Exclamation) => {
                self.next();
                let val = self.parse_unary()?;
                Ok(if val == NUM_ZERO as i64 {
                    NUM_ONE as i64
                } else {
                    NUM_ZERO as i64
                })
            }
            Some(Token::Star) => {
                self.next();
                let ptr_val = self.parse_unary()?;
                if ptr_val == NUM_ZERO as i64 {
                    return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                }
                let val = self
                    .env
                    .get(&format!("ptr_{}", ptr_val))
                    .copied()
                    .unwrap_or(ptr_val);
                Ok(val)
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<i64, VerifyError> {
        match self.next().cloned() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::Ident(name)) => {
                if name == "haetae_trap_sigfpe" {
                    if let Some(Token::LParen) = self.peek() {
                        self.next();
                        if let Some(Token::RParen) = self.peek() {
                            self.next();
                        }
                    }
                    return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                }

                if name == "haetae_aarch64_div_guard" {
                    if let Some(Token::LParen) = self.peek() {
                        self.next();
                        let arg = self.parse_expression()?;
                        if let Some(Token::RParen) = self.peek() {
                            self.next();
                        }
                        if arg == NUM_ZERO as i64 {
                            return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                        }
                    }
                    return Ok(NUM_ZERO as i64);
                }

                if name == "haetae_checked_div" {
                    if let Some(Token::LParen) = self.peek() {
                        self.next();
                        let a = self.parse_expression()?;
                        if let Some(Token::Comma) = self.peek() {
                            self.next();
                        }
                        let b = self.parse_expression()?;
                        if let Some(Token::RParen) = self.peek() {
                            self.next();
                        }
                        if b == NUM_ZERO as i64 {
                            return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                        }
                        return Ok(a.wrapping_div(b));
                    }
                }

                if name == "haetae_checked_mod" {
                    if let Some(Token::LParen) = self.peek() {
                        self.next();
                        let a = self.parse_expression()?;
                        if let Some(Token::Comma) = self.peek() {
                            self.next();
                        }
                        let b = self.parse_expression()?;
                        if let Some(Token::RParen) = self.peek() {
                            self.next();
                        }
                        if b == NUM_ZERO as i64 {
                            return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                        }
                        return Ok(a.wrapping_rem(b));
                    }
                }

                if name == "haetae_checked_index" {
                    if let Some(Token::LParen) = self.peek() {
                        self.next();
                        let len = self.parse_expression()?;
                        if let Some(Token::Comma) = self.peek() {
                            self.next();
                        }
                        let idx = self.parse_expression()?;
                        if let Some(Token::RParen) = self.peek() {
                            self.next();
                        }
                        if idx < NUM_ZERO as i64 || idx >= len {
                            return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                        }
                        return Ok(idx);
                    }
                }

                if let Some(Token::LParen) = self.peek() {
                    self.next();
                    let mut args = Vec::new();
                    while let Some(tok) = self.peek() {
                        if *tok == Token::RParen {
                            self.next();
                            break;
                        }
                        let arg_val = self.parse_expression()?;
                        args.push(arg_val);
                        if let Some(Token::Comma) = self.peek() {
                            self.next();
                        }
                    }
                    return self.call_function(&name, &args);
                }

                if let Some(Token::LBracket) = self.peek() {
                    self.next();
                    let idx = self.parse_expression()?;
                    if let Some(Token::RBracket) = self.peek() {
                        self.next();
                    }
                    if let Some(len) = self.env.get(&format!("{}_len", name)).copied() {
                        if idx < NUM_ZERO as i64 || idx >= len {
                            return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                        }
                    }
                    let key = format!("{}[{}]", name, idx);
                    let val = self.env.get(&key).copied().unwrap_or(NUM_ZERO as i64);
                    return Ok(val);
                }

                let val = self.env.get(&name).copied().unwrap_or(NUM_ZERO as i64);
                Ok(val)
            }
            Some(Token::LParen) => {
                let is_cast = match self.peek() {
                    Some(Token::Ident(t)) => {
                        t == "int" || t == "long" || t == "double" || t == "float" || t == "void"
                    }
                    _ => false,
                };
                if is_cast {
                    self.next();
                    while let Some(Token::Star) = self.peek() {
                        self.next();
                    }
                    if let Some(Token::RParen) = self.peek() {
                        self.next();
                    }
                    return self.parse_unary();
                }
                let val = self.parse_expression()?;
                if let Some(Token::RParen) = self.peek() {
                    self.next();
                }
                Ok(val)
            }
            _ => Ok(NUM_ZERO as i64),
        }
    }

    fn call_function(&mut self, fn_name: &str, args: &[i64]) -> Result<i64, VerifyError> {
        if self.call_depth >= MAX_CALL_STACK_DEPTH {
            return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
        }

        if let Some(src) = self.source {
            if let Some((params, body)) = find_function_params_and_body(src, fn_name) {
                let mut callee_env = self.env.clone();
                for (i, p) in params.iter().enumerate() {
                    let val = if i < args.len() {
                        args[i]
                    } else {
                        NUM_ZERO as i64
                    };
                    callee_env.insert(p.clone(), val);
                    callee_env.insert(format!("p{}", i), val);
                }
                let res = execute_function_lines(
                    &body,
                    &mut callee_env,
                    self.source,
                    self.call_depth + NUM_ONE,
                )?;
                return Ok(res as i64);
            }
        }

        if fn_name == "abs" {
            return Ok(args.first().copied().unwrap_or(NUM_ZERO as i64).abs());
        }

        Ok(NUM_ZERO as i64)
    }
}

pub fn get_type_size(type_str: &str) -> usize {
    let t = type_str.trim();
    if t.contains('*') {
        return NUM_EIGHT;
    }
    if t.contains("char") || t.contains("bool") || t.contains("int8") {
        NUM_ONE
    } else if t.contains("short") || t.contains("int16") {
        NUM_TWO
    } else if t.contains("long long")
        || t.contains("int64")
        || t.contains("double")
        || t.contains("size_t")
    {
        NUM_EIGHT
    } else {
        NUM_FOUR
    }
}

pub fn find_function_params_and_body(
    source: &str,
    fn_name: &str,
) -> Option<(Vec<String>, Vec<String>)> {
    let mut lines_iter = source.lines();
    while let Some(line) = lines_iter.next() {
        let trimmed = line.trim();
        let name_match_prefix = format!(" {}(", fn_name);
        let starts_with_prefix = format!("{}(", fn_name);
        let fn_offset = if trimmed.starts_with(&starts_with_prefix) {
            Some(NUM_ZERO)
        } else {
            trimmed.find(&name_match_prefix).map(|idx| idx + NUM_ONE)
        };

        if let Some(offset) = fn_offset {
            let after_fn = &trimmed[offset..];
            let is_proto = after_fn
                .find(';')
                .is_some_and(|s_idx| after_fn.find('{').is_none_or(|b_idx| s_idx < b_idx));

            if !is_proto {
                let mut params = Vec::new();
                if let Some(open_p) = after_fn.find('(') {
                    if let Some(close_p) = after_fn[open_p..].find(')') {
                        let param_str = &after_fn[open_p + NUM_ONE..open_p + close_p];
                        for p in param_str.split(',') {
                            let p_trim = p.trim();
                            if !p_trim.is_empty() && p_trim != "void" {
                                if let Some(last_tok) = p_trim.split_whitespace().last() {
                                    let cleaned = last_tok
                                        .trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                                    if !cleaned.is_empty() {
                                        params.push(cleaned.to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                let mut body = Vec::new();
                let mut brace_start = None;
                let mut brace_end = None;
                let mut depth = NUM_ZERO;

                for (idx, ch) in after_fn.char_indices() {
                    if ch == '{' {
                        if depth == NUM_ZERO {
                            brace_start = Some(idx);
                        }
                        depth += NUM_ONE;
                    } else if ch == '}' && depth > NUM_ZERO {
                        depth -= NUM_ONE;
                        if depth == NUM_ZERO {
                            brace_end = Some(idx);
                            break;
                        }
                    }
                }

                if let (Some(b_start), Some(b_end)) = (brace_start, brace_end) {
                    let inside = after_fn[b_start + NUM_ONE..b_end].trim();
                    for stmt in inside.split(';') {
                        let st = stmt.trim();
                        if !st.is_empty() {
                            body.push(format!("{};", st));
                        }
                    }
                    return Some((params, body));
                }

                if let Some(b_start) = brace_start {
                    let rest = after_fn[b_start + NUM_ONE..].trim();
                    if !rest.is_empty() && rest != "}" {
                        body.push(rest.to_string());
                    }
                }

                for inner in lines_iter.by_ref() {
                    let inner_trimmed = inner.trim();
                    for ch in inner_trimmed.chars() {
                        if ch == '{' {
                            depth += NUM_ONE;
                        } else if ch == '}' && depth > NUM_ZERO {
                            depth -= NUM_ONE;
                        }
                    }
                    if depth == NUM_ZERO {
                        break;
                    }
                    if !inner_trimmed.is_empty() && inner_trimmed != "{" {
                        body.push(inner_trimmed.to_string());
                    }
                }
                return Some((params, body));
            }
        }
    }
    None
}

pub fn execute_function_lines(
    lines: &[String],
    env: &mut HashMap<String, i64>,
    source: Option<&str>,
    call_depth: usize,
) -> Result<i32, VerifyError> {
    let mut local_stack_bytes: usize = NUM_ZERO;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.contains("haetae_trap_sigfpe();") {
            return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
        }

        if trimmed.starts_with("if ") || trimmed.starts_with("if(") {
            if let Some(open_p) = trimmed.find('(') {
                if let Some(close_p) = trimmed.rfind(')') {
                    let cond_str = &trimmed[open_p + NUM_ONE..close_p];
                    let cond_toks = tokenize(cond_str);
                    let mut parser = ExprParser::with_context(&cond_toks, env, source, call_depth);
                    let cond_res = parser.parse_expression()?;

                    if cond_res != NUM_ZERO as i64 {
                        let raw_rest = trimmed[close_p + NUM_ONE..].trim();
                        let rest = raw_rest
                            .trim_start_matches('{')
                            .trim_end_matches('}')
                            .trim();
                        if rest.starts_with("return ") {
                            let expr_str = rest.trim_start_matches("return ").trim_matches(';');
                            let expr_toks = tokenize(expr_str);
                            let mut p =
                                ExprParser::with_context(&expr_toks, env, source, call_depth);
                            let ret_val = p.parse_expression()?;
                            return Ok(ret_val as i32);
                        } else if rest.contains("haetae_trap_sigfpe") {
                            return Err(VerifyError::CrashSignal(POSIX_SIGFPE));
                        }
                    } else {
                        continue;
                    }
                }
            }
        }

        if trimmed.starts_with("return ") {
            let expr_str = trimmed.trim_start_matches("return ").trim_matches(';');
            let expr_toks = tokenize(expr_str);
            let mut parser = ExprParser::with_context(&expr_toks, env, source, call_depth);
            let ret_val = parser.parse_expression()?;
            return Ok(ret_val as i32);
        }

        if trimmed.starts_with('*') {
            let expr_str = trimmed.trim_end_matches(';');
            let expr_toks = tokenize(expr_str);
            let mut parser = ExprParser::with_context(&expr_toks, env, source, call_depth);
            let _ = parser.parse_expression()?;
        }

        if trimmed.contains('[')
            && trimmed.contains(']')
            && trimmed.contains('=')
            && trimmed.contains('{')
            && trimmed.ends_with(';')
        {
            let decl = trimmed.trim_end_matches(';');
            if let Some(brace_open) = decl.find('{') {
                if let Some(brace_close) = decl.find('}') {
                    let items_str = &decl[brace_open + NUM_ONE..brace_close];
                    let items: Vec<&str> = items_str.split(',').collect();
                    let before_eq = decl[..decl.find('=').unwrap_or(NUM_ZERO)].trim();
                    let type_size = get_type_size(before_eq);
                    let name_part = before_eq.split_whitespace().last().unwrap_or("");
                    if let Some(br_open) = name_part.find('[') {
                        let arr_name = &name_part[..br_open];
                        let br_close = name_part.find(']').unwrap_or(name_part.len());
                        let declared_len = if br_close > br_open + NUM_ONE {
                            name_part[br_open + NUM_ONE..br_close]
                                .trim()
                                .parse::<i64>()
                                .unwrap_or(items.len() as i64)
                        } else {
                            items.len() as i64
                        };
                        let byte_size = type_size.saturating_mul(declared_len as usize);
                        local_stack_bytes = local_stack_bytes.saturating_add(byte_size);
                        if local_stack_bytes > MAX_STACK_FRAME_BYTES {
                            return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                        }
                        env.insert(format!("{}_len", arr_name), declared_len);
                        for (idx, itm) in items.iter().enumerate() {
                            let expr_toks = tokenize(itm);
                            let mut parser =
                                ExprParser::with_context(&expr_toks, env, source, call_depth);
                            let v = parser.parse_expression().unwrap_or(NUM_ZERO as i64);
                            env.insert(format!("{}[{}]", arr_name, idx), v);
                        }
                        continue;
                    }
                }
            }
        }

        if trimmed.contains('[') && trimmed.contains(']') && !trimmed.contains('=') {
            let before_semi = trimmed.trim_end_matches(';');
            let words: Vec<&str> = before_semi.split_whitespace().collect();
            if words.len() >= NUM_TWO {
                let type_part = words[..words.len() - NUM_ONE].join(" ");
                let type_size = get_type_size(&type_part);
                if let Some(var_part) = words.last() {
                    if let (Some(br_o), Some(br_c)) = (var_part.find('['), var_part.find(']')) {
                        if br_o < br_c {
                            let size_str = &var_part[br_o + NUM_ONE..br_c];
                            let elem_count = size_str.trim().parse::<usize>().unwrap_or(NUM_ZERO);
                            let byte_size = type_size.saturating_mul(elem_count);
                            local_stack_bytes = local_stack_bytes.saturating_add(byte_size);
                            if local_stack_bytes > MAX_STACK_FRAME_BYTES {
                                return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                            }
                            let arr_name = &var_part[..br_o];
                            env.insert(format!("{}_len", arr_name), elem_count as i64);
                            continue;
                        }
                    }
                }
            }
        }

        if trimmed.contains('[')
            && trimmed.contains(']')
            && trimmed.contains('=')
            && !trimmed.contains('{')
            && trimmed.ends_with(';')
        {
            let stmt = trimmed.trim_end_matches(';');
            let parts: Vec<&str> = stmt.splitn(NUM_TWO, '=').collect();
            if parts.len() == NUM_TWO {
                let left = parts[NUM_ZERO].trim();
                if let (Some(b_o), Some(b_c)) = (left.find('['), left.find(']')) {
                    let arr_name = &left[..b_o];
                    let idx_str = &left[b_o + NUM_ONE..b_c];
                    let idx_toks = tokenize(idx_str);
                    let mut p_idx = ExprParser::with_context(&idx_toks, env, source, call_depth);
                    let idx = p_idx.parse_expression().unwrap_or(NUM_ZERO as i64);
                    if let Some(len) = env.get(&format!("{}_len", arr_name)).copied() {
                        if idx < NUM_ZERO as i64 || idx >= len {
                            return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                        }
                    }
                    let rhs_toks = tokenize(parts[NUM_ONE]);
                    let mut p_rhs = ExprParser::with_context(&rhs_toks, env, source, call_depth);
                    let val = p_rhs.parse_expression()?;
                    env.insert(format!("{}[{}]", arr_name, idx), val);
                    continue;
                }
            }
        }

        let is_var_decl = trimmed.starts_with("int ")
            || trimmed.starts_with("int*")
            || trimmed.starts_with("long ")
            || trimmed.starts_with("long*")
            || trimmed.starts_with("unsigned int ")
            || trimmed.starts_with("float ")
            || trimmed.starts_with("double ")
            || trimmed.starts_with("const char*")
            || trimmed.starts_with("char ")
            || trimmed.starts_with("char*")
            || trimmed.starts_with("void*");

        if is_var_decl && trimmed.contains('=') && trimmed.ends_with(';') {
            let decl = trimmed.trim_end_matches(';');
            let parts: Vec<&str> = decl.splitn(NUM_TWO, '=').collect();
            if parts.len() == NUM_TWO {
                let type_size = get_type_size(parts[NUM_ZERO]);
                local_stack_bytes = local_stack_bytes.saturating_add(type_size);
                if local_stack_bytes > MAX_STACK_FRAME_BYTES {
                    return Err(VerifyError::CrashSignal(POSIX_SIGSEGV));
                }
                let left_words: Vec<&str> = parts[NUM_ZERO].split_whitespace().collect();
                if let Some(var_name) = left_words.last() {
                    let cleaned_name = var_name.trim_start_matches('*').trim();
                    let expr_toks = tokenize(parts[NUM_ONE]);
                    let mut parser = ExprParser::with_context(&expr_toks, env, source, call_depth);
                    let val = parser.parse_expression()?;
                    env.insert(cleaned_name.to_string(), val);
                    continue;
                }
            }
        }

        if !is_var_decl && trimmed.contains('=') && trimmed.ends_with(';') {
            let decl = trimmed.trim_end_matches(';');
            let parts: Vec<&str> = decl.splitn(NUM_TWO, '=').collect();
            if parts.len() == NUM_TWO {
                let var_name = parts[NUM_ZERO].trim();
                let expr_toks = tokenize(parts[NUM_ONE]);
                let mut parser = ExprParser::with_context(&expr_toks, env, source, call_depth);
                let val = parser.parse_expression()?;
                env.insert(var_name.to_string(), val);
                continue;
            }
        }
    }

    Ok(NUM_ZERO as i32)
}

pub fn evaluate_c_source(source: &str, payload: i64) -> Result<i32, VerifyError> {
    let prefix = "volatile long long res = (long long)";
    let mut callees = Vec::new();
    let mut search_idx = NUM_ZERO;

    while let Some(call_offset) = source[search_idx..].find(prefix) {
        let call_idx = search_idx + call_offset;
        let after_call = &source[call_idx + prefix.len()..];
        let paren_open = after_call.find('(').unwrap_or(NUM_ZERO);
        let callee_name = after_call[..paren_open].trim().trim_start_matches('*');
        if !callee_name.is_empty()
            && callee_name != "haetae_fuzz_entry"
            && !callees.contains(&callee_name.to_string())
        {
            callees.push(callee_name.to_string());
        }
        search_idx = call_idx + prefix.len() + NUM_ONE;
    }

    let (chosen_fn, p_val) = if callees.len() > NUM_ONE {
        let count = callees.len();
        let sel = (payload.unsigned_abs() as usize) % count;
        let p = (payload.unsigned_abs() as usize / count) as i64;
        (callees[sel].clone(), p)
    } else if callees.len() == NUM_ONE {
        (callees[NUM_ZERO].clone(), payload)
    } else {
        (String::new(), payload)
    };

    let (func_params, target_lines) = if !chosen_fn.is_empty() {
        find_function_params_and_body(source, &chosen_fn)
            .or_else(|| find_function_params_and_body(source, "haetae_fuzz_entry"))
            .unwrap_or_default()
    } else {
        find_function_params_and_body(source, "haetae_fuzz_entry").unwrap_or_default()
    };

    let mut env = HashMap::new();
    env.insert("payload".to_string(), p_val);
    env.insert("p".to_string(), p_val);
    env.insert("p0".to_string(), p_val);
    env.insert("p1".to_string(), p_val & NUM_ONE as i64);
    env.insert("p2".to_string(), (p_val >> NUM_ONE) & NUM_ONE as i64);
    env.insert("base_val".to_string(), p_val);
    env.insert("factor".to_string(), p_val & NUM_ONE as i64);
    env.insert("a".to_string(), p_val);
    env.insert("b".to_string(), p_val & NUM_ONE as i64);
    env.insert("cond".to_string(), p_val);

    for (p_idx, p_name) in func_params.iter().enumerate() {
        if p_idx == NUM_ZERO {
            env.insert(p_name.clone(), p_val);
        } else {
            let shift = p_idx - NUM_ONE;
            let v = (p_val >> shift) & NUM_ONE as i64;
            env.insert(p_name.clone(), v);
        }
    }

    let lines_to_eval = if target_lines.is_empty() {
        source
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        target_lines
    };

    execute_function_lines(&lines_to_eval, &mut env, Some(source), NUM_ZERO)
}
