use std::collections::HashMap;
use std::ffi::OsStr;
use firebase_rs::*;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha256::digest;
use rand::Rng;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::{env, path};
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel, Answer};
use std::path::PathBuf;
use std::io::BufWriter;
use std::fs::File;
use pdf_extract::*;
use lopdf::Document;

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
struct SMTP_CRED{
    user:String,
    pass:String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation{

    value : bool,
    response : Response,

}

#[derive(Serialize, Deserialize, Debug)]
struct User_Fetch{
    value:bool,
    user:User,
} 

#[tokio::main]
async fn main(){

    ocr_jpeg().await;
    //
}

async fn ocr_jpeg(){

    let image_path = "./FP.png";
    let base64 = image_base64::to_base64(image_path); 

    let client = Client::builder().build().unwrap();
    let mut h  = HashMap::new();
    h.insert("base64Image".to_owned(), base64);
    h.insert("language".to_owned(), "eng".to_owned());
    h.insert("OCREngine".to_owned(), "1".to_owned());

    let response = client.post("https://api.ocr.space/parse/image").header("apikey", "3d7d76351588957").form(&h).send().await.unwrap().text().await.unwrap();
    
    let parse = json::parse(&response).unwrap();

    println!("{:?}",parse["ParsedResults"][0]["ParsedText"].to_string().replace("\r\n", " "));

    //Promise => 
    //8GB
    //5.0 MB

}

async fn clearStr(){

}

async fn github_rep(){

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("SBSixteen"));

    let client = Client::builder().default_headers(headers).build().unwrap();
    let response = client.get("https://api.github.com/users/SBSixteen/repos").send().await.unwrap();

    let g = response.text().await.unwrap();

    let parse = json::parse(&g).unwrap();

    println!("Repositories [{:?}] :", parse.len());

    for  n in 0..parse.len() {
        println!("{:#} | Language => {:#} ", parse[n]["name"], parse[n]["language"]);
    }

    println!("");


    let response = client.get("https://api.github.com/users/SBSixteen/followers").send().await.unwrap();
    
    let g = response.text().await.unwrap();

    let parse = json::parse(&g).unwrap();
    println!("Followers [{:?}] :", parse.len());
    for  n in 0..parse.len() {
        println!("{:#}", parse[n]["login"]);
    }

}

async fn compute_ner() -> Vec<Vec<Answer>>{

    let blocking_task = tokio::task::spawn_blocking(|| {
        let qa_model = QuestionAnsweringModel::new(Default::default());

        let question = String::from("List all the courses");
        let context = String::from("");
        let a = qa_model.expect("REASON").predict(&[QaInput { question, context }], 1, 32);

        return a;
    });
   
   let a = blocking_task.await.unwrap();
   return a;

}

async fn initConnection() -> Firebase{

        let firebase =
        Firebase::new("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/")
            .unwrap();

        return firebase;

}

async fn create_user(firebase_client: &Firebase, user: &User, map: &HashMap<String, User>) -> Response {

    let email = &user.email;
    let x =user_exist(&email).await;

    if !x {
        let firebase = firebase_client.at("Users");
        let _users = firebase.set::<User>(&user).await;
        let R = Response {
            name: String::from("User created succesfully!"),
        };
        return R;
    }   else {
            let R = Response {
                name: String::from("User already exists!"),
            };
            return R;
    }
}

async fn get_user(email: &String) -> User_Fetch{

    let users = get_users().await;

    for (key, value) in users.into_iter(){
        if value.email == *email{
            return User_Fetch{
                value:false,
                user:value
            }
        }
    }

    return User_Fetch{
        value:false,
        user:User{
            email: String::from("-1"),
            password: String::from("-1"),
            sub_type: -1,
            datasets: String::from("-1"),
        }
    }

}

async fn get_users() -> HashMap<String, User> {
    let firebase =Firebase::new("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/").unwrap();
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

async fn get_cred(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("Users");
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn authenticate_user(e: &String, pwd: &str,) -> Confirmation {

    let map = get_users().await;

    println!("{:?}", map.len());

    let exists = user_exist(e).await;

    if !exists{
        return Confirmation{
            value:false,
            response:Response{
                name: String::from("Incorrect Password/Email")
            }
        };
    }   

    let password = digest(pwd);

    for (key, value) in map.into_iter() {
        if value.email == *e {
            if value.password == password {
                return Confirmation{
                    value:true,
                    response:Response{
                        name: String::from("Login Succesful")
                    }
                }
            } else{
                return Confirmation{
                    value:false,
                    response:Response{
                        name: String::from("Incorrect Password/Email")
                    }
                }
            }
        }
    }

    return Confirmation{
        value:false,
        response:Response{
            name: String::from("Unknown Error!")
        }
    }

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