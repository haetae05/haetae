use crate::config::{
    DEFAULT_CONTEXT_WINDOW_TOKENS, ESTIMATED_CHARS_PER_TOKEN, MAX_COLLECTED_SOURCE_FILES,
    MAX_FILES_FOR_CROSS_REF, MAX_SYMBOLS_FOR_CROSS_REF, PAGERANK_CONVERGENCE_EPSILON,
    PAGERANK_DAMPING_FACTOR, PAGERANK_MAX_ITERATIONS,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SymbolDef {
    pub file_path: PathBuf,
    pub name: String,
    pub signature: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    pub symbols: Vec<SymbolDef>,
    pub symbol_indices: HashMap<String, usize>,
    pub adjacency: HashMap<usize, HashSet<usize>>,
    pub reverse_adjacency: HashMap<usize, HashSet<usize>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_symbol(&mut self, symbol: SymbolDef) -> usize {
        if let Some(&idx) = self.symbol_indices.get(&symbol.name) {
            return idx;
        }
        let idx = self.symbols.len();
        self.symbol_indices.insert(symbol.name.clone(), idx);
        self.symbols.push(symbol);
        self.adjacency.entry(idx).or_default();
        self.reverse_adjacency.entry(idx).or_default();
        idx
    }

    pub fn add_dependency(&mut self, from_idx: usize, to_idx: usize) {
        if from_idx < self.symbols.len() && to_idx < self.symbols.len() {
            self.adjacency.entry(from_idx).or_default().insert(to_idx);
            self.reverse_adjacency
                .entry(to_idx)
                .or_default()
                .insert(from_idx);
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.symbols)
    }

    pub fn compute_personalized_pagerank(&self, seed_indices: &[usize]) -> Vec<f64> {
        let n = self.symbols.len();
        if n == 0 {
            return Vec::new();
        }

        let mut p = vec![0.0; n];
        let mut seed_dist = vec![0.0; n];

        if seed_indices.is_empty() {
            let uniform = 1.0 / (n as f64);
            for idx in 0..n {
                p[idx] = uniform;
                seed_dist[idx] = uniform;
            }
        } else {
            let seed_val = 1.0 / (seed_indices.len() as f64);
            for &s in seed_indices {
                if s < n {
                    seed_dist[s] = seed_val;
                    p[s] = seed_val;
                }
            }
        }

        for _ in 0..PAGERANK_MAX_ITERATIONS {
            let mut next_p = vec![0.0; n];
            let mut dangling_sum = 0.0;

            for (i, &p_val) in p.iter().enumerate() {
                let out_degree = self.adjacency.get(&i).map_or(0, |s| s.len());
                if out_degree == 0 {
                    dangling_sum += p_val;
                } else {
                    let share = (p_val * PAGERANK_DAMPING_FACTOR) / (out_degree as f64);
                    if let Some(neighbors) = self.adjacency.get(&i) {
                        for &v in neighbors {
                            next_p[v] += share;
                        }
                    }
                }
            }

            let mut diff = 0.0;
            for (i, next_p_val) in next_p.iter_mut().enumerate() {
                *next_p_val += (1.0 - PAGERANK_DAMPING_FACTOR) * seed_dist[i]
                    + (PAGERANK_DAMPING_FACTOR * dangling_sum * seed_dist[i]);
                diff += (*next_p_val - p[i]).abs();
            }

            p = next_p;
            if diff < PAGERANK_CONVERGENCE_EPSILON {
                break;
            }
        }

        p
    }
}

#[derive(Debug, Clone, Default)]
pub struct MtimeCache {
    pub file_mtimes: HashMap<PathBuf, u64>,
    pub file_symbols: HashMap<PathBuf, Vec<SymbolDef>>,
}

impl MtimeCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn should_reindex(&self, path: &Path, current_mtime: u64) -> bool {
        match self.file_mtimes.get(path) {
            Some(&cached_mtime) => cached_mtime != current_mtime,
            None => true,
        }
    }

    pub fn record_mtime(&mut self, path: PathBuf, mtime: u64) {
        self.file_mtimes.insert(path, mtime);
    }

    pub fn get_cached_symbols(&self, path: &Path, current_mtime: u64) -> Option<&Vec<SymbolDef>> {
        if !self.should_reindex(path, current_mtime) {
            self.file_symbols.get(path)
        } else {
            None
        }
    }

    pub fn store_symbols(&mut self, path: PathBuf, mtime: u64, symbols: Vec<SymbolDef>) {
        self.file_mtimes.insert(path.clone(), mtime);
        self.file_symbols.insert(path, symbols);
    }
}

pub struct RepoMapCompressor {
    pub max_tokens: usize,
}

impl Default for RepoMapCompressor {
    fn default() -> Self {
        Self {
            max_tokens: DEFAULT_CONTEXT_WINDOW_TOKENS,
        }
    }
}

impl RepoMapCompressor {
    pub fn new(max_tokens: usize) -> Self {
        Self { max_tokens }
    }

    pub fn compress_signatures(&self, graph: &DependencyGraph, ranks: &[f64]) -> String {
        let mut indexed_ranks: Vec<(usize, f64)> = ranks.iter().copied().enumerate().collect();
        indexed_ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut total_chars = 0;
        let char_budget = self.max_tokens * ESTIMATED_CHARS_PER_TOKEN;
        let mut selected_symbols = Vec::new();

        for (idx, _rank) in indexed_ranks {
            if idx >= graph.symbols.len() {
                continue;
            }
            let sym = &graph.symbols[idx];
            let line_len = sym.signature.len() + sym.file_path.to_string_lossy().len() + 1;
            if total_chars + line_len > char_budget && !selected_symbols.is_empty() {
                break;
            }
            total_chars += line_len;
            selected_symbols.push(sym);
        }

        let mut lines = Vec::new();
        for sym in selected_symbols {
            lines.push(format!(
                "{} | {}",
                sym.file_path.to_string_lossy(),
                sym.signature
            ));
        }

        lines.join("\n")
    }

    pub fn build_and_compress_from_dir(&self, dir: &Path, filter_query: Option<&str>) -> String {
        let mut cache = MtimeCache::new();
        self.build_and_compress_with_cache(dir, filter_query, &mut cache)
    }

    pub fn build_and_compress_with_cache(
        &self,
        dir: &Path,
        filter_query: Option<&str>,
        cache: &mut MtimeCache,
    ) -> String {
        let mut graph = DependencyGraph::new();
        let mut file_paths = Vec::new();
        Self::collect_source_files(dir, &mut file_paths);

        let mut file_contents: Vec<(PathBuf, String, Vec<usize>)> = Vec::new();

        for path in file_paths {
            let mtime = std::fs::metadata(&path)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            if let Some(cached_syms) = cache.get_cached_symbols(&path, mtime) {
                let content = std::fs::read_to_string(&path).unwrap_or_default();
                let mut sym_indices = Vec::new();
                for sym in cached_syms.clone() {
                    let idx = graph.add_symbol(sym);
                    sym_indices.push(idx);
                }
                file_contents.push((path, content, sym_indices));
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(&path) {
                let mut sym_indices = Vec::new();
                let mut parsed_syms = Vec::new();
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("pub fn ")
                        || trimmed.starts_with("fn ")
                        || trimmed.starts_with("def ")
                        || trimmed.starts_with("func ")
                        || trimmed.starts_with("int ")
                        || trimmed.starts_with("void ")
                        || trimmed.starts_with("export function ")
                        || trimmed.starts_with("struct ")
                        || trimmed.starts_with("pub struct ")
                        || trimmed.starts_with("class ")
                        || trimmed.starts_with("export class ")
                        || trimmed.starts_with("trait ")
                        || trimmed.starts_with("interface ")
                        || trimmed.starts_with("enum ")
                        || trimmed.starts_with("type ")
                    {
                        let sym_name = trimmed
                            .split('(')
                            .next()
                            .unwrap_or(trimmed)
                            .split('{')
                            .next()
                            .unwrap_or(trimmed)
                            .split(':')
                            .next()
                            .unwrap_or(trimmed)
                            .split_whitespace()
                            .last()
                            .unwrap_or(trimmed)
                            .trim_start_matches('*');

                        if !sym_name.is_empty() && sym_name.len() > 1 {
                            let sym = SymbolDef {
                                file_path: path.clone(),
                                name: sym_name.to_string(),
                                signature: trimmed.to_string(),
                                line_number: line_idx + 1,
                            };
                            let idx = graph.add_symbol(sym.clone());
                            sym_indices.push(idx);
                            parsed_syms.push(sym);
                        }
                    }
                }
                cache.store_symbols(path.clone(), mtime, parsed_syms);
                file_contents.push((path, content, sym_indices));
            }
        }

        let mut edges_to_add = Vec::new();
        if file_contents.len() <= MAX_FILES_FOR_CROSS_REF
            && graph.symbols.len() <= MAX_SYMBOLS_FOR_CROSS_REF
        {
            for (from_path, content, caller_indices) in &file_contents {
                for (target_idx, target_sym) in graph.symbols.iter().enumerate() {
                    if target_sym.file_path != *from_path && content.contains(&target_sym.name) {
                        for &c_idx in caller_indices {
                            edges_to_add.push((c_idx, target_idx));
                        }
                    }
                }
            }
        }
        for (from_idx, to_idx) in edges_to_add {
            graph.add_dependency(from_idx, to_idx);
        }

        let seed_indices: Vec<usize> = if let Some(q) = filter_query {
            graph
                .symbols
                .iter()
                .enumerate()
                .filter(|(_, s)| s.name.contains(q) || s.signature.contains(q))
                .map(|(i, _)| i)
                .collect()
        } else {
            Vec::new()
        };

        let ranks = graph.compute_personalized_pagerank(&seed_indices);
        self.compress_signatures(&graph, &ranks)
    }

    fn collect_source_files(dir: &Path, out: &mut Vec<PathBuf>) {
        if out.len() >= MAX_COLLECTED_SOURCE_FILES {
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            if out.len() >= MAX_COLLECTED_SOURCE_FILES {
                break;
            }
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name.starts_with('.')
                    || name == "target"
                    || name == "node_modules"
                    || name == "dist"
                    || name == "build"
                {
                    continue;
                }
                Self::collect_source_files(&path, out);
            } else if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                if matches!(
                    ext,
                    "rs" | "c"
                        | "cpp"
                        | "cc"
                        | "h"
                        | "hpp"
                        | "go"
                        | "py"
                        | "ts"
                        | "js"
                        | "java"
                        | "cs"
                        | "swift"
                        | "kt"
                        | "php"
                ) {
                    out.push(path);
                }
            }
        }
    }
}
