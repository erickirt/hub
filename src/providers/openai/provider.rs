use crate::config::constants::stream_buffer_size_bytes;
use crate::config::models::{ModelConfig, Provider as ProviderConfig};
use crate::models::chat::{ChatCompletionRequest, ChatCompletionResponse};
use crate::models::completion::{CompletionRequest, CompletionResponse};
use crate::models::embeddings::{EmbeddingsRequest, EmbeddingsResponse};
use crate::models::streaming::ChatCompletionChunk;
use crate::providers::provider::Provider;
use async_trait::async_trait;
use axum::http::StatusCode;
use reqwest::Client;
use reqwest_streams::*;
use tracing::info;

pub struct OpenAIProvider {
    config: ProviderConfig,
    http_client: Client,
}

impl OpenAIProvider {
    fn base_url(&self) -> String {
        self.config
            .params
            .get("base_url")
            .unwrap_or(&String::from("https://api.openai.com/v1"))
            .to_string()
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn new(config: &ProviderConfig) -> Self {
        Self {
            config: config.clone(),
            http_client: Client::new(),
        }
    }

    fn key(&self) -> String {
        self.config.key.clone()
    }

    fn r#type(&self) -> String {
        "openai".to_string()
    }

    async fn chat_completions(
        &self,
        payload: ChatCompletionRequest,
        _model_config: &ModelConfig,
    ) -> Result<ChatCompletionResponse, StatusCode> {
        let response = self
            .http_client
            .post(format!("{}/chat/completions", self.base_url()))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                eprintln!("OpenAI API request error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let status = response.status();
        if status.is_success() {
            if payload.stream.unwrap_or(false) {
                let stream =
                    response.json_array_stream::<ChatCompletionChunk>(stream_buffer_size_bytes());
                Ok(ChatCompletionResponse::Stream(stream))
            } else {
                response
                    .json()
                    .await
                    .map(ChatCompletionResponse::NonStream)
                    .map_err(|e| {
                        eprintln!("OpenAI API response error: {e}");
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
            }
        } else {
            info!(
                "OpenAI API request error: {}",
                response.text().await.unwrap()
            );
            Err(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }

    async fn completions(
        &self,
        payload: CompletionRequest,
        _model_config: &ModelConfig,
    ) -> Result<CompletionResponse, StatusCode> {
        let response = self
            .http_client
            .post(format!("{}/completions", self.base_url()))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                eprintln!("OpenAI API request error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let status = response.status();
        if status.is_success() {
            response.json().await.map_err(|e| {
                eprintln!("OpenAI API response error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })
        } else {
            eprintln!(
                "OpenAI API request error: {}",
                response.text().await.unwrap()
            );
            Err(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }

    async fn embeddings(
        &self,
        payload: EmbeddingsRequest,
        _model_config: &ModelConfig,
    ) -> Result<EmbeddingsResponse, StatusCode> {
        let response = self
            .http_client
            .post(format!("{}/embeddings", self.base_url()))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                eprintln!("OpenAI API request error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let status = response.status();
        if status.is_success() {
            response.json().await.map_err(|e| {
                eprintln!("OpenAI API response error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })
        } else {
            eprintln!(
                "OpenAI API request error: {}",
                response.text().await.unwrap()
            );
            Err(StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
