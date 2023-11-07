use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{ header, MultiPart, SinglePart };

pub async fn user_verification_email(
    auth_email: String, 
    auth_pass: String, 
    auth_host: String, 
    user_email: String, 
    platform_name: String, 
    user_hash: String, 
    platform_host: String
) -> Result<bool, bool> {

    let url = format!("<a href='http://{}/{}'>Verify here</a>", platform_host, user_hash);
    let message = r#"
        <p>welcome, please verify your account before connecting.</p>
        "#.to_string() + url.as_str();

    let email = Message::builder()
        .from(format!("{} <{}>", platform_name, auth_email).parse().unwrap())
        .to(format!("<{}>", user_email).parse().unwrap())
        .subject("Welcome to platform...")
        .header(ContentType::TEXT_PLAIN)
        .multipart(MultiPart::alternative().singlepart(SinglePart::builder().header(header::ContentType::TEXT_HTML).body(message.to_string())))
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
