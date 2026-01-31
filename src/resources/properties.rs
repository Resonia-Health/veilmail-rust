use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Contact property management.
pub struct Properties<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Properties<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        let response = self.http.post("/v1/properties", Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/properties", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("/v1/properties/{}", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .patch(&format!("/v1/properties/{}", id), &params)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/properties/{}", id)).await
    }

    pub async fn get_values(&self, audience_id: &str, subscriber_id: &str) -> Result<Value> {
        self.http
            .get(
                &format!(
                    "/v1/audiences/{}/subscribers/{}/properties",
                    audience_id, subscriber_id
                ),
                None,
            )
            .await
    }

    pub async fn set_values(
        &self,
        audience_id: &str,
        subscriber_id: &str,
        values: Value,
    ) -> Result<Value> {
        self.http
            .put(
                &format!(
                    "/v1/audiences/{}/subscribers/{}/properties",
                    audience_id, subscriber_id
                ),
                &values,
            )
            .await
    }
}

fn unwrap_data(value: Value) -> Value {
    match value.get("data") {
        Some(data) if data.is_object() => data.clone(),
        _ => value,
    }
}
