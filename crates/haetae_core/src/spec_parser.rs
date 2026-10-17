use crate::primitives::Primitive;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContractSpec {
    pub function_name: String,
    pub return_type: String,
    pub param_names: Vec<String>,
    pub param_types: Vec<String>,
    pub invariants: Vec<String>,
    pub pre_conditions: Vec<String>,
    pub post_conditions: Vec<String>,
    pub source_primitives: Vec<Primitive>,
    pub target_primitives: Vec<Primitive>,
}

impl Default for ContractSpec {
    fn default() -> Self {
        Self {
            function_name: "haetae_compute".to_string(),
            return_type: "int".to_string(),
            param_names: vec!["val".to_string()],
            param_types: vec!["int".to_string()],
            invariants: vec!["val >= 0".to_string()],
            pre_conditions: Vec::new(),
            post_conditions: Vec::new(),
            source_primitives: vec![Primitive::IfElse],
            target_primitives: vec![Primitive::BitwiseOp],
        }
    }
}

pub fn parse_markdown_spec(content: &str) -> Vec<ContractSpec> {
    let mut specs = Vec::new();
    let mut current_spec: Option<ContractSpec> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") || trimmed.starts_with("## ") || trimmed.starts_with("### ") {
            if let Some(spec) = current_spec.take() {
                if !spec.function_name.is_empty() {
                    specs.push(spec);
                }
            }

            let heading_text = trimmed.trim_start_matches('#').trim();
            if heading_text.contains('(') && heading_text.contains(')') {
                let mut spec = ContractSpec::default();
                spec.invariants.clear();
                spec.pre_conditions.clear();
                spec.post_conditions.clear();
                spec.source_primitives.clear();
                spec.target_primitives.clear();
                if let Some((name, ret, p_names, p_types)) = parse_function_signature(heading_text)
                {
                    spec.function_name = name;
                    spec.return_type = ret;
                    spec.param_names = p_names;
                    spec.param_types = p_types;
                } else if let Some(name) = extract_func_name_from_heading(heading_text) {
                    spec.function_name = name;
                }
                current_spec = Some(spec);
            } else {
                let lower_h = heading_text.to_lowercase();
                if lower_h.starts_with("specification:") || lower_h.starts_with("contract:") {
                    let mut spec = ContractSpec::default();
                    spec.invariants.clear();
                    spec.pre_conditions.clear();
                    spec.post_conditions.clear();
                    spec.param_names.clear();
                    spec.param_types.clear();
                    spec.source_primitives.clear();
                    spec.target_primitives.clear();

                    let raw_name = heading_text
                        .split_once(':')
                        .map(|(_, b)| b.trim())
                        .unwrap_or_default();
                    let clean_name = raw_name
                        .to_lowercase()
                        .replace("contract", "")
                        .replace("specification", "")
                        .trim()
                        .replace(' ', "_");
                    if !clean_name.is_empty() {
                        spec.function_name = clean_name;
                    }
                    current_spec = Some(spec);
                }
            }
            continue;
        }

        if trimmed.starts_with("fn ") || trimmed.starts_with("int ") || trimmed.starts_with("def ")
        {
            let mut spec = ContractSpec::default();
            spec.invariants.clear();
            spec.pre_conditions.clear();
            spec.post_conditions.clear();
            spec.source_primitives.clear();
            spec.target_primitives.clear();
            if let Some((name, ret, p_names, p_types)) = parse_function_signature(trimmed) {
                spec.function_name = name;
                spec.return_type = ret;
                spec.param_names = p_names;
                spec.param_types = p_types;
            }
            current_spec = Some(spec);
            continue;
        }

        if let Some(ref mut spec) = current_spec {
            let lower = trimmed.to_lowercase();
            if lower.starts_with("- invariant")
                || lower.starts_with("* invariant")
                || lower.starts_with("- contract")
                || lower.starts_with("* contract")
            {
                if let Some((_, text)) = trimmed.split_once(':') {
                    let invariant_text = text.trim().to_string();
                    if !invariant_text.is_empty() {
                        spec.invariants.push(invariant_text);
                    }
                }
            } else if lower.starts_with("- precondition")
                || lower.starts_with("* precondition")
                || lower.starts_with("- pre-condition")
                || lower.starts_with("* pre-condition")
                || lower.starts_with("- requires")
                || lower.starts_with("* requires")
            {
                if let Some((_, text)) = trimmed.split_once(':') {
                    let pre_text = text.trim().to_string();
                    if !pre_text.is_empty() {
                        spec.pre_conditions.push(pre_text);
                    }
                }
            } else if lower.starts_with("- postcondition")
                || lower.starts_with("* postcondition")
                || lower.starts_with("- post-condition")
                || lower.starts_with("* post-condition")
                || lower.starts_with("- ensures")
                || lower.starts_with("* ensures")
            {
                if let Some((_, text)) = trimmed.split_once(':') {
                    let post_text = text.trim().to_string();
                    if !post_text.is_empty() {
                        spec.post_conditions.push(post_text);
                    }
                }
            } else if lower.starts_with("- parameter")
                || lower.starts_with("* parameter")
                || lower.starts_with("- param")
                || lower.starts_with("* param")
            {
                if let Some((_, text)) = trimmed.split_once(':') {
                    let ptext = text.trim();
                    if ptext.contains('(') && ptext.contains(')') {
                        if let Some(open) = ptext.find('(') {
                            if let Some(close) = ptext[open..].find(')') {
                                let name = ptext[..open].trim().to_string();
                                let ptype = ptext[open + 1..open + close].trim().to_string();
                                if !name.is_empty() {
                                    spec.param_names.push(name);
                                    spec.param_types.push(if ptype.is_empty() {
                                        "int".to_string()
                                    } else {
                                        ptype
                                    });
                                }
                            }
                        }
                    } else {
                        let mut parts = ptext.split_whitespace();
                        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                            spec.param_types.push(first.to_string());
                            spec.param_names.push(second.to_string());
                        } else if let Some(only) = ptext.split_whitespace().next() {
                            spec.param_names.push(only.to_string());
                            spec.param_types.push("int".to_string());
                        }
                    }
                }
            } else if lower.starts_with("- returns")
                || lower.starts_with("* returns")
                || lower.starts_with("- return")
                || lower.starts_with("* return")
            {
                if let Some((_, text)) = trimmed.split_once(':') {
                    let ret = text.trim();
                    if !ret.is_empty() {
                        spec.return_type = ret.to_string();
                    }
                }
            }

            for word in trimmed.split_whitespace() {
                if word.starts_with('#') {
                    let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    if let Some(prim) = Primitive::from_tag(&format!("#{}", cleaned)) {
                        if !spec.source_primitives.contains(&prim) {
                            spec.source_primitives.push(prim);
                        }
                    }
                }
            }
        }
    }

    if let Some(mut spec) = current_spec {
        if spec.param_names.is_empty() {
            spec.param_names.push("val".to_string());
            spec.param_types.push("int".to_string());
        }
        if !spec.function_name.is_empty() {
            specs.push(spec);
        }
    }

    if specs.is_empty() {
        specs.push(ContractSpec::default());
    }

    specs
}

fn extract_func_name_from_heading(heading: &str) -> Option<String> {
    let before_paren = heading.split('(').next()?.trim();
    let name = before_paren.split_whitespace().last()?.trim();
    if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Some(name.to_string())
    } else {
        None
    }
}

fn parse_function_signature(sig: &str) -> Option<(String, String, Vec<String>, Vec<String>)> {
    let open_idx = sig.find('(')?;
    let close_idx = sig.find(')')?;
    if close_idx < open_idx {
        return None;
    }

    let prefix = &sig[0..open_idx].trim();
    let func_name = prefix.split_whitespace().last()?.to_string();

    let params_str = &sig[open_idx + 1..close_idx].trim();
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();

    if !params_str.is_empty() {
        for part in params_str.split(',') {
            let part_trim = part.trim();
            if part_trim.contains(':') {
                let mut pieces = part_trim.split(':');
                let name = pieces.next()?.trim().to_string();
                let ptype = pieces.next()?.trim().to_string();
                param_names.push(name);
                param_types.push(ptype);
            } else {
                let mut pieces = part_trim.split_whitespace();
                if let (Some(first), Some(second)) = (pieces.next(), pieces.next()) {
                    param_types.push(first.to_string());
                    param_names.push(second.to_string());
                } else if let Some(only) = part_trim.split_whitespace().next() {
                    param_names.push(only.to_string());
                    param_types.push("int".to_string());
                }
            }
        }
    }

    let mut ret_type = "int".to_string();
    let suffix = &sig[close_idx + 1..].trim();
    if let Some(arrow_idx) = suffix.find("->") {
        let after_arrow = &suffix[arrow_idx + "->".len()..].trim();
        let ret_name = after_arrow.split_whitespace().next().unwrap_or("int");
        ret_type = ret_name.trim_matches('{').trim().to_string();
    }

    Some((func_name, ret_type, param_names, param_types))
}

/// Deterministic offline C code generator from a parsed `ContractSpec`.
/// Used strictly for offline unit test suites and the `/boost` hardware pipeline benchmark.
///
/// CRITICAL MANDATE:
/// Must NOT be used as a mock or silent fallback for Ollama inference in CLI `handle_spec`.
#[allow(clippy::get_first)]
pub fn generate_c_implementation(spec: &ContractSpec) -> String {
    let mut out = String::new();
    let ret = match spec.return_type.as_str() {
        "i64" | "long" => "long long",
        "u64" | "usize" => "unsigned long long",
        "f64" | "double" => "double",
        "f32" | "float" => "float",
        _ => "int",
    };

    let mut param_decls = Vec::new();
    for (name, ptype) in spec.param_names.iter().zip(spec.param_types.iter()) {
        let ctype = match ptype.as_str() {
            "i64" | "long" => "long long",
            "u64" | "usize" => "unsigned long long",
            "f64" | "double" => "double",
            "f32" | "float" => "float",
            _ => "int",
        };
        param_decls.push(format!("{} {}", ctype, name));
    }

    out.push_str(&format!(
        "{} {}({}) {{\n",
        ret,
        spec.function_name,
        param_decls.join(", ")
    ));

    for inv in &spec.invariants {
        let inv_lower = inv.to_lowercase();
        if inv_lower.contains("!= 0")
            || inv_lower.contains("must not equal 0")
            || inv_lower.contains("not equal 0")
        {
            let mut found_var = None;
            for p in &spec.param_names {
                if inv_lower.contains(&p.to_lowercase()) {
                    found_var = Some(p.as_str());
                    break;
                }
            }
            let var = found_var.unwrap_or_else(|| inv.split("!=").next().unwrap_or("val").trim());
            out.push_str(&format!("    if ({} == 0) return 0;\n", var));
        } else if inv.contains("> 0") {
            let var = inv.split('>').next().unwrap_or("val").trim();
            out.push_str(&format!("    if ({} <= 0) return 0;\n", var));
        }
    }

    if spec.param_names.is_empty() {
        out.push_str("    return 0;\n");
    } else if spec.function_name.contains("divide") {
        if let (Some(v0), Some(v1)) = (spec.param_names.get(0), spec.param_names.get(1)) {
            out.push_str(&format!(
                "    #if defined(__aarch64__) || defined(_M_ARM64)\n    haetae_aarch64_div_guard({});\n    #endif\n",
                v1
            ));
            out.push_str(&format!(
                "    return ({} == 0) ? 0 : ({} / {});\n",
                v1, v0, v1
            ));
        } else {
            let first_var = spec.param_names.first().map(|s| s.as_str()).unwrap_or("0");
            out.push_str(&format!("    return {};\n", first_var));
        }
    } else {
        let first_var = spec.param_names.first().map(|s| s.as_str()).unwrap_or("0");
        out.push_str(&format!("    return {};\n", first_var));
    }
    out.push_str("}\n");

    out
}

/// Constructs the prompt for real Ollama LLM inference to synthesize a C implementation satisfying the specification.
pub fn build_spec_synthesis_prompt(spec: &ContractSpec) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "Generate a production-ready C function satisfying the contract specification:\n",
    );
    prompt.push_str(&format!("Function: {}\n", spec.function_name));
    prompt.push_str(&format!("Return Type: {}\n", spec.return_type));

    let mut params = Vec::new();
    for (name, ptype) in spec.param_names.iter().zip(spec.param_types.iter()) {
        params.push(format!("{}: {}", name, ptype));
    }
    if !params.is_empty() {
        prompt.push_str(&format!("Parameters: {}\n", params.join(", ")));
    }

    if !spec.pre_conditions.is_empty() {
        prompt.push_str("Preconditions:\n");
        for pre in &spec.pre_conditions {
            prompt.push_str(&format!("- {}\n", pre));
        }
    }

    if !spec.invariants.is_empty() {
        prompt.push_str("Invariants:\n");
        for inv in &spec.invariants {
            prompt.push_str(&format!("- {}\n", inv));
        }
    }

    if !spec.post_conditions.is_empty() {
        prompt.push_str("Postconditions:\n");
        for post in &spec.post_conditions {
            prompt.push_str(&format!("- {}\n", post));
        }
    }

    prompt.push_str("Rules: Output strictly the C function implementation. Do not include markdown code fences, comments, or explanations. Satisfy all invariants with explicit safety guards.\n");

    prompt
}
