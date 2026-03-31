use lettre::{
    message::{header::ContentType, Mailbox, Message},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use std::fs;

use crate::config::settings::EmailConfig;


pub async fn send_email(
    config: &EmailConfig,
    to_email: &str,
    subject: &str,
    html_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 📄 Read HTML file
    let html_body = fs::read_to_string(html_file_path)?;

    // 📨 Build email
    let email = Message::builder()
        .from(Mailbox::new(
            Some("My App".to_string()),
            config.smtp_username.parse()?,
        ))
        .to(to_email.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_body)?;

    // 🔐 Credentials
    let creds = Credentials::new(
        config.smtp_username.clone(),
        config.smtp_password.clone(),
    );

    // 🚀 SMTP Transport
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_server)?
        .port(config.smtp_port)
        .credentials(creds)
        .build();

    // 📤 Send email
    mailer.send(email).await?;

    println!("✅ Email sent to {}", to_email);

    Ok(())
}

