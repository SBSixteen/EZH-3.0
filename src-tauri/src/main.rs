#![allow(dead_code, unused_variables)]

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

    // //Generate your cloud storage directories
    // generate_dir(String::from("monkaw.gmail.com")).await;

    //H:/DDR4/EZHIRE/src
    //C:/PROGRAM/RZH/src

    // ./ = C:/PROGRAM/RZH/src
    // ./ = H:/DDR4/EZHIRE/src

    //Start PDF Processing. <Put PDF in TEMP_PDF> and set to true

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![LogReg::create_user, LogReg::login_user, LogReg::match_vcode, LogReg::match_2fa, LogReg::remember_me_token, LogReg::generate_changepass_code, LogReg::update_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


    // let owner = String::from("nabeelmirza79@gmail.com");

    // generate_PDF_queue_report(owner.clone(), false).await;

    // let mut headers = HeaderMap::new();
    // headers.insert(USER_AGENT, HeaderValue::from_static("SBSixteen"));
    // headers.insert(AUTHORIZATION,HeaderValue::from_static("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") );

    // let client = Client::builder().default_headers(headers).build().unwrap();

    //let temp = NER_EZH::Git_Repo_Info::automatic(&client, String::from("SBSixteen"), String::from("Academia")).await;
    // let r1 = client.get("https://api.github.com/users/SBSixteen/repos").send().await.unwrap().text().await.unwrap();

    // let value:Value = serde_json::from_str(&r1).unwrap();

    // println!("{}", &value.is_array());

    // let temp = value.as_array().unwrap().len();

    // for i in 0..temp{
    //     println!("{}",value[i]["name"]);
    // }

    // let mut candidates: Vec<NER_EZH::Candidate> = Vec::new();

    // for i in 0..return_dir_count(owner.clone(), "TEMP_TXT".to_string()).await{
    //     candidates.push(Candidate::new().await);
    //     println!("Candidate #{} is generated.", (i+1));
    // }

    // let temp = return_dir_contents(owner.clone(), "TEMP_TXT".to_string());

    // let contexts = generate_contexts(owner.clone(), temp.clone()).await;

    // let mut temple = Template::new();
    // temple.generate_from_default("languages.txt".to_string());
    // temple.change_name("Junior Software Developer".to_string());

    // candidates[6].set_name(temp[6].clone());
    // println!("Name => {:?}", candidates[6].get_name());
    // candidates[6].generate_tokens(contexts[6].clone()).await;
    // candidates[6].generate_skills(&temple).await;
    // candidates[6].clear_tokens();

    // for i in 0..candidates.len(){
    //     candidates[i].set_name(temp[i].clone());
    //     println!("Name => {:?}", candidates[i].get_name());
    //     candidates[i].generate_tokens(contexts[i].clone()).await;
    //     candidates[i].generate_skills(&temple).await;
    //     //candidates[i].clear_tokens();
    // }

    //println!("{:#?}", candidates[6]);

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
