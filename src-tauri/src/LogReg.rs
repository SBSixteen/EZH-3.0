#![allow(warnings)]

use rand::Rng;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use sha256::digest;

use crate::SMTP_EZH;


//Session Token -> username + current time ->256 -> unique hash // Session Tokens, Remember tokens
//Global User : monkaw@gmail.com

#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    name: String,
    password: String,
    sub_type: i32, //0 = free, 1 = paid, 2 =pro
    two_fa: bool,
    verified: bool,
    datasets: Vec<String>,
    blocked:bool,
}

// struct Table{   
//     columns: Vec<Strings>,
//     cells: Vec<Vec<String>>, //2D array
//     //Name, Institute, Contact, Skills,  //Skills.append(<Skill>)
// }

#[derive(Serialize, Deserialize, Debug)]
struct ConfirmationLogin {
    value: bool,
    response: String,
    two_fa: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwoFACode{
    account:String,
    code:String,
    attempts:i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct response{
    proceed:bool,
    response:String
}

#[derive(Serialize, Deserialize, Debug)]
struct RM_Token{   
    email:String,
    token:String,
}

async fn url_generator(email:String) -> String{

    let mut url = String::from("");

    url.push_str("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/Users/");
    url.push_str(&sha256::digest(email.to_owned()));
    url.push_str(".json");

    return url;

}

async fn url_generator_vcodes(email:String) -> String{

    let mut url = String::from("");

    url.push_str("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/V_Codes/");
    url.push_str(&sha256::digest(email.to_owned()));
    url.push_str(".json");

    return url;

}

async fn url_generator_2fa(email:String) -> String{

    let mut url = String::from("");

    url.push_str("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/2FA_Codes/");
    url.push_str(&sha256::digest(email.to_owned()));
    url.push_str(".json");

    return url;

}

async fn url_generator_rm(email:String) -> String{

    let mut url = String::from("");

    url.push_str("https://ezhire-c4044-default-rtdb.asia-southeast1.firebasedatabase.app/RM_Token/");
    url.push_str(&sha256::digest(email.to_owned()));
    url.push_str(".json");

    return url;

}

async fn json_patch_generator(var:String, value:String, dtype:String) -> String{

    let mut dynstr = String::from("{\"");
    dynstr.push_str(&var);
    
    match dtype.to_lowercase().as_str(){

        "string" =>{
            dynstr.push_str("\":\"");
            dynstr.push_str(&value);
            dynstr.push_str("\"}");
        }

        _ =>{
            dynstr.push_str("\":");
            dynstr.push_str(&value);
            dynstr.push_str("}");
        }

    }


    return dynstr;

}

async fn randomcode(length:i32) ->String{

    let charset = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");

    let mut vcode = String::from("");

    for _i in 0..length as usize{

        let num = ((rand::thread_rng().gen_range(0..999))%charset.len()) as usize;
        let letter = charset.chars().nth(num).unwrap();
        vcode.push(letter);

    }

    return vcode

}

#[tauri::command]
pub async fn create_user(email:String, name:String, password:String) -> String{

    let x = User{

        name,
        email:email.clone().to_lowercase(),
        password: sha256::digest(password),
        sub_type:0,
        datasets:Vec::new(),
        two_fa:false,
        verified:false,
        blocked:false,
    };

    let client = Client::builder().build().unwrap();

    let url = url_generator(email.clone()).await;

    let r1 = client.get(&url).send().await.unwrap();

    let result = r1.text().await.unwrap(); 

    let mut answer = ConfirmationLogin{
        value:true,
        response:"".to_owned(),
        two_fa:false,
    };

    if &result == "null"{
        let _response = client.put(&url).body(serde_json::to_string(&x).unwrap().replace("\\", "")).send().await.unwrap();
        generate_vcode(email.clone()).await;
        answer.response = String::from("Success!")
    }else{
        println!("Account Already exists in Database");
        answer.response = "Account already exists in Database!".to_owned();
        answer.value = false;
    }

    return serde_json::to_string(&answer).unwrap();

}

#[tauri::command]
pub async fn login_user(email:String, password:String) -> String{

    let url = url_generator(email.clone().to_lowercase()).await;
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    let mut answer = ConfirmationLogin{
        value:false,
        response:"Account DNE".to_owned(),
        two_fa:false,
    };

    match r1.as_str(){

        "null" =>{
        }

        _=>{
            let mut str = String::from(""); 
            let pass = sha256::digest(password);
            let v: Value = serde_json::from_str(&r1).unwrap();

            if v["blocked"].as_bool().unwrap(){
                str.push_str("Account is Blocked!");
            }
            else{            
                
                if v["password"].as_str().unwrap() == &pass{
                    answer.value = true;
                    str.push_str("Login Success!");

                        if v["two_fa"].as_bool().unwrap(){

                            answer.two_fa = true;
                            generate_2fa(email.clone()).await;
                        
                        }
                        else{
                            str.push_str(" No 2FA!"); 
                        }
                    }
                    else{
                str.push_str("Bad Credentials!");
            }
        }

            answer.response = str;
        }
    }

    return serde_json::to_string(&answer).unwrap();

}

pub async fn generate_vcode(email:String){

    let url = url_generator_vcodes(email.clone()).await;
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" =>{
            let answer = TwoFACode{
                account: email.clone(),
                code:randomcode(7).await,
                attempts:3,
            };
            let _response = client.put(&url).body(serde_json::to_string(&answer).unwrap().replace("\\", "")).send().await.unwrap();
            SMTP_EZH::send_mail_vcode(email.clone(), answer.code).await;
        }

        _ =>{
            let vcode = randomcode(7).await;
            
            let mut dynstr = String::from("{\"");
            dynstr.push_str("code\":\"");
            dynstr.push_str(&vcode);
            dynstr.push_str("\"}");

            let _r2 = client.patch(&url).body(dynstr).send().await.unwrap();
            SMTP_EZH::send_mail_vcode(email.clone(), vcode).await;
        }
    }
}

pub async fn generate_2fa(email:String){

    let client = Client::builder().build().unwrap();

    let url = url_generator(email.clone()).await;
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap().replace("\\", "");
    let v: Value = serde_json::from_str(&r1).unwrap();
    
    if !v["two_fa"].as_bool().unwrap(){
        println!("Account requires 2FA enabled to generate codes");
        return;
    }

    let url = url_generator_2fa(email.clone()).await;
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" =>{
            let answer = TwoFACode{
                account: email.clone(),
                code:randomcode(4).await,
                attempts:3
            };
            let _response = client.put(&url).body(serde_json::to_string(&answer).unwrap().replace("\\", "")).send().await.unwrap();
            SMTP_EZH::send_mail_2FA(email.clone(), answer.code.clone()).await;
        }

        _ =>{
            
            let vcode = randomcode(4).await;
            let dynstr = json_patch_generator("code".to_string(), vcode.clone(), "string".to_owned()).await;
            let _r2 = client.patch(&url).body(dynstr).send().await.unwrap();
            SMTP_EZH::send_mail_2FA(email.clone(), vcode.clone()).await;
        }
    }


}

pub async fn toggle_verified(email:String){

    let url = url_generator(email.clone()).await;

    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" => {
            println!("Account DNE");
            return;
        }
        _=>{
            let v: Value = serde_json::from_str(&r1).unwrap();

            if v["verified"].as_bool().unwrap(){

                let _r2 = client.patch(&url).body("{\"verified\":false}").send().await.unwrap();
                println!("Account Found! Now Unverified");

            }else{
                let _r2 = client.patch(&url).body("{\"verified\":true}").send().await.unwrap();
                println!("Account Found! Now Verified");
            }

        }

    }
}

pub async fn toggle_blocked(email:String){

    let url = url_generator(email.clone()).await;

    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" => {
            println!("Account DNE");
            return;
        }
        _=>{
            let v: Value = serde_json::from_str(&r1).unwrap();

            if v["blocked"].as_bool().unwrap(){

                let _r2 = client.patch(&url).body("{\"blocked\":false}").send().await.unwrap();
                println!("Account Found! Account is now unblocked");

            }else{
                let _r2 = client.patch(&url).body("{\"blocked\":true}").send().await.unwrap();
                println!("Account Found! Now Verified");
            }

        }

    }

}

pub async fn toggle_2fa(email:String){

    let url = url_generator(email.clone()).await;

    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" => {
            println!("Account DNE");
            return;
        }
        _=>{
            let v: Value = serde_json::from_str(&r1).unwrap();

            if v["two_fa"].as_bool().unwrap(){

                let _r2 = client.patch(&url).body("{\"two_fa\":false}").send().await.unwrap();
                println!("Account Found! Now ships without 2FA");

            }else{
                let _r2 = client.patch(&url).body("{\"two_fa\":true}").send().await.unwrap();
                println!("Account Found! Now ships with 2FA");
            }

        }

    }

}

#[tauri::command]
pub async fn user_exist(email:String) -> bool{

    let url = url_generator(email.clone()).await;
    let client = Client::builder().build().unwrap();

    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();
    
    match r1.as_str(){
        "null"=>{
            println!("Account DNE");
            return false;
        }
        _=>{
            println!("Account found!");
            return true;
        }
    }
}

#[tauri::command]
pub async fn match_2fa(email:String, attempt:String) -> String{

    let url = url_generator_2fa(email.clone()).await;
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    let mut answer = response{

        proceed:false,
        response: String::from("")
    };

    if &r1 == "null"{
        answer.response = String::from("2FA code for account DNE");
        return serde_json::to_string(&answer).unwrap();
    }

    let v: Value = serde_json::from_str(&r1).unwrap();
    if v["code"].as_str().unwrap() == &attempt{
        answer.response = String::from("Correct!");
        answer.proceed = true;
        client.delete(&url).send().await.unwrap();
        return serde_json::to_string(&answer).unwrap();
    }else{
        let mut tries = v["attempts"].as_i64().unwrap();
        tries-=1;

        if tries == 0{
            let new_url = url_generator(email.clone()).await;
            let patch = json_patch_generator("blocked".to_string(), "true".to_string(), "boolean".to_string()).await;
            client.patch(&new_url).body(patch).send().await.unwrap();
            client.delete(&url).send().await.unwrap();
            answer.response = String::from(format!("Too many incorrect attempts, account is blocked"));
            return serde_json::to_string(&answer).unwrap();
        }

        let patch = json_patch_generator(String::from("attempts"), tries.to_string(), String::from("integer")).await;
        client.patch(&url).body(patch).send().await.unwrap();
        answer.response = String::from(format!("Incorrect Attempt, {} tries remaining!", tries));
        answer.proceed = false;
        return serde_json::to_string(&answer).unwrap();
    }



}

#[tauri::command]
pub async fn match_vcode(email:String, attempt:String) -> String{

    let url = url_generator_vcodes(email.clone()).await;
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();
    let new_url = url_generator(email.clone()).await;

    let mut answer = response{
        proceed:false,
        response:String::from("Correct! Account is now verified!")
    };

    if &r1 == "null"{ 
        answer.response = String::from("Account does not have a valid Verification Code");
        return serde_json::to_string(&answer).unwrap();
    }

    let v: Value = serde_json::from_str(&r1).unwrap();

    if v["code"].as_str().unwrap() == &attempt{
        println!("Correct!");
        client.delete(&url).send().await.unwrap();
        let patch = json_patch_generator("verified".to_string(), "true".to_string(), "boolean".to_string()).await;
        client.patch(&new_url).body(patch).send().await.unwrap();

        answer.proceed = true;

        return serde_json::to_string(&answer).unwrap();

    }else{
        println!("Incorrect!");
        let mut tries = v["attempts"].as_i64().unwrap();
        tries-=1;

        if tries == 0{

            client.delete(&new_url).send().await.unwrap();
            client.delete(&url).send().await.unwrap();
            answer.response = String::from("Your account has been deleted due to failure to type in a valid Verification Code");
            return serde_json::to_string(&answer).unwrap();
        }

        let patch = json_patch_generator(String::from("attempts"), tries.to_string(), String::from("integer")).await;
        client.patch(&url).body(patch).send().await.unwrap();

        answer.response = format!("Incorrect Attempt, Tries Remaining = {}", tries).to_string();

        return serde_json::to_string(&answer).unwrap();
    }

}

pub async fn match_password(email:String, password:String) -> response{

    let url = url_generator(email).await;

    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" => {
            return response{
                proceed:false,
                response:String::from("Account DNE")
            }
        }
        _  =>{
            let v: Value = serde_json::from_str(&r1).unwrap();
            let attempt = sha256::digest(password);

            let target = v["password"].as_str().unwrap(); 

            if target == &attempt{
                return response{
                    proceed:true,
                    response:String::from("Passwords Match!")
                }
            }else{
                return response{
                    proceed:false,
                    response:String::from("Passwords do not match!")
                }
            }
        }

    }

}

pub async fn update_password(email:String, password:String) -> response{

    let url = url_generator(email).await;

    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" => {
            return response{
                proceed:false,
                response:String::from("Account DNE")
            }
        }
        _  =>{

            let dynstr = json_patch_generator(String::from("password"), password, String::from("String")).await;
            let _r2 = client.patch(&url).body(dynstr).send().await.unwrap();

            return response{
                proceed:true,
                response:String::from("Password has been updated!")
            }

        }

    }

}

pub async fn isBlocked(email:String)-> response{

    let url = url_generator(email).await;

    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    match r1.as_str(){

        "null" => {
            return response{
                proceed:false,
                response:String::from("Account DNE")
            }
        }
        _  =>{

            let v: Value = serde_json::from_str(&r1).unwrap();
            let target = v["blocked"].as_bool().unwrap(); 

            return response{
                proceed:target,
                response: String::from("Fetch successful.")
            }
        }

    }

}

#[tauri::command]
pub async fn remember_me_token(email:String) -> String{

    let temp = format!("{} - {}", &email, chrono::Utc::now().to_string());

    let url = url_generator_rm(email.clone()).await;
    let client = Client::builder().build().unwrap();
    let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();

    let mut answer = RM_Token{
        email:email.clone(),
        token:String::new(),
    };

    println!("Generating token for remember me");

    match r1.as_str() {

        "null"=>{
            let xx = digest(temp);

            answer.token = xx.clone();

            let r2= client.put(&url).body(serde_json::to_string(&answer).unwrap().replace("\\", "")).send().await.unwrap();
        }

        _=>{
            let xx = digest(temp);
            let dynstr = json_patch_generator("token".to_string(), xx.clone(), "String".to_string()).await;
            let _r2 = client.patch(&url).body(dynstr).send().await.unwrap();
            answer.token= xx.clone();

        }
    }
    return serde_json::to_string(&answer).unwrap();

}