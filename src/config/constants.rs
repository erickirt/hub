use std::env;

pub fn stream_buffer_size_bytes() -> usize {
    env::var("STREAM_BUFFER_SIZE_BYTES")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000)
}

pub fn default_max_tokens() -> u32 {
    env::var("DEFAULT_MAX_TOKENS")
        .unwrap_or_else(|_| "4096".to_string())
        .parse()
        .unwrap_or(4096)
}

// Required field for the TitanEmbeddingRequest
pub fn default_embedding_dimension() -> u32 {
    env::var("DEFAULT_EMBEDDING_DIMENSION")
        .unwrap_or_else(|_| "512".to_string())
        .parse()
        .unwrap_or(512)
}
// Required field for the TitanEmbeddingRequest
pub fn default_embedding_normalize() -> bool {
    env::var("DEFAULT_EMBEDDING_NORMALIZE")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}
