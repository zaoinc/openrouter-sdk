use crate::{models::*, error::OpenRouterError};
use reqwest::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OpenRouterClient {
    inner: Arc<Client>,
    api_key: String,
    base_url: String,
}

impl OpenRouterClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(Client::new()),
            api_key: api_key.into(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    pub async fn chat(
        &self,
        request: ChatRequest
    ) -> Result<ChatResponse, OpenRouterError> {
        self.request("/chat/completions", request).await
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl serde::Serialize,
    ) -> Result<T, OpenRouterError> {
        let response = self.inner
            .post(format!("{}{}", self.base_url, endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OpenRouterError::ApiError(error_text));
        }

        response.json().await.map_err(Into::into)
    }
}