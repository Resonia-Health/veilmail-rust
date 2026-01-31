use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Webhook endpoint management.
pub struct Webhooks<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Webhooks<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        let response = self.http.post("/v1/webhooks", Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/webhooks", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("/v1/webhooks/{}", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .patch(&format!("/v1/webhooks/{}", id), &params)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/webhooks/{}", id)).await
    }

    pub async fn test(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/webhooks/{}/test", id), None)
            .await
    }

    pub async fn rotate_secret(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/webhooks/{}/rotate-secret", id), None)
            .await?;
        Ok(unwrap_data(response))
    }
}

fn unwrap_data(value: Value) -> Value {
    match value.get("data") {
        Some(data) if data.is_object() => data.clone(),
        _ => value,
    }
}
