use veilmail::VeilMailClient;

pub struct MailService {
    client: VeilMailClient,
    from: String,
    app_url: String,
}

impl MailService {
    pub fn new() -> Self {
        Self {
            client: VeilMailClient::new(
                &std::env::var("VEILMAIL_API_KEY").expect("VEILMAIL_API_KEY required"),
            ),
            from: std::env::var("VEILMAIL_FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@veilmail.xyz".to_string()),
            app_url: std::env::var("APP_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        }
    }

    pub async fn send_verification_email(&self, email: &str, name: &str, token: &str) {
        let url = format!("{}/auth/verify-email?token={}", self.app_url, token);
        self.client
            .emails()
            .send(
                &self.from,
                email,
                "Verify your email address",
                &format!(
                    "<p>Hi {},</p><p>Click <a href=\"{}\">here</a> to verify your email.</p>",
                    name, url
                ),
                &["auth", "verification"],
            )
            .await
            .ok();
    }

    pub async fn send_password_reset_email(&self, email: &str, token: &str) {
        let url = format!("{}/auth/reset-password?token={}", self.app_url, token);
        self.client
            .emails()
            .send(
                &self.from,
                email,
                "Reset your password",
                &format!(
                    "<p>Click <a href=\"{}\">here</a> to reset your password.</p>",
                    url
                ),
                &["auth", "password-reset"],
            )
            .await
            .ok();
    }

    pub async fn send_two_factor_code(&self, email: &str, code: &str) {
        self.client
            .emails()
            .send(
                &self.from,
                email,
                &format!("{} is your verification code", code),
                &format!(
                    "<p>Your code: <strong>{}</strong></p><p>Expires in 5 minutes.</p>",
                    code
                ),
                &["auth", "2fa"],
            )
            .await
            .ok();
    }

    pub async fn send_welcome_email(&self, email: &str, name: &str) {
        self.client
            .emails()
            .send(
                &self.from,
                email,
                "Welcome!",
                &format!("<p>Welcome, {}! Your account is active.</p>", name),
                &["auth", "welcome"],
            )
            .await
            .ok();
    }

    pub async fn send_password_changed_email(&self, email: &str) {
        self.client
            .emails()
            .send(
                &self.from,
                email,
                "Your password was changed",
                "<p>Your password was changed. If you didn't do this, reset it immediately.</p>",
                &["auth", "security"],
            )
            .await
            .ok();
    }

    pub async fn send_2fa_toggled_email(&self, email: &str, enabled: bool) {
        let status = if enabled { "enabled" } else { "disabled" };
        self.client
            .emails()
            .send(
                &self.from,
                email,
                &format!("Two-factor authentication {}", status),
                &format!("<p>2FA has been {} on your account.</p>", status),
                &["auth", "2fa", "security"],
            )
            .await
            .ok();
    }
}
