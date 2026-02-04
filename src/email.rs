
mod raw;

use raw::RawEmailService;

pub struct EmailService;

impl EmailService {
    pub async fn send_example_email(mail_to: String, example_text: String) -> Result<(), String> {
        RawEmailService::send_email("Example Mail: from rust-littre-powered email microservice".to_string(), 
            "This is an example piece of mail coming from the email microservice".to_string(),
            mail_to
        ).await
    }
}