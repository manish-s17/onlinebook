use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: u16,
}

pub fn load_email_config() -> EmailConfig {
    dotenv::dotenv().ok();
    envy::from_env().expect("Failed to load env config")
}

