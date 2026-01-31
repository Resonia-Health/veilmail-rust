use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

use crate::error::{Result, VeilMailError};

const DEFAULT_BASE_URL: &str = "https://api.veilmail.xyz";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const VERSION: &str = "0.1.0";

pub(crate) struct HttpClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl HttpClient {
    pub(crate) fn new(api_key: &str, base_url: Option<&str>, timeout_secs: Option<u64>) -> Result<Self> {
        let timeout = Duration::from_secs(timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(format!("veilmail-rust/{}", VERSION))
            .build()
            .map_err(VeilMailError::Http)?;

        Ok(Self {
            client,
            base_url: base_url
                .unwrap_or(DEFAULT_BASE_URL)
                .trim_end_matches('/')
                .to_string(),
            api_key: api_key.to_string(),
        })
    }

    pub(crate) async fn get(&self, path: &str, query: Option<&[(&str, &str)]>) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url).bearer_auth(&self.api_key);

        if let Some(params) = query {
            let filtered: Vec<_> = params.iter().filter(|(_, v)| !v.is_empty()).collect();
            if !filtered.is_empty() {
                req = req.query(&filtered);
            }
        }

        self.handle_response(req.send().await?).await
    }

    pub(crate) async fn post(&self, path: &str, body: Option<&Value>) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.post(&url).bearer_auth(&self.api_key);

        if let Some(b) = body {
            req = req.json(b);
        }

        self.handle_response(req.send().await?).await
    }

    pub(crate) async fn patch(&self, path: &str, body: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let req = self
            .client
            .patch(&url)
            .bearer_auth(&self.api_key)
            .json(body);

        self.handle_response(req.send().await?).await
    }

    pub(crate) async fn put(&self, path: &str, body: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let req = self
            .client
            .put(&url)
            .bearer_auth(&self.api_key)
            .json(body);

        self.handle_response(req.send().await?).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let status = resp.status().as_u16();
        if status >= 400 {
            let body: Value = resp.json().await.unwrap_or(Value::Null);
            return Err(VeilMailError::from_response(status, &body));
        }

        Ok(())
    }

    pub(crate) async fn get_raw(&self, path: &str, query: Option<&[(&str, &str)]>) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url).bearer_auth(&self.api_key);

        if let Some(params) = query {
            let filtered: Vec<_> = params.iter().filter(|(_, v)| !v.is_empty()).collect();
            if !filtered.is_empty() {
                req = req.query(&filtered);
            }
        }

        let resp = req.send().await?;
        let status = resp.status().as_u16();

        if status >= 400 {
            let body: Value = resp.json().await.unwrap_or(Value::Null);
            return Err(VeilMailError::from_response(status, &body));
        }

        Ok(resp.text().await?)
    }

    async fn handle_response(&self, resp: reqwest::Response) -> Result<Value> {
        let status = resp.status().as_u16();

        if status == 204 {
            return Ok(Value::Object(serde_json::Map::new()));
        }

        let body: Value = resp.json().await.unwrap_or(Value::Null);

        if status >= 400 {
            return Err(VeilMailError::from_response(status, &body));
        }

        Ok(body)
    }
}
