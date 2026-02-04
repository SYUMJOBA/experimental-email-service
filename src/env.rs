pub struct Env;

impl Env {
    pub fn init() {
        dotenv::dotenv().ok().expect("could not init dotenv");
    }

    fn get_or_fail(key: &str) -> String {
        let os_string = std::env::var_os(key).expect(format!("could not get key {}", key).as_str());
        let p_os_string = os_string.clone();
        os_string.into_string().expect(format!("could not change string {:?} from key {} into a String", p_os_string.clone(), key).as_str())
    }

    pub fn get_smpt_password() -> String {
        Self::get_or_fail("smtp_password")
    }

    pub fn get_smtp_username() -> String {
        Self::get_or_fail("smtp_username")
    }

    pub fn get_super_key() -> String {
        Self::get_or_fail("super_key")
    }
    pub fn get_smtp_relay() -> String {
        Self::get_or_fail("smtp_relay")
    }
}