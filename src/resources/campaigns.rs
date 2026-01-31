use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Campaign management.
pub struct Campaigns<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Campaigns<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        let response = self.http.post("/v1/campaigns", Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/campaigns", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("/v1/campaigns/{}", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .patch(&format!("/v1/campaigns/{}", id), &params)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/campaigns/{}", id)).await
    }

    pub async fn schedule(&self, id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/campaigns/{}/schedule", id), Some(&params))
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn send(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/campaigns/{}/send", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn pause(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/campaigns/{}/pause", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn resume(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/campaigns/{}/resume", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn cancel(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/campaigns/{}/cancel", id), None)
            .await?;
        Ok(unwrap_data(response))
    }
}

    pub async fn send_test(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .post(&format!("/v1/campaigns/{}/test", id), Some(&params))
            .await
    }

    pub async fn clone_campaign(&self, id: &str, params: Option<Value>) -> Result<Value> {
        let body = params.unwrap_or_else(|| serde_json::json!({}));
        self.http
            .post(&format!("/v1/campaigns/{}/clone", id), Some(&body))
            .await
    }

    pub async fn links(&self, id: &str, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http
            .get(&format!("/v1/campaigns/{}/links", id), params)
            .await
    }
}

fn unwrap_data(value: Value) -> Value {
    match value.get("data") {
        Some(data) if data.is_object() => data.clone(),
        _ => value,
    }
}
