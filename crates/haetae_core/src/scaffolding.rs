use crate::primitives::Primitive;

pub const FIM_PREFIX_TOKEN: &str = "<|fim_prefix|>";
pub const FIM_SUFFIX_TOKEN: &str = "<|fim_suffix|>";
pub const FIM_MIDDLE_TOKEN: &str = "<|fim_middle|>";

pub const SCAFFOLDING_HEADER: &str = "[AST Scaffolding Context]";
pub const SOURCE_PRIMITIVES_LABEL: &str = "Source Primitives: ";
pub const TARGET_PRIMITIVES_LABEL: &str = "Target Primitive Transformation: ";
pub const CONTRACT_CONSTRAINTS_LABEL: &str = "Contract Constraints: ";
pub const RESEARCH_HEADER: &str = "[Web Research Context]";
pub const REPO_MAP_HEADER: &str = "[Repo Map Context]";

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Failed to parse FIM prompt: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FimPrompt {
    pub prefix: String,
    pub suffix: String,
    pub source_primitives: Vec<Primitive>,
    pub target_primitives: Vec<Primitive>,
    pub contract_constraints: Vec<String>,
    pub research_context: Option<String>,
    pub repo_map_context: Option<String>,
}

impl FimPrompt {
    pub fn new(prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            suffix: suffix.into(),
            source_primitives: Vec::new(),
            target_primitives: Vec::new(),
            contract_constraints: Vec::new(),
            research_context: None,
            repo_map_context: None,
        }
    }

    pub fn with_source_primitive(mut self, p: Primitive) -> Self {
        self.source_primitives.push(p);
        self
    }

    pub fn with_target_primitive(mut self, p: Primitive) -> Self {
        self.target_primitives.push(p);
        self
    }

    pub fn with_contract_constraint(mut self, c: impl Into<String>) -> Self {
        self.contract_constraints.push(c.into());
        self
    }

    pub fn with_research_context(mut self, r: impl Into<String>) -> Self {
        self.research_context = Some(r.into());
        self
    }

    pub fn with_repo_map_context(mut self, rm: impl Into<String>) -> Self {
        self.repo_map_context = Some(rm.into());
        self
    }

    pub fn assemble(&self) -> String {
        let mut out = String::new();
        out.push_str(FIM_PREFIX_TOKEN);
        out.push('\n');

        let has_scaffolding = !self.source_primitives.is_empty()
            || !self.target_primitives.is_empty()
            || !self.contract_constraints.is_empty();

        if has_scaffolding {
            out.push_str(SCAFFOLDING_HEADER);
            out.push('\n');

            if !self.source_primitives.is_empty() {
                out.push_str(SOURCE_PRIMITIVES_LABEL);
                let tags: Vec<&'static str> =
                    self.source_primitives.iter().map(|p| p.tag()).collect();
                out.push_str(&tags.join(", "));
                out.push('\n');
            }

            if !self.target_primitives.is_empty() {
                out.push_str(TARGET_PRIMITIVES_LABEL);
                let tags: Vec<&'static str> =
                    self.target_primitives.iter().map(|p| p.tag()).collect();
                out.push_str(&tags.join(", "));
                out.push('\n');
            }

            if !self.contract_constraints.is_empty() {
                out.push_str(CONTRACT_CONSTRAINTS_LABEL);
                out.push_str(&self.contract_constraints.join("; "));
                out.push('\n');
            }
        }

        if let Some(ref res) = self.research_context {
            out.push_str(RESEARCH_HEADER);
            out.push('\n');
            out.push_str(res);
            out.push('\n');
        }

        if let Some(ref rm) = self.repo_map_context {
            out.push_str(REPO_MAP_HEADER);
            out.push('\n');
            out.push_str(rm);
            out.push('\n');
        }

        out.push_str(&self.prefix);
        out.push('\n');
        out.push_str(FIM_SUFFIX_TOKEN);
        out.push('\n');
        out.push_str(&self.suffix);
        out.push('\n');
        out.push_str(FIM_MIDDLE_TOKEN);
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedScaffolding {
    pub source_primitives: Vec<Primitive>,
    pub target_primitives: Vec<Primitive>,
    pub contract_constraints: Vec<String>,
    pub prefix: String,
    pub suffix: String,
}

impl ParsedScaffolding {
    pub fn parse(assembled: &str) -> Option<Self> {
        if !assembled.starts_with(FIM_PREFIX_TOKEN) {
            return None;
        }

        let prefix_end = assembled.find(FIM_SUFFIX_TOKEN)?;
        let suffix_start = prefix_end + FIM_SUFFIX_TOKEN.len();
        let suffix_end = assembled.find(FIM_MIDDLE_TOKEN)?;

        let prefix_block = &assembled[FIM_PREFIX_TOKEN.len()..prefix_end].trim();
        let suffix_block = &assembled[suffix_start..suffix_end].trim();

        let mut source_primitives = Vec::new();
        let mut target_primitives = Vec::new();
        let mut contract_constraints = Vec::new();
        let mut remaining_prefix_lines = Vec::new();

        let mut in_scaffolding = false;
        let mut in_ignored_block = false;
        for line in prefix_block.lines() {
            let trimmed = line.trim();
            if trimmed == SCAFFOLDING_HEADER {
                in_scaffolding = true;
                in_ignored_block = false;
                continue;
            }
            if trimmed == RESEARCH_HEADER || trimmed == REPO_MAP_HEADER {
                in_scaffolding = false;
                in_ignored_block = true;
                continue;
            }

            if in_scaffolding {
                if let Some(rest) = trimmed.strip_prefix(SOURCE_PRIMITIVES_LABEL) {
                    for tag in rest.split(',') {
                        if let Some(prim) = Primitive::from_tag(tag.trim()) {
                            source_primitives.push(prim);
                        }
                    }
                    continue;
                }
                if let Some(rest) = trimmed.strip_prefix(TARGET_PRIMITIVES_LABEL) {
                    for tag in rest.split(',') {
                        if let Some(prim) = Primitive::from_tag(tag.trim()) {
                            target_primitives.push(prim);
                        }
                    }
                    continue;
                }
                if let Some(rest) = trimmed.strip_prefix(CONTRACT_CONSTRAINTS_LABEL) {
                    for c in rest.split(';') {
                        contract_constraints.push(c.trim().to_string());
                    }
                    continue;
                }
            }

            if in_ignored_block {
                if trimmed.starts_with("int ")
                    || trimmed.starts_with("fn ")
                    || trimmed.starts_with("def ")
                {
                    in_ignored_block = false;
                } else {
                    continue;
                }
            }

            remaining_prefix_lines.push(line);
        }

        Some(Self {
            source_primitives,
            target_primitives,
            contract_constraints,
            prefix: remaining_prefix_lines.join("\n").trim().to_string(),
            suffix: suffix_block.to_string(),
        })
    }
}
