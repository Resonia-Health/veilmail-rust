use serde_json::{json, Value};

use crate::error::Result;
use crate::http::HttpClient;

/// Email sending and management.
pub struct Emails<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Emails<'a> {
    /// Send a single email.
    pub async fn send(&self, params: Value) -> Result<Value> {
        self.http.post("/v1/emails", Some(&params)).await
    }

    /// Send a batch of up to 100 emails.
    pub async fn send_batch(&self, emails: Vec<Value>) -> Result<Value> {
        let body = json!({ "emails": emails });
        self.http.post("/v1/emails/batch", Some(&body)).await
    }

    /// List emails with optional filters.
    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/emails", params).await
    }

    /// Get a single email by ID.
    pub async fn get(&self, id: &str) -> Result<Value> {
        self.http.get(&format!("/v1/emails/{}", id), None).await
    }

    /// Cancel a scheduled email.
    pub async fn cancel(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/emails/{}/cancel", id), None)
            .await
    }

    /// Reschedule a scheduled email.
    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .patch(&format!("/v1/emails/{}", id), &params)
            .await
    }

    /// Get tracked link analytics for a specific email.
    pub async fn links(&self, id: &str, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http
            .get(&format!("/v1/emails/{}/links", id), params)
            .await
    }
}
