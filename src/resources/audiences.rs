use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Audience management.
pub struct Audiences<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Audiences<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        let response = self.http.post("/v1/audiences", Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/audiences", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("/v1/audiences/{}", id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .put(&format!("/v1/audiences/{}", id), &params)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/audiences/{}", id)).await
    }

    /// Get a Subscribers helper scoped to the given audience.
    pub fn subscribers(&self, audience_id: &str) -> Subscribers<'a> {
        Subscribers {
            http: self.http,
            base_path: format!("/v1/audiences/{}/subscribers", audience_id),
        }
    }

    /// Recalculate engagement scores for all subscribers.
    pub async fn recalculate_engagement(&self, audience_id: &str) -> Result<Value> {
        let body = serde_json::json!({});
        self.http
            .post(
                &format!("/v1/audiences/{}/recalculate-engagement", audience_id),
                Some(&body),
            )
            .await
    }

    /// Get engagement statistics for an audience.
    pub async fn get_engagement_stats(&self, audience_id: &str) -> Result<Value> {
        self.http
            .get(
                &format!("/v1/audiences/{}/engagement-stats", audience_id),
                None,
            )
            .await
    }
}

/// Subscriber management within an audience.
pub struct Subscribers<'a> {
    http: &'a HttpClient,
    base_path: String,
}

impl<'a> Subscribers<'a> {
    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get(&self.base_path, params).await
    }

    pub async fn add(&self, params: Value) -> Result<Value> {
        let response = self.http.post(&self.base_path, Some(&params)).await?;
        Ok(unwrap_data(response))
    }

    pub async fn get(&self, subscriber_id: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("{}/{}", self.base_path, subscriber_id), None)
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn update(&self, subscriber_id: &str, params: Value) -> Result<Value> {
        let response = self
            .http
            .put(
                &format!("{}/{}", self.base_path, subscriber_id),
                &params,
            )
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn remove(&self, subscriber_id: &str) -> Result<()> {
        self.http
            .delete(&format!("{}/{}", self.base_path, subscriber_id))
            .await
    }

    pub async fn confirm(&self, subscriber_id: &str) -> Result<Value> {
        let response = self
            .http
            .post(
                &format!("{}/{}/confirm", self.base_path, subscriber_id),
                None,
            )
            .await?;
        Ok(unwrap_data(response))
    }

    pub async fn import(&self, params: Value) -> Result<Value> {
        self.http
            .post(&format!("{}/import", self.base_path), Some(&params))
            .await
    }

    pub async fn export(&self, params: Option<&[(&str, &str)]>) -> Result<String> {
        self.http
            .get_raw(&format!("{}/export", self.base_path), params)
            .await
    }

    pub async fn activity(
        &self,
        subscriber_id: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        self.http
            .get(
                &format!("{}/{}/activity", self.base_path, subscriber_id),
                params,
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
