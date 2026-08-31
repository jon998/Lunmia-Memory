pub mod classifier;
pub mod context;
pub mod embeddings;
pub mod formato;
pub mod llm;
pub mod llm_log;
pub mod pipeline;
pub mod rules;

pub use llm::LlmClient;
pub use pipeline::{ClassificationResult, Pipeline};
