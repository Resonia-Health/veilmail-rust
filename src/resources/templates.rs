use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Email template management.
pub struct Templates<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Templates<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        let response = self.http.post("/v1/templates", Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/templates", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("/v1/templates/{}", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .patch(&format!("/v1/templates/{}", id), &params)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn preview(&self, params: Value) -> Result<Value> {
        self.http.post("/v1/templates/preview", Some(&params)).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/templates/{}", id)).await
    }
}

fn unwrap_data(value: Value) -> Value {
    match value.get("data") {
        Some(data) if data.is_object() => data.clone(),
        _ => value,
    }
}
