pub mod config;
pub mod primitives;
pub mod repo_map;
pub mod scaffolding;
pub mod spec_parser;
pub mod syntax_matrix;
pub mod web_research;

pub use config::{Config, TargetIsa, DEFAULT_CONTEXT_WINDOW_TOKENS, DEFAULT_FUZZ_ITERATIONS};
pub use primitives::{MetaCategory, Primitive, TargetLanguage};
pub use repo_map::{DependencyGraph, MtimeCache, RepoMapCompressor, SymbolDef};
pub use scaffolding::{CoreError, FimPrompt, ParsedScaffolding};
pub use spec_parser::{
    build_spec_synthesis_prompt, generate_c_implementation, parse_markdown_spec, ContractSpec,
};
pub use web_research::{extract_signatures, WebResearchClient, WebResearchError};
