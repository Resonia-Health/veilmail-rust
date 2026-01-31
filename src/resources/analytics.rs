use serde_json::Value;

use crate::error::Result;
use crate::http::HttpClient;

/// Geo and device analytics.
pub struct Analytics<'a> {
    pub(crate) http: &'a HttpClient,
}

impl<'a> Analytics<'a> {
    /// Get organization-level geo analytics.
    pub async fn geo(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/analytics/geo", params).await
    }

    /// Get organization-level device analytics.
    pub async fn devices(&self, params: Option<&[(&str, &str)]>) -> Result<Value> {
        self.http.get("/v1/analytics/devices", params).await
    }

    /// Get campaign-level geo analytics.
    pub async fn campaign_geo(
        &self,
        campaign_id: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        self.http
            .get(
                &format!("/v1/campaigns/{}/analytics/geo", campaign_id),
                params,
            )
            .await
    }

    /// Get campaign-level device analytics.
    pub async fn campaign_devices(
        &self,
        campaign_id: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        self.http
            .get(
                &format!("/v1/campaigns/{}/analytics/devices", campaign_id),
                params,
            )
            .await
    }
}
