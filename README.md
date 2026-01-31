# Veil Mail Rust SDK

The official Rust SDK for the [Veil Mail](https://veilmail.xyz) email API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
veilmail = "0.1"
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), veilmail::error::VeilMailError> {
    let client = veilmail::VeilMail::new("veil_live_xxxxx")?;

    let email = client.emails().send(json!({
        "from": "hello@yourdomain.com",
        "to": ["user@example.com"],
        "subject": "Hello from Rust!",
        "html": "<h1>Welcome!</h1>"
    })).await?;

    println!("Sent: {}", email);
    Ok(())
}
```

## Configuration

```rust
use veilmail::{VeilMail, VeilMailOptions};

let client = VeilMail::with_options("veil_live_xxxxx", Some(VeilMailOptions {
    base_url: Some("https://custom-api.example.com"),
    timeout_secs: Some(10),
}))?;
```

## Resources

| Resource | Accessor | Description |
|----------|----------|-------------|
| Emails | `client.emails()` | Send, batch send, list, get, cancel, update |
| Domains | `client.domains()` | Create, verify, update, list, delete |
| Templates | `client.templates()` | Create, update, preview, list, delete |
| Audiences | `client.audiences()` | Manage audiences and subscribers |
| Campaigns | `client.campaigns()` | Create, schedule, send, pause, resume, cancel |
| Webhooks | `client.webhooks()` | Manage endpoints, test, rotate secrets |
| Topics | `client.topics()` | Manage subscription topics and preferences |
| Properties | `client.properties()` | Manage contact property definitions and values |

## Sending Emails

```rust
use serde_json::json;

// Simple send
let email = client.emails().send(json!({
    "from": "hello@yourdomain.com",
    "to": ["user@example.com"],
    "subject": "Hello!",
    "html": "<p>Hello World!</p>"
})).await?;

// With template
let email = client.emails().send(json!({
    "from": "hello@yourdomain.com",
    "to": ["user@example.com"],
    "templateId": "tmpl_xxx",
    "templateData": { "name": "Alice" }
})).await?;

// Batch send (up to 100)
let result = client.emails().send_batch(vec![
    json!({ "from": "hi@yourdomain.com", "to": ["user1@example.com"], "subject": "Hi", "html": "<p>Hi!</p>" }),
    json!({ "from": "hi@yourdomain.com", "to": ["user2@example.com"], "subject": "Hi", "html": "<p>Hi!</p>" }),
]).await?;
```

## Subscriber Management

```rust
use serde_json::json;

// Get subscribers for an audience
let subs = client.audiences().subscribers("audience_xxxxx");

// Add a subscriber
let subscriber = subs.add(json!({
    "email": "user@example.com",
    "firstName": "Alice",
    "lastName": "Smith"
})).await?;

// List subscribers
let list = subs.list(Some(&[("limit", "50"), ("status", "active")])).await?;

// Export as CSV
let csv = subs.export(None).await?;
```

## Error Handling

```rust
use veilmail::error::VeilMailError;

match client.emails().send(params).await {
    Ok(email) => println!("Sent: {}", email),
    Err(VeilMailError::Authentication { message, .. }) => {
        eprintln!("Invalid API key: {}", message);
    }
    Err(VeilMailError::PiiDetected { pii_types, .. }) => {
        eprintln!("PII detected: {:?}", pii_types);
    }
    Err(VeilMailError::RateLimit { retry_after, .. }) => {
        if let Some(secs) = retry_after {
            eprintln!("Rate limited, retry after {} seconds", secs);
        }
    }
    Err(VeilMailError::Validation { message, details, .. }) => {
        eprintln!("Validation error: {} ({:?})", message, details);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Webhook Verification

```rust
use veilmail::webhook::verify_signature;

// In your web handler (e.g., with Actix Web or Axum)
fn handle_webhook(body: &str, signature: &str) -> bool {
    let secret = "whsec_xxxxx";
    verify_signature(body, signature, secret)
}
```

### Axum Example

```rust
use axum::{extract::Request, http::StatusCode, routing::post, Router};
use veilmail::webhook::verify_signature;

const WEBHOOK_SECRET: &str = "whsec_xxxxx";

async fn webhook_handler(request: Request) -> StatusCode {
    let signature = request
        .headers()
        .get("x-signature-hash")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let body = axum::body::to_bytes(request.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body);

    if !verify_signature(&body_str, signature, WEBHOOK_SECRET) {
        return StatusCode::UNAUTHORIZED;
    }

    let event: serde_json::Value = serde_json::from_slice(&body).unwrap();
    match event["type"].as_str() {
        Some("email.delivered") => println!("Delivered: {}", event["data"]),
        Some("email.bounced") => println!("Bounced: {}", event["data"]),
        _ => {}
    }

    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/webhooks/veilmail", post(webhook_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## License

MIT
