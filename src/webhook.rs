use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify a webhook signature using constant-time HMAC-SHA256 comparison.
///
/// # Arguments
///
/// * `body` - The raw request body
/// * `signature` - The signature from the X-Signature-Hash header
/// * `secret` - The webhook signing secret
///
/// # Returns
///
/// `true` if the signature is valid
///
/// # Example
///
/// ```
/// use veilmail::webhook::verify_signature;
///
/// let body = r#"{"type":"email.delivered"}"#;
/// let secret = "whsec_test";
/// // In practice, the signature comes from the X-Signature-Hash header
/// let valid = verify_signature(body, "some_signature", secret);
/// ```
pub fn verify_signature(body: &str, signature: &str, secret: &str) -> bool {
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };

    mac.update(body.as_bytes());
    let result = mac.finalize();
    let expected = hex::encode(result.into_bytes());

    constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
