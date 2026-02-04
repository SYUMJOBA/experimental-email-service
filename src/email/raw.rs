
use std::collections::HashMap;

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
            .from(from_email.parse().map_err(|e| format!("[EMAIL ERROR]: Invalid from address: {}", e))?)
            .to(to_emal.parse().map_err(|_| format!("[EMAIL ERROR]: Invalid to address: {}", to_emal))?)
            .subject(title)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| format!("[EMAIL ERROR]: Failed to build email: {}", e))?;

        let creds = Credentials::new(
            Env::get_smtp_username(),
            Env::get_smpt_password()
        );

        let mailer = SmtpTransport::relay(&Env::get_smtp_relay())
            .map_err(|e| format!("[EMAIL ERROR]: Failed to create SMTP transport: {}", e))?
            .credentials(creds)
            .build();

        mailer.send(&email)
            .map_err(|e| format!("[EMAIL ERROR]: Failed to send email: {}", e))?;

        Ok(())
    }

    /// Send HTML email (for styled templates)
    pub async fn send_email_html(title: String, html_content: String, to_email: String, mail_contents: HashMap<String, String>) -> Result<(), String> {
        let from_email = Env::get_smtp_username();

        // Process template: replace {{key}} blocks with values from mail_contents
        let rendered_html = Self::render_template(&html_content, &mail_contents)?;

        let email = Message::builder()
            .from(from_email.parse().map_err(|e| format!("[EMAIL ERROR]: Invalid from address: {}", e))?)
            .to(to_email.parse().map_err(|_| format!("[EMAIL ERROR]: Invalid to address: {}", to_email))?)
            .subject(title)
            .header(ContentType::TEXT_HTML)
            .body(rendered_html)
            .map_err(|e| format!("[EMAIL ERROR]: Failed to build email: {}", e))?;

        let creds = Credentials::new(
            Env::get_smtp_username(),
            Env::get_smpt_password()
        );

        let mailer = SmtpTransport::relay(&Env::get_smtp_relay())
            .map_err(|e| format!("[EMAIL ERROR]: Failed to create SMTP transport: {}", e))?
            .credentials(creds)
            .build();

        mailer.send(&email)
            .map_err(|e| format!("[EMAIL ERROR]: Failed to send email: {}", e))?;

        Ok(())
    }

    /// Render template by replacing {{key}} blocks with values from the HashMap
    fn render_template(template: &str, variables: &HashMap<String, String>) -> Result<String, String> {
        let mut result = template.to_string();
        let mut missing_keys = Vec::new();

        // Scan for all {{...}} blocks and replace them
        let mut chars = template.chars().peekable();
        let mut pos = 0;

        while pos < template.len() {
            if let Some(start_idx) = template[pos..].find("{{") {
                let absolute_start = pos + start_idx;
                
                // Find the closing }}
                if let Some(end_idx) = template[absolute_start..].find("}}") {
                    let absolute_end = absolute_start + end_idx;
                    
                    // Extract the key between {{ and }}
                    let key_start = absolute_start + 2;
                    let key = template[key_start..absolute_end].trim();
                    
                    // Check if we have a value for this key
                    if let Some(value) = variables.get(key) {
                        let placeholder = format!("{{{{{}}}}}", key);
                        result = result.replace(&placeholder, value);
                    } else {
                        // Track missing keys for error reporting
                        if !missing_keys.contains(&key.to_string()) {
                            missing_keys.push(key.to_string());
                        }
                    }
                    
                    pos = absolute_end + 2;
                } else {
                    // No closing brace found, skip this
                    pos = absolute_start + 2;
                }
            } else {
                // No more {{ blocks found
                break;
            }
        }

        // Report missing keys as a warning or error
        if !missing_keys.is_empty() {
            return Err(format!(
                "[EMAIL ERROR]: Missing template variables: {}. Provide values for all {{{{key}}}} blocks.",
                missing_keys.join(", ")
            ));
        }

        Ok(result)
    }
}