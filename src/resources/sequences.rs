use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Automation sequence management.
pub struct Sequences<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Sequences<'a> {
    pub async fn create(&self, params: Value) -> Result<Value> {
        self.http.post("/v1/sequences", Some(&params)).await
    }

    pub async fn list(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/sequences", params).await
    }

    pub async fn get(&self, id: &str) -> Result<Value> {
        self.http.get(&format!("/v1/sequences/{}", id), None).await
    }

    pub async fn update(&self, id: &str, params: Value) -> Result<Value> {
        self.http
            .put(&format!("/v1/sequences/{}", id), &params)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.http.delete(&format!("/v1/sequences/{}", id)).await
    }

    pub async fn activate(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/sequences/{}/activate", id), None)
            .await
    }

    pub async fn pause(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/sequences/{}/pause", id), None)
            .await
    }

    pub async fn archive(&self, id: &str) -> Result<Value> {
        self.http
            .post(&format!("/v1/sequences/{}/archive", id), None)
            .await
    }

    pub async fn add_step(&self, sequence_id: &str, params: Value) -> Result<Value> {
        self.http
            .post(
                &format!("/v1/sequences/{}/steps", sequence_id),
                Some(&params),
            )
            .await
    }

    pub async fn update_step(
        &self,
        sequence_id: &str,
        step_id: &str,
        params: Value,
    ) -> Result<Value> {
        self.http
            .put(
                &format!("/v1/sequences/{}/steps/{}", sequence_id, step_id),
                &params,
            )
            .await
    }

    pub async fn delete_step(&self, sequence_id: &str, step_id: &str) -> Result<()> {
        self.http
            .delete(&format!(
                "/v1/sequences/{}/steps/{}",
                sequence_id, step_id
            ))
            .await
    }

    pub async fn reorder_steps(&self, sequence_id: &str, params: Value) -> Result<Value> {
        self.http
            .post(
                &format!("/v1/sequences/{}/steps/reorder", sequence_id),
                Some(&params),
            )
            .await
    }

    pub async fn enroll(&self, sequence_id: &str, params: Value) -> Result<Value> {
        self.http
            .post(
                &format!("/v1/sequences/{}/enroll", sequence_id),
                Some(&params),
            )
            .await
    }

    pub async fn list_enrollments(
        &self,
        sequence_id: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        self.http
            .get(
                &format!("/v1/sequences/{}/enrollments", sequence_id),
                params,
            )
            .await
    }

    pub async fn remove_enrollment(
        &self,
        sequence_id: &str,
        enrollment_id: &str,
    ) -> Result<()> {
        self.http
            .delete(&format!(
                "/v1/sequences/{}/enrollments/{}",
                sequence_id, enrollment_id
            ))
            .await
    }
}
