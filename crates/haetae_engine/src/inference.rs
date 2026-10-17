use crate::model_loader::ModelError;
use crate::ollama::OllamaDaemonManager;
use haetae_core::config::{
    Config, HTTP_CONNECT_TIMEOUT_SECS, HTTP_OK, INFERENCE_TIMEOUT_SECS, MODEL_PULL_TIMEOUT_SECS,
    OLLAMA_GENERATE_ENDPOINT, OLLAMA_NUM_PREDICT, OLLAMA_PULL_ENDPOINT, OLLAMA_TAGS_ENDPOINT,
    OLLAMA_TEMPERATURE_ZERO,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[derive(serde::Serialize)]
struct OllamaOptions {
    temperature: f64,
    num_predict: usize,
    stop: Vec<String>,
}

#[derive(serde::Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    raw: bool,
    stream: bool,
    options: OllamaOptions,
}

#[derive(serde::Deserialize)]
struct OllamaResponse {
    response: Option<String>,
    error: Option<String>,
}

#[derive(serde::Serialize)]
struct OllamaPullRequest<'a> {
    model: &'a str,
    stream: bool,
}

#[derive(serde::Deserialize)]
struct OllamaModelTag {
    name: Option<String>,
    model: Option<String>,
}

#[derive(serde::Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaModelTag>>,
}

pub fn strip_reasoning_tags(input: &str) -> String {
    let mut result = input.to_string();
    let think_open = "<think>";
    let think_close = "</think>";
    while let Some(start) = result.find(think_open) {
        if let Some(end) = result.find(think_close) {
            let close_len = think_close.len();
            result.replace_range(start..end + close_len, "");
        } else {
            break;
        }
    }
    result.trim().to_string()
}

pub fn strip_markdown_code_fences(input: &str) -> String {
    let trimmed = input.trim();
    let fence = "```";
    if let Some(start_pos) = trimmed.find(fence) {
        let after_start = &trimmed[start_pos + fence.len()..];
        let first_newline = after_start.find('\n').unwrap_or(0);
        let code_start = if first_newline > 0 {
            first_newline + 1
        } else {
            0
        };
        let inner = &after_start[code_start..];
        if let Some(end_pos) = inner.find(fence) {
            return inner[..end_pos].trim().to_string();
        }
    }
    trimmed
        .lines()
        .filter(|l| !l.trim().starts_with(fence))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

pub struct InferenceClient {
    client: reqwest::Client,
    host: String,
    endpoint: String,
    model: String,
    model_verified: AtomicBool,
    daemon_manager: OllamaDaemonManager,
}

impl InferenceClient {
    pub fn new(host_or_endpoint: impl Into<String>, model: impl Into<String>) -> Self {
        let raw_target = host_or_endpoint.into();
        let (host, endpoint) = if raw_target.ends_with(OLLAMA_GENERATE_ENDPOINT) {
            let h = raw_target
                .strip_suffix(OLLAMA_GENERATE_ENDPOINT)
                .unwrap_or(&raw_target)
                .to_string();
            (h, raw_target)
        } else {
            let ep = format!("{}{}", raw_target, OLLAMA_GENERATE_ENDPOINT);
            (raw_target, ep)
        };

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(INFERENCE_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(HTTP_CONNECT_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let daemon_manager = OllamaDaemonManager::new(host.clone());

        Self {
            client,
            host,
            endpoint,
            model: model.into(),
            model_verified: AtomicBool::new(false),
            daemon_manager,
        }
    }

    pub fn from_config(cfg: &Config) -> Self {
        Self::new(&cfg.ollama_host, &cfg.ollama_model)
    }

    pub fn default_local() -> Self {
        let cfg = Config::from_env();
        Self::from_config(&cfg)
    }

    pub fn model_name(&self) -> &str {
        &self.model
    }

    pub fn daemon_manager(&self) -> &OllamaDaemonManager {
        &self.daemon_manager
    }

    pub async fn ensure_daemon_running(&self) -> Result<(), ModelError> {
        self.daemon_manager.ensure_running().await
    }

    pub async fn is_model_available(&self) -> bool {
        if !self.daemon_manager.is_alive().await {
            let _ = self.daemon_manager.ensure_running().await;
        }
        if !self.daemon_manager.is_alive().await {
            return false;
        }

        let tags_url = format!("{}{}", self.host, OLLAMA_TAGS_ENDPOINT);
        let resp = match self.client.get(&tags_url).send().await {
            Ok(r) if r.status().as_u16() == HTTP_OK => r,
            _ => return false,
        };
        let tags: OllamaTagsResponse = match resp.json().await {
            Ok(t) => t,
            _ => return false,
        };
        if let Some(models) = tags.models {
            models.iter().any(|m| {
                if let Some(ref n) = m.name {
                    if n == &self.model || n.starts_with(&self.model) || self.model.starts_with(n) {
                        return true;
                    }
                }
                if let Some(ref mdl) = m.model {
                    if mdl == &self.model
                        || mdl.starts_with(&self.model)
                        || self.model.starts_with(mdl)
                    {
                        return true;
                    }
                }
                false
            })
        } else {
            false
        }
    }

    pub async fn pull_model(&self) -> Result<(), ModelError> {
        let pull_url = format!("{}{}", self.host, OLLAMA_PULL_ENDPOINT);
        let pull_body = OllamaPullRequest {
            model: &self.model,
            stream: false,
        };
        let pull_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(MODEL_PULL_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(HTTP_CONNECT_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let resp = pull_client
            .post(&pull_url)
            .json(&pull_body)
            .send()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        let status = resp.status().as_u16();
        if status != HTTP_OK {
            return Err(ModelError::HttpStatus(status));
        }
        Ok(())
    }

    pub async fn ensure_model_available(&self) -> Result<(), ModelError> {
        if self.is_model_available().await {
            return Ok(());
        }
        if std::env::var("HAETAE_AUTO_PULL").unwrap_or_default() != "1" {
            return Err(ModelError::ModelUnavailable(self.model.clone()));
        }
        eprintln!(
            "[haetae] Auto-installing model '{}' via Ollama...",
            self.model
        );
        self.pull_model().await
    }

    pub async fn generate_raw(
        &self,
        prompt: &str,
        stop_tokens: Vec<String>,
    ) -> Result<String, ModelError> {
        if !self.daemon_manager.is_alive().await {
            self.daemon_manager.ensure_running().await?;
        }

        if !self.model_verified.load(Ordering::Relaxed) {
            self.ensure_model_available().await?;
            self.model_verified.store(true, Ordering::Relaxed);
        }

        let req_body = OllamaRequest {
            model: &self.model,
            prompt,
            raw: true,
            stream: false,
            options: OllamaOptions {
                temperature: OLLAMA_TEMPERATURE_ZERO,
                num_predict: OLLAMA_NUM_PREDICT,
                stop: stop_tokens,
            },
        };

        let resp = self
            .client
            .post(&self.endpoint)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        let status = resp.status().as_u16();
        if status != HTTP_OK {
            return Err(ModelError::HttpStatus(status));
        }

        let resp_json: OllamaResponse = resp
            .json()
            .await
            .map_err(|e| ModelError::Inference(e.to_string()))?;

        if let Some(err) = resp_json.error {
            return Err(ModelError::Inference(err));
        }

        let raw_text = resp_json.response.unwrap_or_default();
        let without_thinking = strip_reasoning_tags(&raw_text);
        let cleaned = strip_markdown_code_fences(&without_thinking);
        Ok(cleaned)
    }

    pub async fn generate_fim(&self, assembled_fim: &str) -> Result<String, ModelError> {
        let stop_tokens = vec![
            "<|fim_prefix|>".to_string(),
            "<|fim_suffix|>".to_string(),
            "<|fim_middle|>".to_string(),
            "<|endoftext|>".to_string(),
            "<|file_sep|>".to_string(),
            "<|im_end|>".to_string(),
        ];
        self.generate_raw(assembled_fim, stop_tokens).await
    }

    pub async fn generate_instruction(
        &self,
        target_lang: &str,
        instruction: &str,
    ) -> Result<String, ModelError> {
        let prompt = format!(
            "<|im_start|>system\nYou are Haetae, a contract-driven silent coding engine. Output only valid, production-ready {} code fulfilling the user instruction. Zero markdown formatting, zero comments, zero conversation.<|im_end|>\n<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            target_lang, instruction
        );
        let stop_tokens = vec![
            "<|im_start|>".to_string(),
            "<|im_end|>".to_string(),
            "<|endoftext|>".to_string(),
        ];
        self.generate_raw(&prompt, stop_tokens).await
    }

    pub async fn generate_refactor(
        &self,
        target_lang: &str,
        instruction: &str,
        code_to_refactor: &str,
        source_primitive: &str,
        target_primitive: &str,
    ) -> Result<String, ModelError> {
        let prompt = format!(
            "<|im_start|>system\nYou are Haetae, a contract-driven silent coding engine. Refactor the provided {} code according to the instruction. Transform primitives from {} to {}. Output strictly valid refactored code. Zero markdown formatting, zero comments, zero conversation.<|im_end|>\n<|im_start|>user\nInstruction: {}\nOriginal Code:\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            target_lang, source_primitive, target_primitive, instruction, code_to_refactor
        );
        let stop_tokens = vec![
            "<|im_start|>".to_string(),
            "<|im_end|>".to_string(),
            "<|endoftext|>".to_string(),
        ];
        self.generate_raw(&prompt, stop_tokens).await
    }
}

#[allow(async_fn_in_trait)]
pub trait ModelServingEngine: Send + Sync {
    fn engine_name(&self) -> &str;
    fn model_name(&self) -> &str;
    async fn is_available(&self) -> bool;
    async fn generate_raw(
        &self,
        prompt: &str,
        stop_tokens: Vec<String>,
    ) -> Result<String, ModelError>;

    async fn generate_fim(&self, assembled_fim: &str) -> Result<String, ModelError> {
        let stop_tokens = vec![
            "<|fim_prefix|>".to_string(),
            "<|fim_suffix|>".to_string(),
            "<|fim_middle|>".to_string(),
            "<|endoftext|>".to_string(),
            "<|file_sep|>".to_string(),
            "<|im_end|>".to_string(),
        ];
        self.generate_raw(assembled_fim, stop_tokens).await
    }

    async fn generate_instruction(
        &self,
        target_lang: &str,
        instruction: &str,
    ) -> Result<String, ModelError> {
        let prompt = format!(
            "<|im_start|>system\nYou are Haetae, a contract-driven silent coding engine. Output only valid, production-ready {} code fulfilling the user instruction. Zero markdown formatting, zero comments, zero conversation.<|im_end|>\n<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            target_lang, instruction
        );
        let stop_tokens = vec![
            "<|im_start|>".to_string(),
            "<|im_end|>".to_string(),
            "<|endoftext|>".to_string(),
        ];
        self.generate_raw(&prompt, stop_tokens).await
    }

    async fn generate_refactor(
        &self,
        target_lang: &str,
        instruction: &str,
        code_to_refactor: &str,
        source_primitive: &str,
        target_primitive: &str,
    ) -> Result<String, ModelError> {
        let prompt = format!(
            "<|im_start|>system\nYou are Haetae, a contract-driven silent coding engine. Refactor the provided {} code according to the instruction. Transform primitives from {} to {}. Output strictly valid refactored code. Zero markdown formatting, zero comments, zero conversation.<|im_end|>\n<|im_start|>user\nInstruction: {}\nOriginal Code:\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            target_lang, source_primitive, target_primitive, instruction, code_to_refactor
        );
        let stop_tokens = vec![
            "<|im_start|>".to_string(),
            "<|im_end|>".to_string(),
            "<|endoftext|>".to_string(),
        ];
        self.generate_raw(&prompt, stop_tokens).await
    }
}

impl ModelServingEngine for InferenceClient {
    fn engine_name(&self) -> &str {
        "ollama"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn is_available(&self) -> bool {
        self.is_model_available().await
    }

    async fn generate_raw(
        &self,
        prompt: &str,
        stop_tokens: Vec<String>,
    ) -> Result<String, ModelError> {
        self.generate_raw(prompt, stop_tokens).await
    }

    async fn generate_fim(&self, assembled_fim: &str) -> Result<String, ModelError> {
        self.generate_fim(assembled_fim).await
    }

    async fn generate_instruction(
        &self,
        target_lang: &str,
        instruction: &str,
    ) -> Result<String, ModelError> {
        self.generate_instruction(target_lang, instruction).await
    }

    async fn generate_refactor(
        &self,
        target_lang: &str,
        instruction: &str,
        code_to_refactor: &str,
        source_primitive: &str,
        target_primitive: &str,
    ) -> Result<String, ModelError> {
        self.generate_refactor(
            target_lang,
            instruction,
            code_to_refactor,
            source_primitive,
            target_primitive,
        )
        .await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LocalBackendKind {
    Ollama,
    LlamaCpp,
    Candle,
}

impl LocalBackendKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::LlamaCpp => "llama.cpp",
            Self::Candle => "candle",
        }
    }
}

pub struct LlamaCppEngine {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl LlamaCppEngine {
    pub fn new(endpoint: impl Into<String>, model: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(INFERENCE_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(HTTP_CONNECT_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            endpoint: endpoint.into(),
            model: model.into(),
            client,
        }
    }
}

impl ModelServingEngine for LlamaCppEngine {
    fn engine_name(&self) -> &str {
        "llama.cpp"
    }

    fn model_name(&self) -> &str {
        &self.model
    }

    async fn is_available(&self) -> bool {
        let health_url = format!("{}/health", self.endpoint);
        match self.client.get(&health_url).send().await {
            Ok(resp) => resp.status().as_u16() == HTTP_OK,
            Err(_) => false,
        }
    }

    async fn generate_raw(
        &self,
        prompt: &str,
        stop_tokens: Vec<String>,
    ) -> Result<String, ModelError> {
        let completion_url = format!("{}/completion", self.endpoint);
        #[derive(serde::Serialize)]
        struct LlamaCppRequest<'a> {
            prompt: &'a str,
            n_predict: usize,
            temperature: f64,
            stop: Vec<String>,
        }
        #[derive(serde::Deserialize)]
        struct LlamaCppResponse {
            content: Option<String>,
        }
        let req = LlamaCppRequest {
            prompt,
            n_predict: OLLAMA_NUM_PREDICT,
            temperature: OLLAMA_TEMPERATURE_ZERO,
            stop: stop_tokens,
        };
        let resp = self
            .client
            .post(&completion_url)
            .json(&req)
            .send()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        let status = resp.status().as_u16();
        if status != HTTP_OK {
            return Err(ModelError::HttpStatus(status));
        }

        let body: LlamaCppResponse = resp
            .json()
            .await
            .map_err(|e| ModelError::Inference(e.to_string()))?;
        let raw = body.content.unwrap_or_default();
        let stripped = strip_reasoning_tags(&raw);
        Ok(strip_markdown_code_fences(&stripped))
    }
}

pub struct CandleEngine {
    model_path: std::path::PathBuf,
    model_name: String,
}

impl CandleEngine {
    pub fn new(model_path: impl Into<std::path::PathBuf>, model_name: impl Into<String>) -> Self {
        Self {
            model_path: model_path.into(),
            model_name: model_name.into(),
        }
    }

    pub fn model_path(&self) -> &std::path::Path {
        &self.model_path
    }
}

impl ModelServingEngine for CandleEngine {
    fn engine_name(&self) -> &str {
        "candle"
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    async fn is_available(&self) -> bool {
        self.model_path.is_file()
    }

    async fn generate_raw(
        &self,
        _prompt: &str,
        _stop_tokens: Vec<String>,
    ) -> Result<String, ModelError> {
        if !self.is_available().await {
            return Err(ModelError::ModelUnavailable(format!(
                "Candle model weights not found at {:?}",
                self.model_path
            )));
        }
        Err(ModelError::Inference(
            "Candle in-process generation requires local weights to be compiled or loaded"
                .to_string(),
        ))
    }
}
