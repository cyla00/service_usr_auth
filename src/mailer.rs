use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn user_verification_email(
    auth_email: String, 
    auth_pass: String, 
    auth_host: String, 
    user_email: String, 
    platform_name: String, 
    user_hash: String, 
    platform_host: String
) -> Result<bool, bool> {

    let email = Message::builder()
        .from(format!("{} <{}>", platform_name, auth_email).parse().unwrap())
        .to(format!("<{}>", user_email).parse().unwrap())
        .subject("Welcome to platform...")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("
            welcome, please verify your account before connecting.
            <a href='http://{platform_host}/{user_hash}'>verify here!</a>
        "))
        .unwrap();

    let creds = Credentials::new(auth_email, auth_pass);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&auth_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}
