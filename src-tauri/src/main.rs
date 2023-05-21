#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod LogReg;
pub mod SMTP_EZH;
pub mod PDF_EZH;
pub mod CloudStorage_EZH;
pub mod Single_Column;
mod NER_EZH;
use CloudStorage_EZH::{return_dir_count, return_dir_contents};
use NER_EZH::*;
use serde::{Deserialize, Serialize};
use reqwest::{header::{USER_AGENT, HeaderMap, HeaderValue, AUTHORIZATION}, Client};
use regex::Regex;
use serde_json::{json, Value};

use crate::CloudStorage_EZH::generate_PDF_queue_report;
use crate::CloudStorage_EZH::jpeg2txt;

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

// use std::collections::HashMap;
// use crate::NER_EZH::{Git_Repo_Info, experience};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Candidate2 {
//     // Personal Info
//     name: String,
//     phone: String,

//     // Crux
//     skills: HashMap<String, i32>,
//     institutes: Vec<String>,
//     workexp: experience,
//     git_repos: Vec<Git_Repo_Info>,

//     // Socials
//     github: String,
//     git_username: String,
//     linkedin: String,

//     // Metadata
//     template: String,
//     tokens: Vec<String>,
// }
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


#[tokio::main]
async fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![LogReg::create_user,LogReg::login_user, LogReg::match_vcode, LogReg::match_2fa,LogReg::remember_me_token, 
             CloudStorage_EZH::write_file,
            //  CloudStorage_EZH::test,
             CloudStorage_EZH::jpeg2txt,
             CloudStorage_EZH::destroy_directory,
             CloudStorage_EZH::create_dataset,
             NER_EZH::ner_caller,
             ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}




fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}

async fn perform_batch_process(account:String,){



}

















// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

// pub mod LogReg;
// pub mod SMTP_EZH;
// pub mod PDF_EZH;
// pub mod CloudStorage_EZH;
// pub mod Single_Column;
// mod NER_EZH;
// // use CloudStorage_EZH::{, generate_dir};
// use serde::{Deserialize, Serialize};
// use reqwest::{header::{USER_AGENT, HeaderMap, HeaderValue, AUTHORIZATION}, Client};
// use regex::Regex;
// use serde_json::{json, Value};

// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     email: String,
//     name: String,
//     password: String,
//     sub_type: i32, //0 = free, 1 = paid, 2 =pro
//     twofa: bool,
//     verified: bool,
//     datasets: String,
// }

// struct filepath {
//     path: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Response {
//     name: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Confirmation {
//     value: bool,
//     response: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Confirmation_Login {
//     value: bool,
//     response: String,
//     TWO_FA: bool,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct User_Fetch {
//     value: bool,
//     user: User,
// }


// #[derive(Serialize, Deserialize, Debug)]
// struct SMTP_CRED{
//     user:String,
//     pass:String,
// }
// struct User_Login {
//     value: bool,
//     twofa: bool,
//     hash: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct V_Code{

//     email:String,
//     vcode:String

// }

// #[derive(Serialize, Deserialize)]
// struct test {
//     message : String,
// }

// #[tokio::main]
// async fn main() {

//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![LogReg::create_user,LogReg::login_user, LogReg::match_vcode, LogReg::match_2fa,LogReg::remember_me_token, 
//              CloudStorage_EZH::write_file,CloudStorage_EZH::test,
//              CloudStorage_EZH::jpeg2txt,
//              ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");


// }

// fn string_to_response(s: &str) -> Response {
//     serde_json::from_str(s).unwrap()
// }

// fn string_to_user(s: &str) -> User {
//     serde_json::from_str(s).unwrap()
// }

// async fn perform_batch_process(account:String,){

// }
