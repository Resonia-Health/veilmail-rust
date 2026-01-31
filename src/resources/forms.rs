use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Signup form management.
pub struct Forms<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Forms<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        self.http.post("/v1/forms", Some(&params)).await
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/forms", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        self.http.get(&format!("/v1/forms/{}", id), None).await
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .put(&format!("/v1/forms/{}", id), &params)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/forms/{}", id)).await
    }
}
