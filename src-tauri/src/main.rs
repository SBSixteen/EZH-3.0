#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use firebase_rs::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::collections::HashMap;
use std::env;
#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    name: String,
    password: String,
    sub_type: i32, //0 = free, 1 = paid, 2 =pro
    twofa: bool,
    verified: bool,
    datasets: String,
    //Remember_Token :bool
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
        .invoke_handler(tauri::generate_handler![create_user, authenticate_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initConnection() -> Firebase {
    let firebase =
        Firebase::new("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/")
            .unwrap();

    return firebase;
}

#[tauri::command]
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

    let mut msg = num.to_string();
    msg.push_str(" is your Verification Code for EZHire UAUTH service.");

    let email = Message::builder()
        .from("noreply.ezhire@gmail.com".parse().unwrap())
        .reply_to(mail.parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("EZHire Verification Code")
        .header(ContentType::TEXT_PLAIN)
        .body(msg)
        .unwrap();

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
