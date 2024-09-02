use lettre::message::Message;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use rand::Rng; // For generating the random 4-digit code

pub async fn send_code_to_email(email: &str, username: &str) {
    // Generate a random 4-digit code
    let code = generate_code();

    // Create the email message
    let email_message = Message::builder()
        .from("Your App <carlos@mail.arcade-x.tech>".parse().unwrap())  // Replace with your "from" address
        .to(email.parse().unwrap())
        .subject("Your Login Code")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(create_email_body(username, &code))
        .unwrap();

    // Replace with your SMTP credentials
    let creds = Credentials::new("your_username".to_string(), "your_password".to_string());

    // Create the SMTP client and send the email
    let mailer = SmtpTransport::relay("mail.arcade-x.tech")  // Your domain's mail server
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email_message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}

fn create_email_body(username: &str, code: &str) -> String {
    format!(
        r#"
        <html>
        <body style="font-family: Arial, sans-serif;">
            <div style="background-color: #f7f7f7; padding: 20px;">
                <h1 style="color: #333;">Hello, {username}!</h1>
                <p>Your login code is:</p>
                <div style="font-size: 24px; font-weight: bold; margin: 10px 0; color: #ff6600;">
                    {code}
                </div>
                <p>This code is valid for 1 minute.</p>
            </div>
        </body>
        </html>
        "#,
        username = username,
        code = code,
    )
}

fn generate_code() -> String {
    let code: u32 = rand::thread_rng().gen_range(1000..9999);
    code.to_string()
}