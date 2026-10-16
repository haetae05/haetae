pub mod inference;
pub mod model_loader;
pub mod ollama;
pub mod ring_buffer;
pub mod zipper;

pub use inference::{strip_markdown_code_fences, strip_reasoning_tags, InferenceClient};
pub use model_loader::{parse_gguf_header, GgufHeader, ModelError, ModelLoader};
pub use ollama::{OllamaDaemonManager, OllamaLocator, OllamaStatus};
pub use ring_buffer::WeightRingBuffer;
pub use zipper::ZipperDecoder;
