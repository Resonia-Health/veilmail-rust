use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Subscription topic management.
pub struct Topics<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Topics<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        self.http.post("/v1/topics", Some(&params)).await
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/topics", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        self.http.get(&format!("/v1/topics/{}", id), None).await
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .patch(&format!("/v1/topics/{}", id), &params)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/topics/{}", id)).await
    }

    pub async fn get_preferences(&self, audience_id: &str, subscriber_id: &str) -> Result<Value> {
        self.http
            .get(
                &format!(
                    "/v1/audiences/{}/subscribers/{}/topics",
                    audience_id, subscriber_id
                ),
                None,
            )
            .await
    }

    pub async fn set_preferences(
        &self,
        audience_id: &str,
        subscriber_id: &str,
        params: Value,
    ) -> Result<Value> {
        self.http
            .put(
                &format!(
                    "/v1/audiences/{}/subscribers/{}/topics",
                    audience_id, subscriber_id
                ),
                &params,
            )
            .await
    }
}
