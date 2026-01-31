use std::collections::HashMap;
use thiserror::Error;

/// Error types returned by the Veil Mail API.
#[derive(Debug, Error)]
pub enum VeilMailError {
    #[error("Authentication failed: {message}")]
    Authentication {
        message: String,
        code: Option<String>,
    },

    #[error("Access denied: {message}")]
    Forbidden {
        message: String,
        code: Option<String>,
    },

    #[error("Resource not found: {message}")]
    NotFound {
        message: String,
        code: Option<String>,
    },

    #[error("Validation error: {message}")]
    Validation {
        message: String,
        code: Option<String>,
        details: Option<HashMap<String, serde_json::Value>>,
    },

    #[error("PII detected: {message}")]
    PiiDetected {
        message: String,
        pii_types: Vec<String>,
        code: Option<String>,
    },

    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        retry_after: Option<u64>,
        code: Option<String>,
    },

    #[error("Server error: {message}")]
    Server {
        message: String,
        status_code: u16,
        code: Option<String>,
    },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

impl VeilMailError {
    pub(crate) fn from_response(status: u16, body: &serde_json::Value) -> Self {
        let error = body.get("error").unwrap_or(body);
        let message = error
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error")
            .to_string();
        let code = error.get("code").and_then(|v| v.as_str()).map(String::from);

        match status {
            401 => VeilMailError::Authentication { message, code },
            403 => VeilMailError::Forbidden { message, code },
            404 => VeilMailError::NotFound { message, code },
            400 => VeilMailError::Validation {
                message,
                code,
                details: error
                    .get("details")
                    .and_then(|v| serde_json::from_value(v.clone()).ok()),
            },
            422 => {
                let pii_types = error
                    .get("piiTypes")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                if code.as_deref() == Some("pii_detected") || !pii_types.is_empty() {
                    VeilMailError::PiiDetected {
                        message,
                        pii_types,
                        code,
                    }
                } else {
                    VeilMailError::Validation {
                        message,
                        code,
                        details: None,
                    }
                }
            }
            429 => {
                let retry_after = error.get("retryAfter").and_then(|v| v.as_u64());
                VeilMailError::RateLimit {
                    message,
                    retry_after,
                    code,
                }
            }
            s if s >= 500 => VeilMailError::Server {
                message,
                status_code: s,
                code,
            },
            _ => VeilMailError::Other(message),
        }
    }
}

pub type Result<T> = std::result::Result<T, VeilMailError>;
