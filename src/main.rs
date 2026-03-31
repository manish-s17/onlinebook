

mod config;
mod email;

use crate::config::settings::load_email_config;
use crate::email::smpt::send_email;

#[tokio::main]
async fn main() {
    let config = load_email_config();

    println!("{:?}", std::env::vars().collect::<Vec<_>>());

    let result = send_email(
        &config,
        "dev.manishsah@gmail.com",
        "Welcome!",
        "src/email/templates/welcome.html",
    )
    .await;

    match result {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

