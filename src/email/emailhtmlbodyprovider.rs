use std::fs;
use std::path::Path;


pub struct EmailHtmlBodyProvider;

static HTML_PATH_PREFIX: &'static str = "email_bodies";

impl EmailHtmlBodyProvider {
    fn get_body(body_name: &str) -> Option<String> {
        let file_path = Path::new(HTML_PATH_PREFIX).join(format!("{}.html", body_name));
        match fs::read_to_string(file_path) {
            Ok(content) => return Some(content),
            Err(_) => return None
        }
    }

    pub fn get_example_body() -> String {
        let email_name= "example";
        Self::get_body(email_name).expect(format!("[EMAIL ERROR]: Could not get file {}", email_name).as_str())
    }
}