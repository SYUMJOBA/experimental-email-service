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
        "sylvio.classico@gmail.com".to_string(), 
        "Here is some example text for you feed on! :D".to_string()
    ).await?;

    println!("email sent!");

    Ok(())
}
