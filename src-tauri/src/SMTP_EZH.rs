use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, from_str};
use sha256::digest;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};

pub async fn send_mail_vcode(mail: String, vcode: String) {
    
    let url = "https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/SMTP_Credentials/-NSRyEvgXnEMKbAGGcBZ.json".to_string();
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    let v: Value = serde_json::from_str(&r1).unwrap();

    let mut creds = Credentials::new(
        String::from(v["user"].as_str().unwrap()),
        String::from(v["pass"].as_str().unwrap()),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let mut msg_html = String::from("");
    msg_html.push_str(r#"<font color="red" style="" size="6"><b>"#);
    msg_html.push_str(&vcode);
    msg_html.push_str(r#"</b></font>"#);
    msg_html.push_str(" is your Verification Code for EZHire UAUTH service.");

    let mut msg = vcode;
    msg.push_str("is your Verification Code for EZHire UAUTH service.");

    let email = Message::builder()
        .from("noreply.ezhire@gmail.com".parse().unwrap())
        .reply_to(mail.parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("EZHire Verification Code")
        .multipart(MultiPart::alternative_plain_html(
            String::from(msg),
            String::from(msg_html),
        ))
        .unwrap();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

pub async fn send_mail_2FA(mail: String, vcode: String) {
    
    let url = "https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/SMTP_Credentials/-NSRyEvgXnEMKbAGGcBZ.json".to_string();
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    let v: Value = serde_json::from_str(&r1).unwrap();

    let mut creds = Credentials::new(
        String::from(v["user"].as_str().unwrap()),
        String::from(v["pass"].as_str().unwrap()),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let mut msg_html = String::from("");
    msg_html.push_str(r#"<font color="red" style="" size="6"><b>"#);
    msg_html.push_str(&vcode);
    msg_html.push_str(r#"</b></font>"#);
    msg_html.push_str(" is your Verification Code for EZHire UAUTH service.");

    let mut msg = vcode;
    msg.push_str("is your 2FA Code for EZHire UAUTH service.");

    let email = Message::builder()
        .from("noreply.ezhire@gmail.com".parse().unwrap())
        .reply_to(mail.parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("EZHire Verification Code")
        .multipart(MultiPart::alternative_plain_html(
            String::from(msg),
            String::from(msg_html),
        ))
        .unwrap();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
