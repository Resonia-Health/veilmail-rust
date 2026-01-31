use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// RSS feed management.
pub struct Feeds<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Feeds<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        self.http.post("/v1/feeds", Some(&params)).await
    }

    pub async fn list(&self) -> Result<Value> {
        self.http.get("/v1/feeds", None).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        self.http.get(&format!("/v1/feeds/{}", id), None).await
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .put(&format!("/v1/feeds/{}", id), &params)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/feeds/{}", id)).await
    }

    pub async fn poll(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/feeds/{}/poll", id), None)
            .await
    }

    pub async fn pause(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/feeds/{}/pause", id), None)
            .await
    }

    pub async fn resume(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/feeds/{}/resume", id), None)
            .await
    }

    pub async fn list_items(
        &self,
        feed_id: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        self.http
            .get(&format!("/v1/feeds/{}/items", feed_id), params)
            .await
    }
}
