#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod LogReg;
pub mod SMTP_EZH;
pub mod PDF_EZH;
pub mod CloudStorage_EZH;
use serde::{Deserialize, Serialize};
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
        .invoke_handler(tauri::generate_handler![LogReg::create_user, LogReg::login_user, LogReg::match_vcode, LogReg::match_2fa, LogReg::remember_me_token])
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

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
