use crate::ring_buffer::WeightRingBuffer;
use haetae_core::config::{
    GGUF_HEADER_PROBE_BYTES, GGUF_MAGIC_LE, GGUF_VERSION_V3, HTTP_OK, HTTP_PARTIAL_CONTENT,
    HTTP_TIMEOUT_SECS,
};
use std::time::Duration;

pub const GGUF_MAGIC_SIZE: usize = std::mem::size_of::<u32>();
pub const GGUF_VERSION_SIZE: usize = std::mem::size_of::<u32>();
pub const GGUF_TENSOR_COUNT_SIZE: usize = std::mem::size_of::<u64>();
pub const GGUF_METADATA_COUNT_SIZE: usize = std::mem::size_of::<u64>();

pub const GGUF_MAGIC_END: usize = GGUF_MAGIC_SIZE;
pub const GGUF_VERSION_END: usize = GGUF_MAGIC_END + GGUF_VERSION_SIZE;
pub const GGUF_TENSOR_COUNT_END: usize = GGUF_VERSION_END + GGUF_TENSOR_COUNT_SIZE;
pub const GGUF_METADATA_COUNT_END: usize = GGUF_TENSOR_COUNT_END + GGUF_METADATA_COUNT_SIZE;
pub const GGUF_MIN_HEADER_BYTES: usize = GGUF_METADATA_COUNT_END;

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("Invalid GGUF magic bytes: expected {0:#x}, got {1:#x}")]
    InvalidMagic(u32, u32),
    #[error("Unsupported GGUF version: {0}")]
    UnsupportedVersion(u32),
    #[error("Buffer too short for GGUF header: {0} bytes")]
    HeaderTooShort(usize),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Server returned status: {0}")]
    HttpStatus(u16),
    #[error("Inference error: {0}")]
    Inference(String),
    #[error("Model unavailable: {0}")]
    ModelUnavailable(String),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GgufHeader {
    pub magic: u32,
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
}

pub fn parse_gguf_header(bytes: &[u8]) -> Result<GgufHeader, ModelError> {
    if bytes.len() < GGUF_MIN_HEADER_BYTES {
        return Err(ModelError::HeaderTooShort(bytes.len()));
    }

    let magic = u32::from_le_bytes(
        bytes[0..GGUF_MAGIC_END]
            .try_into()
            .map_err(|_| ModelError::HeaderTooShort(bytes.len()))?,
    );
    if magic != GGUF_MAGIC_LE {
        return Err(ModelError::InvalidMagic(GGUF_MAGIC_LE, magic));
    }

    let version = u32::from_le_bytes(
        bytes[GGUF_MAGIC_END..GGUF_VERSION_END]
            .try_into()
            .map_err(|_| ModelError::HeaderTooShort(bytes.len()))?,
    );
    if version != GGUF_VERSION_V3 {
        return Err(ModelError::UnsupportedVersion(version));
    }

    let tensor_count = u64::from_le_bytes(
        bytes[GGUF_VERSION_END..GGUF_TENSOR_COUNT_END]
            .try_into()
            .map_err(|_| ModelError::HeaderTooShort(bytes.len()))?,
    );

    let metadata_kv_count = u64::from_le_bytes(
        bytes[GGUF_TENSOR_COUNT_END..GGUF_METADATA_COUNT_END]
            .try_into()
            .map_err(|_| ModelError::HeaderTooShort(bytes.len()))?,
    );

    Ok(GgufHeader {
        magic,
        version,
        tensor_count,
        metadata_kv_count,
    })
}

pub struct ModelLoader {
    client: reqwest::Client,
    model_url: String,
}

impl ModelLoader {
    pub fn new(model_url: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            model_url: model_url.into(),
        }
    }

    pub async fn probe_remote_header(&self) -> Result<GgufHeader, ModelError> {
        let range_header = format!("bytes=0-{}", GGUF_HEADER_PROBE_BYTES - 1);
        let resp = self
            .client
            .get(&self.model_url)
            .header("Range", range_header)
            .send()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        let status = resp.status().as_u16();
        if status != HTTP_OK && status != HTTP_PARTIAL_CONTENT {
            return Err(ModelError::HttpStatus(status));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| ModelError::Network(e.to_string()))?;

        parse_gguf_header(&bytes)
    }

    pub fn stream_slice_into_ring(&self, slice_data: &[u8], ring: &mut WeightRingBuffer) {
        ring.prefetch_next_slice(slice_data);
        ring.swap_buffers();
    }
}
