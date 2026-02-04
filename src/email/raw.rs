
use lettre::{
    Message, SmtpTransport, Transport,
    message::header::ContentType,
    transport::smtp::authentication::{self, Credentials}
};

use crate::env::Env;
pub struct RawEmailService;

impl RawEmailService {
    pub async fn send_email(title: String, content: String, to_emal: String) -> Result<(), String> {
        let title = title;
        let body = content;

        let from_email = Env::get_smtp_username();
        let email = Message::builder()
            .from(from_email.parse().map_err(|e| format!("Invalid from address: {}", e))?)
            .to(to_emal.parse().map_err(|e| format!("Invalid to address: {}", to_emal))?)
            .subject(title)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| format!("Failed to build email: {}", e))?;

        let creds = Credentials::new(
            Env::get_smtp_username(),
            Env::get_smpt_password()
        );

        let mailer = SmtpTransport::relay(&Env::get_smtp_relay())
            .map_err(|e| format!("Failed to create SMTP transport: {}", e))?
            .credentials(creds)
            .build();

        mailer.send(&email)
            .map_err(|e| format!("Failed to send email: {}", e));

        Ok(())
    }
}