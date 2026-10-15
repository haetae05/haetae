use haetae_core::config::{NUM_ONE, NUM_SEVEN, NUM_TWO, NUM_ZERO};
use std::collections::HashMap;

pub fn generate_c_fuzz_harness(func_name: &str, param_types: &[&str]) -> String {
    generate_multi_c_fuzz_harness(&[(func_name, param_types)])
}

pub fn generate_multi_c_fuzz_harness(funcs: &[(&str, &[&str])]) -> String {
    let count = funcs.len();
    if count == NUM_ZERO {
        return "int haetae_fuzz_entry(long long payload) {\n  (void)payload;\n  return 0;\n}\n"
            .to_string();
    }

    let is_multi = count > NUM_ONE;
    let mut out = String::new();
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

            if p_idx == NUM_ZERO {
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
                let shift = p_idx - NUM_ONE;
                let init_val = match c_type {
                    "float" | "double" => format!("(double)((({} >> {}) & 1))", src_var, shift),
                    "const char*" => "\"\"".to_string(),
                    "void*" => "0".to_string(),
                    _ => format!("(({} >> {}) & 1)", src_var, shift),
                };
                declarations.push(format!(
                    "{} {} = ({}){}",
                    c_type, var_name, c_type, init_val
                ));
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
    out.push_str("}\n");

    out
}

pub fn generate_fuzz_payload_assignment(param_index: usize, c_type: &str, val: i64) -> String {
    let var_name = format!("p{}", param_index);
    match c_type {
        "float" | "double" => format!("{} = (double){};", var_name, val),
        "const char*" => {
            if val == NUM_ZERO as i64 {
                format!("{} = 0;", var_name)
            } else {
                format!("{} = \"test_payload\";", var_name)
            }
        }
        "void*" => {
            if val == NUM_ZERO as i64 {
                format!("{} = 0;", var_name)
            } else {
                format!("{} = (void*)1;", var_name)
            }
        }
        _ => format!("{} = {};", var_name, val),
    }
}

pub fn rewrite_c_divisions(source: &str) -> String {
    let mut out = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.contains("haetae_checked_div") {
            out.push_str(line);
            out.push('\n');
            continue;
        }

        if !line.contains('/') {
            out.push_str(line);
            out.push('\n');
            continue;
        }

        let mut reconstructed = String::new();
        let mut remaining = line;
        while let Some(ret_idx) = remaining.find("return ") {
            let before = &remaining[..ret_idx + NUM_SEVEN];
            reconstructed.push_str(before);
            let after = &remaining[ret_idx + NUM_SEVEN..];
            let semi_idx = after.find(';').unwrap_or(after.len());
            let expr = &after[..semi_idx];
            if expr.contains('/') {
                reconstructed.push_str(&lower_statement_expr(expr, &HashMap::new()));
            } else {
                reconstructed.push_str(expr);
            }
            if semi_idx < after.len() {
                reconstructed.push(';');
                remaining = &after[semi_idx + NUM_ONE..];
            } else {
                remaining = "";
                break;
            }
        }
        reconstructed.push_str(remaining);
        out.push_str(&reconstructed);
        out.push('\n');
    }
    out
}

pub fn lower_source_to_c_ir(source: &str, lang_or_ext: &str) -> String {
    let guard_prefix = "#ifndef HAETAE_DIV_GUARD_H_\n#define HAETAE_DIV_GUARD_H_\n#include <signal.h>\nstatic void haetae_trap_sigfpe(void) {\n  raise(SIGFPE);\n}\nstatic long long haetae_checked_div(long long num, long long denom) {\n  if (denom == 0) raise(SIGFPE);\n  return num / denom;\n}\nstatic long long haetae_checked_mod(long long num, long long denom) {\n  if (denom == 0) raise(SIGFPE);\n  return num % denom;\n}\nstatic void haetae_aarch64_div_guard(long long denom) {\n  if (denom == 0) raise(SIGFPE);\n}\n#endif\n#ifndef HAETAE_BOUNDS_GUARD_H_\n#define HAETAE_BOUNDS_GUARD_H_\n#include <signal.h>\nstatic inline long long haetae_checked_index(long long len, long long idx) {\n  if (idx < 0 || idx >= len) { raise(SIGSEGV); }\n  return idx;\n}\n#endif\n";

    let lang_lower = lang_or_ext.to_lowercase();
    let has_non_c_keyword = source.starts_with("def ")
        || source.contains("\ndef ")
        || source.starts_with("fn ")
        || source.contains("\nfn ")
        || source.starts_with("pub fn ")
        || source.contains("\npub fn ")
        || source.starts_with("func ")
        || source.contains("\nfunc ")
        || source.starts_with("fun ")
        || source.contains("\nfun ")
        || source.starts_with("function ")
        || source.contains("\nfunction ");

    let is_explicit_c = (lang_lower.ends_with(".c")
        || lang_lower.ends_with(".h")
        || lang_lower == "c"
        || lang_lower == "cpp")
        && !has_non_c_keyword;

    let looks_like_c = source.contains("#include")
        || source.contains("int main(")
        || ((source.starts_with("int ")
            || source.starts_with("void ")
            || source.starts_with("long ")
            || source.starts_with("double ")
            || source.starts_with("float ")
            || source.starts_with("char* ")
            || source.starts_with("void* "))
            && !has_non_c_keyword);

    let is_c_like = is_explicit_c || looks_like_c;

    if is_c_like || source.contains("haetae_fuzz_entry") {
        if source.contains("haetae_fuzz_entry") {
            return source.to_string();
        }

        let mut discovered_c_funcs: Vec<(String, Vec<String>)> = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("haetae_") || trimmed.starts_with('#') {
                continue;
            }
            if let (Some(open_paren), Some(close_paren)) = (trimmed.find('('), trimmed.find(')')) {
                if open_paren < close_paren {
                    let before_paren = trimmed[..open_paren].trim();
                    let parts: Vec<&str> = before_paren.split_whitespace().collect();
                    if parts.len() >= NUM_TWO
                        || (parts.len() == NUM_ONE
                            && (parts[NUM_ZERO].starts_with("int*")
                                || parts[NUM_ZERO].starts_with("char*")
                                || parts[NUM_ZERO].starts_with("void*")))
                    {
                        let fn_name = parts.last().unwrap_or(&"").trim_start_matches('*');
                        let is_valid_name = !fn_name.is_empty()
                            && fn_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                            && !fn_name.starts_with("haetae_")
                            && fn_name != "if"
                            && fn_name != "while"
                            && fn_name != "for"
                            && fn_name != "switch"
                            && fn_name != "return";

                        let after_close = trimmed[close_paren + NUM_ONE..].trim();
                        let is_prototype = after_close.starts_with(';');

                        if is_valid_name && !is_prototype {
                            let params_str = &trimmed[open_paren + NUM_ONE..close_paren];
                            let mut param_types: Vec<String> = Vec::new();
                            for p in params_str.split(',') {
                                let p_trimmed = p.trim();
                                if let Some(first_word) = p_trimmed.split_whitespace().next() {
                                    if first_word != "void" && !first_word.is_empty() {
                                        param_types.push(first_word.to_string());
                                    }
                                }
                            }
                            if !discovered_c_funcs.iter().any(|(n, _)| n == fn_name) {
                                discovered_c_funcs.push((fn_name.to_string(), param_types));
                            }
                        }
                    }
                }
            }
        }

        let rewritten_src = rewrite_c_divisions(source);
        if !discovered_c_funcs.is_empty() {
            let targets: Vec<(&str, Vec<&str>)> = discovered_c_funcs
                .iter()
                .map(|(n, p)| (n.as_str(), p.iter().map(|s| s.as_str()).collect()))
                .collect();
            let slice_targets: Vec<(&str, &[&str])> =
                targets.iter().map(|(n, p)| (*n, p.as_slice())).collect();
            let harness = generate_multi_c_fuzz_harness(&slice_targets);
            return format!("{}\n{}\n{}\n", guard_prefix, rewritten_src, harness);
        }

        let fallback_harness = format!(
            "\nint haetae_fuzz_entry(long long p) {{\n  (void)p;\n  {}\n  return (int)(p & 1);\n}}\n",
            rewritten_src
        );
        return format!("{}\n{}\n", guard_prefix, fallback_harness);
    }

    let lines: Vec<&str> = source.lines().collect();
    let mut fn_starts = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let is_start = trimmed.starts_with("def ")
            || trimmed.starts_with("async def ")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("pub async fn ")
            || trimmed.starts_with("async fn ")
            || trimmed.starts_with("pub(crate) fn ")
            || trimmed.starts_with("function ")
            || trimmed.starts_with("export function ")
            || trimmed.starts_with("export async function ")
            || trimmed.starts_with("async function ")
            || trimmed.starts_with("func ")
            || trimmed.starts_with("fun ")
            || trimmed.starts_with("public ")
            || trimmed.starts_with("private ")
            || trimmed.starts_with("protected ")
            || trimmed.starts_with("static ");

        if is_start {
            fn_starts.push(idx);
        }
    }

    if fn_starts.is_empty() {
        let (c_code, p_types) = lower_single_function(
            "lowered_function",
            &["a".to_string(), "b".to_string()],
            source,
        );
        let p_refs: Vec<&str> = p_types.iter().map(|s| s.as_str()).collect();
        let harness = generate_c_fuzz_harness("lowered_function", &p_refs);
        return format!("{}\n{}\n{}\n", guard_prefix, c_code, harness);
    }

    let mut lowered_funcs = Vec::new();
    let mut fn_descriptors: Vec<(String, Vec<String>)> = Vec::new();

    for k in NUM_ZERO..fn_starts.len() {
        let start_idx = fn_starts[k];
        let end_idx = if k + NUM_ONE < fn_starts.len() {
            fn_starts[k + NUM_ONE]
        } else {
            lines.len()
        };

        let chunk = &lines[start_idx..end_idx];
        let mut combined_header = String::new();
        let mut header_lines_count = NUM_ZERO;

        for line in chunk {
            header_lines_count += NUM_ONE;
            combined_header.push(' ');
            combined_header.push_str(line.trim());
            if combined_header.contains(')') {
                break;
            }
        }

        let mut fn_name = format!("function_{}", k);
        let mut param_names = Vec::new();

        if let Some(open_paren) = combined_header.find('(') {
            let pre = combined_header[..open_paren].trim();
            if let Some(last_word) = pre.split_whitespace().last() {
                let cleaned = last_word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if !cleaned.is_empty() {
                    fn_name = cleaned.to_string();
                }
            }

            if let Some(close_paren) = combined_header.find(')') {
                let p_str = &combined_header[open_paren + NUM_ONE..close_paren];
                for raw_p in p_str.split(',') {
                    let p = raw_p.trim();
                    if p.is_empty() {
                        continue;
                    }
                    let without_type = p.find(':').map(|i| &p[..i]).unwrap_or(p);
                    let without_eq = without_type
                        .find('=')
                        .map(|i| &without_type[..i])
                        .unwrap_or(without_type);
                    let p_tokens: Vec<&str> = without_eq.split_whitespace().collect();
                    let candidate = if p_tokens.len() >= NUM_TWO {
                        let first = p_tokens[NUM_ZERO];
                        if first == "_" || first == "mut" {
                            p_tokens[NUM_ONE]
                        } else if first == "int"
                            || first == "long"
                            || first == "float"
                            || first == "double"
                            || first == "number"
                            || first == "string"
                            || first == "bool"
                            || first == "const"
                        {
                            p_tokens.last().copied().unwrap_or("")
                        } else {
                            p_tokens[NUM_ZERO]
                        }
                    } else if !p_tokens.is_empty() {
                        p_tokens[NUM_ZERO]
                    } else {
                        ""
                    };

                    let cleaned_p = candidate
                        .trim_start_matches('$')
                        .trim_start_matches('*')
                        .trim_start_matches('&')
                        .trim_matches(|c: char| !c.is_alphanumeric() && c != '_');

                    if !cleaned_p.is_empty() && cleaned_p != "self" && cleaned_p != "cls" {
                        param_names.push(cleaned_p.to_string());
                    }
                }
            }
        }

        let chunk_full_text = chunk.join("\n");
        let mut raw_body = String::new();

        if let (Some(first_brace), Some(last_brace)) =
            (chunk_full_text.find('{'), chunk_full_text.rfind('}'))
        {
            if first_brace < last_brace {
                raw_body = chunk_full_text[first_brace + NUM_ONE..last_brace].to_string();
            }
        }

        if raw_body.is_empty() {
            if let Some(close_paren) = combined_header.find(')') {
                if let Some(colon_offset) = combined_header[close_paren..].find(':') {
                    let colon_pos = close_paren + colon_offset;
                    let inline_after = combined_header[colon_pos + NUM_ONE..].trim();
                    let remaining = if header_lines_count < chunk.len() {
                        chunk[header_lines_count..].join("\n")
                    } else {
                        String::new()
                    };
                    if !inline_after.is_empty() {
                        raw_body = if !remaining.is_empty() {
                            format!("{}\n{}", inline_after, remaining)
                        } else {
                            inline_after.to_string()
                        };
                    } else {
                        raw_body = remaining;
                    }
                } else if header_lines_count < chunk.len() {
                    raw_body = chunk[header_lines_count..].join("\n");
                }
            } else if header_lines_count < chunk.len() {
                raw_body = chunk[header_lines_count..].join("\n");
            }
        }

        let (c_code, p_types) = lower_single_function(&fn_name, &param_names, &raw_body);
        lowered_funcs.push(c_code);
        fn_descriptors.push((fn_name, p_types));
    }

    let all_c_code = lowered_funcs.join("\n\n");
    let targets: Vec<(&str, Vec<&str>)> = fn_descriptors
        .iter()
        .map(|(n, p)| (n.as_str(), p.iter().map(|s| s.as_str()).collect()))
        .collect();
    let slice_targets: Vec<(&str, &[&str])> =
        targets.iter().map(|(n, p)| (*n, p.as_slice())).collect();
    let harness = generate_multi_c_fuzz_harness(&slice_targets);
    format!("{}\n{}\n{}\n", guard_prefix, all_c_code, harness)
}

fn lower_single_function(
    fn_name: &str,
    param_names: &[String],
    body_text: &str,
) -> (String, Vec<String>) {
    let mut resolved_params = param_names.to_vec();
    if resolved_params.is_empty() {
        resolved_params.push("a".to_string());
        resolved_params.push("b".to_string());
    }

    let mut c_func_sig = format!("int {}(", fn_name);
    let mut param_decls = Vec::new();
    let mut param_types = Vec::new();
    for p in &resolved_params {
        param_decls.push(format!("int {}", p));
        param_types.push("int".to_string());
    }
    c_func_sig.push_str(&param_decls.join(", "));
    c_func_sig.push_str(") {\n");

    let statements = split_into_statements(body_text);
    let mut lowered_body = String::new();
    let mut known_arrays: HashMap<String, String> = HashMap::new();

    for raw in &statements {
        let trimmed_raw = raw.trim();
        if trimmed_raw.starts_with('#') || trimmed_raw.starts_with("//") {
            continue;
        }

        let mut s = raw.replace("===", "==").replace("!==", "!=");
        s = s.replace("//", "/");
        s = s.replace("$", "");
        s = s.replace("Math.floor", "");
        s = s.replace(" or ", " || ");
        s = s.replace(" and ", " && ");
        let s = s.trim();

        if !s.starts_with("if") && s.contains('=') && (s.contains('[') || s.contains("vec![")) {
            let eq_pos = s.find('=').unwrap_or(NUM_ZERO);
            let left_raw = s[..eq_pos].trim();
            let right_raw = s[eq_pos + NUM_ONE..].trim().trim_end_matches(';');
            let clean_left = left_raw
                .trim_start_matches("let mut ")
                .trim_start_matches("let ")
                .trim_start_matches("const ")
                .trim_start_matches("var ")
                .trim_start_matches("val ")
                .trim();
            let var_name = clean_left
                .split(':')
                .next()
                .unwrap_or(clean_left)
                .trim()
                .trim_matches(|c: char| !c.is_alphanumeric() && c != '_');

            if !var_name.is_empty() && right_raw.contains('[') && right_raw.contains(']') {
                let open_br = right_raw.find('[').unwrap_or(NUM_ZERO);
                let close_br = right_raw.rfind(']').unwrap_or(right_raw.len());
                if open_br < close_br {
                    let items_raw = right_raw[open_br + NUM_ONE..close_br].trim();
                    let items: Vec<&str> = if items_raw.is_empty() {
                        Vec::new()
                    } else {
                        items_raw
                            .split(',')
                            .map(|x| x.trim())
                            .filter(|x| !x.is_empty())
                            .collect()
                    };
                    let len_val = items.len();
                    let len_var = format!("{}_len", var_name);
                    lowered_body.push_str(&format!("  int {} = {};\n", len_var, len_val));
                    if items.is_empty() {
                        lowered_body.push_str(&format!("  int {}[1] = {{0}};\n", var_name));
                    } else {
                        lowered_body.push_str(&format!(
                            "  int {}[] = {{{}}};\n",
                            var_name,
                            items.join(", ")
                        ));
                    }
                    known_arrays.insert(var_name.to_string(), len_var);
                    continue;
                }
            }
        }

        if s.starts_with("if") {
            if s.contains(':') {
                let colon_idx = s.find(':').unwrap_or(s.len());
                let cond = s[NUM_ZERO..colon_idx].trim_start_matches("if").trim();
                let body_part = s[colon_idx + NUM_ONE..].trim();
                let lowered_sub =
                    lower_statement_expr(body_part.trim_end_matches(';'), &known_arrays);
                lowered_body.push_str(&format!("  if ({}) {{ {}; }}\n", cond, lowered_sub));
                continue;
            }

            if s.contains('{') && s.contains('}') {
                let open_b = s.find('{').unwrap_or(s.len());
                let close_b = s.rfind('}').unwrap_or(s.len());
                let cond_part = s[NUM_ZERO..open_b].trim_start_matches("if").trim();
                let cond = cond_part.trim_matches(|c| c == '(' || c == ')');
                let inside_b = s[open_b + NUM_ONE..close_b].trim();
                let lowered_sub =
                    lower_statement_expr(inside_b.trim_end_matches(';'), &known_arrays);
                lowered_body.push_str(&format!("  if ({}) {{ {}; }}\n", cond, lowered_sub));
                continue;
            }

            if s.contains("return ") {
                let ret_idx = s.find("return ").unwrap_or(s.len());
                let cond_part = s[NUM_ZERO..ret_idx].trim_start_matches("if").trim();
                let cond = cond_part.trim_matches(|c| c == '(' || c == ')');
                let ret_part = s[ret_idx..].trim().trim_end_matches(';');
                let lowered_ret = lower_statement_expr(ret_part, &known_arrays);
                lowered_body.push_str(&format!("  if ({}) {{ {}; }}\n", cond, lowered_ret));
                continue;
            }
        }

        if s.starts_with("return ") {
            let ret_expr = s.trim_start_matches("return ").trim().trim_end_matches(';');
            let lowered_expr = lower_statement_expr(ret_expr, &known_arrays);
            lowered_body.push_str(&format!("  return {};\n", lowered_expr));
            continue;
        }

        if s.starts_with("const ") || s.starts_with("let ") || s.starts_with("var ") {
            let after_kw = s
                .split_whitespace()
                .skip(NUM_ONE)
                .collect::<Vec<&str>>()
                .join(" ");
            let lowered = lower_statement_expr(after_kw.trim_end_matches(';'), &known_arrays);
            lowered_body.push_str(&format!("  int {};\n", lowered));
            continue;
        }

        if !s.contains('=') && !s.starts_with("if") {
            let lowered_expr = lower_statement_expr(s.trim_end_matches(';'), &known_arrays);
            if !lowered_expr.is_empty() {
                lowered_body.push_str(&format!("  return {};\n", lowered_expr));
                continue;
            }
        }

        let lowered = lower_statement_expr(s.trim_end_matches(';'), &known_arrays);
        if !lowered.is_empty() {
            lowered_body.push_str(&format!("  {};\n", lowered));
        }
    }

    if lowered_body.is_empty() {
        let p0 = &resolved_params[NUM_ZERO];
        let p1 = if resolved_params.len() > NUM_ONE {
            &resolved_params[NUM_ONE]
        } else {
            p0
        };
        lowered_body.push_str(&format!("  return {} + {};\n", p0, p1));
    }

    (format!("{}{}}}\n", c_func_sig, lowered_body), param_types)
}

fn split_into_statements(body: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut brace_depth = NUM_ZERO;

    for ch in body.chars() {
        if ch == '{' {
            brace_depth += NUM_ONE;
            current.push(ch);
        } else if ch == '}' {
            if brace_depth > NUM_ZERO {
                brace_depth -= NUM_ONE;
            }
            current.push(ch);
            if brace_depth == NUM_ZERO {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    statements.push(trimmed.to_string());
                }
                current.clear();
            }
        } else if ch == ';' && brace_depth == NUM_ZERO {
            current.push(ch);
            let trimmed = current.trim();
            if !trimmed.is_empty() {
                statements.push(trimmed.to_string());
            }
            current.clear();
        } else if ch == '\n' && brace_depth == NUM_ZERO {
            let trimmed = current.trim();
            if trimmed.ends_with(':') {
                current.push(ch);
            } else if !trimmed.is_empty() {
                statements.push(trimmed.to_string());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        statements.push(trimmed.to_string());
    }
    statements
}

fn lower_statement_expr(input: &str, known_arrays: &HashMap<String, String>) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with("return ") {
        let ret_expr = trimmed.trim_start_matches("return ").trim();
        return format!("return {}", lower_statement_expr(ret_expr, known_arrays));
    }
    let mut s = trimmed.to_string();

    for (arr_name, len_name) in known_arrays {
        let target_prefix = format!("{}[", arr_name);
        while let Some(open_idx) = s.find(&target_prefix) {
            let bracket_start = open_idx + target_prefix.len();
            if let Some(close_offset) = s[bracket_start..].find(']') {
                let bracket_end = bracket_start + close_offset;
                let inner_expr = &s[bracket_start..bracket_end];
                if inner_expr.contains("haetae_checked_index") {
                    break;
                }
                let rewritten = format!(
                    "{}[haetae_checked_index({}, {})]",
                    arr_name, len_name, inner_expr
                );
                s = format!(
                    "{}{}{}",
                    &s[..open_idx],
                    rewritten,
                    &s[bracket_end + NUM_ONE..]
                );
            } else {
                break;
            }
        }
    }

    if s.contains('/') && !s.contains("haetae_checked_div") {
        let parts: Vec<&str> = s.splitn(NUM_TWO, '/').collect();
        if parts.len() == NUM_TWO {
            let left = parts[NUM_ZERO].trim();
            let right = parts[NUM_ONE].trim().trim_end_matches(';');
            s = format!("haetae_checked_div({}, {})", left, right);
        }
    }
    s
}
