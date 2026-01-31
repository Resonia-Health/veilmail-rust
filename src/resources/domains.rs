use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Domain management for email sending.
pub struct Domains<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Domains<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        let response = self.http.post("/v1/domains", Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/domains", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        let response = self.http.get(&format!("/v1/domains/{}", id), None).await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .patch(&format!("/v1/domains/{}", id), &params)
            .await
    }

    pub async fn verify(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .post(&format!("/v1/domains/{}/verify", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/domains/{}", id)).await
    }
}

fn unwrap_data(value: Value) -> Value {
    match value.get("data") {
        Some(data) if data.is_object() => data.clone(),
        _ => value,
    }
}
