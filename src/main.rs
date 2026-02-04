use crate::{email::EmailService, env::Env};

mod env;
mod email;

#[tokio::main]
async fn main() -> Result<(), String> {

    Env::init();

    println!("Email configuration: ");    
    println!("username: {}", Env::get_smtp_username());
    println!("passwrod: {}", Env::get_smpt_password());
    println!("relay:    {}", Env::get_smtp_relay());
    println!("key:      {}", Env::get_super_key());
    println!("");


    EmailService::send_example_email(
        "<ENTER YOUR EXAMPLE TARGET EMAIL>".to_string(), 
        "Here is some example text for you feed on! :D".to_string(),
        "Some user name".to_string(),
        "<ENTER YOUR EXAMPLE TARGET EMAIL>".to_string(),
        "007".to_string(),
        "now I guess".to_string(),
        "<EXAMPLE SUBSCRIBE LINK>".to_string(),
        "<EXAMPLE UNSUBSCRIBE LINK>".to_string(),
        "<EXAMPLE PREFERENCES LINK>".to_string()
    ).await?;

    println!("email sent!");

    Ok(())
}
