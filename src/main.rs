use crate::env::Env;

mod env;

fn main() {

    Env::init();
    
    println!("{}", Env::get_smpt_password());
    println!("{}", Env::get_smtp_username());
    println!("{}", Env::get_super_key());

    println!("Hello, world!");
}
