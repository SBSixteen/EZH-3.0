#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize, Serialize};
use sha256::digest;
use rand::Rng;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    password: String,
    sub_type: i32,
    datasets: String,
    //Remember_Token :bool
}

struct filepath{
    path:String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation{

    value : bool,
    response : String,

}

#[derive(Serialize, Deserialize, Debug)]
struct User_Fetch{
    value:bool,
    user:User,
} 

fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![create_user])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn initConnection() -> Firebase{

  let firebase =
  Firebase::new("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/")
      .unwrap();

  return firebase;

}

#[tauri::command]
async fn create_user( mail:String, pwd:String ) ->String {

  let firebase_client = initConnection().await;
  let x = user_exist(&mail).await;

  let mut R = Confirmation{
    value:false,
    response : String::from("")
  };

  if x {
    R.response = String::from("A user with this email already exists on EZHire");
  }else{
    
    let email = mail.clone();
    send_auth_code(email);
    R.response = String::from("Successfully Registered. Please check your email for a verification code");
    R.value = true;
    let _firebase = firebase_client.at("Users");
    let user = User{
      email: mail,
      password: digest(String::from(pwd)),
      sub_type: 0,
      datasets: String::from(""),
    };
    let _users = _firebase.set::<User>(&user).await;
  }

  let g =  serde_json::to_string(&R).unwrap();
  return g.into()
  
}

fn string_to_response(s: &str) -> Response {
  serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
  serde_json::from_str(s).unwrap()
}

async fn user_exist(email : &String) -> bool{

  let users = get_users().await;

  for (key, value) in users.into_iter() {
      if value.email == *email {
          return true;
      }
  }
  return false;

}

async fn get_users() -> HashMap<String, User> {
  let firebase = initConnection().await;
  let firebase = firebase.at("Users");
  let users = firebase.get::<HashMap<String, User>>().await;

  let _users= match users{
      Ok(users) => return users,
      Err(_err) =>{
          let temp = HashMap::new();
          return temp;
      }
  };
}

fn send_auth_code(email: String){

  let num = rand::thread_rng().gen_range(1000000..9999999);

  println!("{:?} | {:?}", env::var("SMTP_Email").unwrap(),  env::var("SMTP_PASS").unwrap());

  let creds = Credentials::new(env::var("SMTP_Email").unwrap(), env::var("SMTP_PASS").unwrap());
  let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

  let mut msg = num.to_string();
  msg.push_str(" is your Verification Code for EZHire UAUTH service.");

    let email = Message::builder()
    .from("noreply.ezhire@gmail.com".parse().unwrap())
    .reply_to(email.parse().unwrap())
    .to(email.parse().unwrap())
    .subject("EZHire Verification Code")
    .header(ContentType::TEXT_PLAIN)
    .body(msg)
    .unwrap();

    match mailer.send(&email) {
      Ok(_) => println!("Email sent successfully!"),
      Err(e) => panic!("Could not send email: {:?}", e),
  }


}