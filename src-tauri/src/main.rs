#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(warnings)]

pub mod LogReg;
pub mod SMTP_EZH;
use firebase_rs::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::collections::HashMap;
use std::env;
use reqwest::Client;
use serde_json::{Value, json};
#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    name: String,
    password: String,
    sub_type: i32, //0 = free, 1 = paid, 2 =pro
    twofa: bool,
    verified: bool,
    datasets: String,
}

struct filepath {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation {
    value: bool,
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation_Login {
    value: bool,
    response: String,
    TWO_FA: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct User_Fetch {
    value: bool,
    user: User,
}


#[derive(Serialize, Deserialize, Debug)]
struct SMTP_CRED{
    user:String,
    pass:String,
}
struct User_Login {
    value: bool,
    twofa: bool,
    hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct V_Code{

    email:String,
    vcode:String

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![LogReg::create_user, LogReg::login_user, user_exist_w, LogReg::match_vcode, LogReg::match_2fa, LogReg::remember_me_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    //LogReg::toggle_2fa(String::from("nabeelmirza80@gmail.com")).await;
    //println!("{}", LogReg::create_user(String::from("nabeelmirza80@gmail.com"), String::from("Nabeel Mirza"), String::from("12345678")).await);
    //LogReg::create_user(String::from("nabeelmirza79@gmail.com"), String::from("Nabeel Mirza"), String::from("12345678")).await;
    //LogReg::toggle_2fa(String::from("nabeelmirza79@gmail.com")).await;
    //LogReg::generate_2fa(String::from("nabeelmirza80@gmail.com")).await;
    //LogReg::toggle_blocked(String::from("nabeelmirza79@gmail.com")).await;
    //println!("{}",login_user(String::from("nabeelmirza79@gmail.com"), String::from("12345678")).await);
    //match_2fa(String::from("nabeelmirza79@gmail.com"), String::from("BksQ")).await;
    //match_vcode(String::from("nabeelmirza79@gmail.com"), String::from("UOQYsuB")).await;

}

async fn info(){
    
    let url = "https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/SMTP_Credentials/-NSRyEvgXnEMKbAGGcBZ".to_string();
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    let v: Value = serde_json::from_str(&r1).unwrap();
    println!("{}", v);

}

async fn initConnection() -> Firebase {
    let firebase =
        Firebase::new("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/")
            .unwrap();

    return firebase;
}

async fn create_user(mail: String, username: String, pwd: String) -> String {
    let firebase_client = initConnection().await;
    let x = user_exist(&mail).await;

    let mut R = Confirmation {
        value: false,
        response: String::from(""),
    };

    if x.value {
        R.response = String::from("A user with this email already exists on EZHire");
    } else {
        let email = mail.clone();
        send_auth_code(email).await;
        R.response = String::from(
            "Successfully Registered. Please check your email for a verification code",
        );
        R.value = true;
        let _firebase = firebase_client.at("Users");
        let user = User {
            email: mail,
            name: username,
            password: digest(String::from(pwd)),
            sub_type: 0,
            twofa: false,
            verified: false,
            datasets: String::from(""),
        };
        let _users = _firebase.set::<User>(&user).await;
    }

    let g = serde_json::to_string(&R).unwrap();
    return g.into();
}

#[tauri::command]
async fn user_exist_w(email:String)->bool{

    return LogReg::user_exist(email).await;

}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}

async fn user_exist(email: &String) -> User_Login {
    let users = get_users().await;

    for (key, value) in users.into_iter() {
        if value.email == *email {
            if value.twofa {
                return User_Login {
                    value: true,
                    twofa: true,
                    hash: value.password,
                };
            } else {
                return User_Login {
                    value: true,
                    twofa: false,
                    hash: value.password,
                };
            }
        }
    }
    return User_Login {
        value: false,
        twofa: false,
        hash: String::from("USER DNE"),
    };
}

async fn get_users() -> HashMap<String, User> {
    let firebase = initConnection().await;
    let firebase = firebase.at("Users");
    let users = firebase.get::<HashMap<String, User>>().await;

    let _users = match users {
        Ok(users) => return users,
        Err(_err) => {
            let temp = HashMap::new();
            return temp;
        }
    };
}

async fn send_auth_code(mail: String) {
    let num = rand::thread_rng().gen_range(1000000..9999999);

    let firebase = initConnection().await;
    let _firebase = firebase.at("V_Codes");

    let g = V_Code{

        email:mail.clone(),
        vcode:num.to_string()

    };

    let _send = _firebase.set::<V_Code>(&g).await;

    let g = firebase.at("SMTP_Credentials");
    let map = g.get::<HashMap<String, SMTP_CRED>>().await.unwrap();

    let mut creds = Credentials::new(
        String::from("123"),
        String::from("123"),
    );

    for (key,value) in map.into_iter() {
        
        creds = Credentials::new(
            String::from(value.user),
            String::from(value.pass),
        )

    }

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let mut msg_html = String::from("");
    msg_html.push_str(  r#"<font color="red" style="" size="6"><b>"#);
    msg_html.push_str(&num.to_string());
    msg_html.push_str(  r#"</b></font>"#);
    msg_html.push_str(" is your Verification Code for EZHire UAUTH service.");

    let mut msg = num.to_string();
    msg.push_str( "is your Verification Code for EZHire UAUTH service.");

    let email = Message::builder()
        .from("noreply.ezhire@gmail.com".parse().unwrap())
        .reply_to(mail.parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("EZHire Verification Code")
        .multipart(MultiPart::alternative_plain_html(
            String::from(msg),
            String::from(msg_html),
        ),).unwrap();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

async fn get_cred(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("Users");
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

#[tauri::command]
async fn authenticate_user(e: String, pwd: String) -> String {
    let exists = user_exist(&e).await;

    let password = digest(pwd);

    if exists.hash == String::from("USER DNE") {
        let b = Confirmation_Login {
            value: false,
            response: String::from("The current email is not registered on EZHire"),
            TWO_FA: false,
        };

        return serde_json::to_string(&b).unwrap().into()
        
    } else {
        if exists.twofa {
            let b = Confirmation_Login {
                value: true,
                response: String::from("Login Success!"),
                TWO_FA: true,
            };

            return serde_json::to_string(&b).unwrap().into()
        } else {
            let b = Confirmation_Login {
                value: true,
                response: String::from("Login Success! But no 2FA"),
                TWO_FA: false,
            };
            return serde_json::to_string(&b).unwrap().into()
        }
    }
}
