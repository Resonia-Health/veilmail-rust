# Actix-web Auth Example with VeilMail

Authentication email integration using the VeilMail Rust SDK in an Actix-web application.

## Key Files

- `src/mail.rs` - Mail service with auth email methods

## Setup

1. Add the VeilMail crate to your `Cargo.toml`:
   ```toml
   [dependencies]
   veilmail = "0.1"
   ```
2. Copy `src/mail.rs` into your Actix-web project
3. Set environment variables:
   ```bash
   export VEILMAIL_API_KEY=veil_live_your_key
   export VEILMAIL_FROM_EMAIL=noreply@yourdomain.com
   export APP_URL=https://yourdomain.com
   ```
4. Add `MailService` as app data in your Actix-web server:
   ```rust
   let mail = web::Data::new(MailService::new());
   App::new().app_data(mail.clone())
   ```

## Emails Covered

- Email verification
- Password reset
- Two-factor authentication codes
- Welcome email
- Password changed notification
- 2FA toggled notification
