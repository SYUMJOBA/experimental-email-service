
mod raw;
mod emailhtmlbodyprovider;

use std::collections::HashMap;

use raw::RawEmailService;

pub struct EmailService;

impl EmailService {
    pub async fn send_example_email(mail_to: String, example_text: String,
        user_name: String,
        user_email: String,
        account_id: String,
        created_date: String,
        action_url: String,
        unsubscribe_url: String,
        preferences_url: String
    ) -> Result<(), String> {
        RawEmailService::send_email_html("Example Mail: from rust-littre-powered email microservice".to_string(), 
            emailhtmlbodyprovider::EmailHtmlBodyProvider::get_example_body(),
            mail_to,
            {
                let mut map = HashMap::new();
                map.insert("email_title".to_string(), "Example mail".to_string());
                map.insert("user_name".to_string(), user_name);
                map.insert("user_email".to_string(), user_email);
                map.insert("account_id".to_string(), account_id);
                map.insert("created_date".to_string(), created_date);
                map.insert("action_url".to_string(), action_url);
                map.insert("unsubscribe_url".to_string(), unsubscribe_url);
                map.insert("preferences_url".to_string(), preferences_url);
                map
            }
        ).await
    }
}