//! Official Rust SDK for the Veil Mail API.
//!
//! # Quick Start
//!
//! ```no_run
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), veilmail::error::VeilMailError> {
//!     let client = veilmail::VeilMail::new("veil_live_xxxxx")?;
//!
//!     let email = client.emails().send(json!({
//!         "from": "hello@yourdomain.com",
//!         "to": ["user@example.com"],
//!         "subject": "Hello from Rust!",
//!         "html": "<h1>Welcome!</h1>"
//!     })).await?;
//!
//!     println!("Sent: {}", email);
//!     Ok(())
//! }
//! ```

pub mod error;
mod http;
pub mod resources;
pub mod webhook;

use error::Result;
use http::HttpClient;
use resources::{
    analytics::Analytics, audiences::Audiences, campaigns::Campaigns, domains::Domains,
    emails::Emails, feeds::Feeds, forms::Forms, properties::Properties, sequences::Sequences,
    templates::Templates, topics::Topics, webhooks::Webhooks,
};

/// Options for configuring the Veil Mail client.
pub struct VeilMailOptions<'a> {
    /// Custom base URL for the API (defaults to `https://api.veilmail.xyz`).
    pub base_url: Option<&'a str>,
    /// Request timeout in seconds (defaults to 30).
    pub timeout_secs: Option<u64>,
}

/// The Veil Mail API client.
///
/// Create an instance with [`VeilMail::new`] or [`VeilMail::with_options`] and
/// use the resource accessors to interact with the API.
pub struct VeilMail {
    http: HttpClient,
}

impl VeilMail {
    /// Create a new client with the given API key.
    ///
    /// The key must start with `veil_live_` or `veil_test_`.
    pub fn new(api_key: &str) -> Result<Self> {
        Self::with_options(api_key, None)
    }

    /// Create a new client with custom options.
    pub fn with_options(api_key: &str, options: Option<VeilMailOptions<'_>>) -> Result<Self> {
        if !api_key.starts_with("veil_live_") && !api_key.starts_with("veil_test_") {
            return Err(error::VeilMailError::Other(
                "API key must start with 'veil_live_' or 'veil_test_'".to_string(),
            ));
        }

        let (base_url, timeout) = match options {
            Some(opts) => (opts.base_url, opts.timeout_secs),
            None => (None, None),
        };

        let http = HttpClient::new(api_key, base_url, timeout)?;
        Ok(Self { http })
    }

    /// Email sending and management.
    pub fn emails(&self) -> Emails<'_> {
        Emails { http: &self.http }
    }

    /// Domain management for email sending.
    pub fn domains(&self) -> Domains<'_> {
        Domains { http: &self.http }
    }

    /// Email template management.
    pub fn templates(&self) -> Templates<'_> {
        Templates { http: &self.http }
    }

    /// Audience management.
    pub fn audiences(&self) -> Audiences<'_> {
        Audiences { http: &self.http }
    }

    /// Campaign management.
    pub fn campaigns(&self) -> Campaigns<'_> {
        Campaigns { http: &self.http }
    }

    /// Webhook endpoint management.
    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { http: &self.http }
    }

    /// Subscription topic management.
    pub fn topics(&self) -> Topics<'_> {
        Topics { http: &self.http }
    }

    /// Contact property management.
    pub fn properties(&self) -> Properties<'_> {
        Properties { http: &self.http }
    }

    /// Automation sequence management.
    pub fn sequences(&self) -> Sequences<'_> {
        Sequences { http: &self.http }
    }

    /// RSS feed management.
    pub fn feeds(&self) -> Feeds<'_> {
        Feeds { http: &self.http }
    }

    /// Signup form management.
    pub fn forms(&self) -> Forms<'_> {
        Forms { http: &self.http }
    }

    /// Geo and device analytics.
    pub fn analytics(&self) -> Analytics<'_> {
        Analytics { http: &self.http }
    }
}
