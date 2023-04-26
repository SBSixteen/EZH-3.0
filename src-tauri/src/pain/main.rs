mod LogReg;
mod Subscriptions;
use std::collections::HashMap;
use LogReg::{login_user, match_2fa, generate_2fa, toggle_2fa, match_vcode};
use firebase_rs::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use pdfium_render::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha256::{digest};
use lettre::{Message, SmtpTransport, Transport};
use std::{fs};
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel, Answer};
use std::io::*;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    name: String,
    password: String,
    sub_type: i32, //0 = free, 1 = paid, 2 =pro
    twofa: bool,
    verified: bool,
    datasets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct candidate{

filename:String,
name:String,
email:String,
Git_User:String,
LinkedIn_URL:String,
Phone:String,
git_repos:Vec<String>,

}

#[derive(Serialize, Deserialize, Debug)]
struct candidateData{

 tokens:Vec<String>,

}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation_Login {
    value: bool,
    response: String,
    TWO_FA: bool,
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

    let client = Client::builder().build().unwrap();
    perform().await;

    //LogReg::toggle_2fa(String::from("nabeelmirza80@gmail.com")).await;
    //println!("{}", LogReg::create_user(String::from("nabeelmirza80@gmail.com"), String::from("Nabeel Mirza"), String::from("12345678")).await);
    //LogReg::create_user(String::from("monkaw@gmail.com"), String::from("Ali Haider"), String::from("12345678")).await;
    //LogReg::toggle_2fa(String::from("monkaw@gmail.com")).await;
    //LogReg::generate_2fa(String::from("monkaw@gmail.com")).await;
    //LogReg::generate_vcode(String::from("monkaw@gmail.com")).await;
    //LogReg::toggle_blocked(String::from("nabeelmirza79@gmail.com")).await;
    //println!("{}",login_user(String::from("monkaw@gmail.com"), String::from("12345678")).await);
    //match_2fa(String::from("monkaw@gmail.com"), String::from("tWS8")).await;
    //match_vcode(String::from("monkaw@gmail.com"), String::from("xbbhzcv")).await;
    
    // let mut x = Subscriptions::Subscription::new();
    // x.set_name("Hello World".to_owned());
    // x.set_id("3ARC".to_owned());
    // x.append_features("30 Datasets".to_owned());
    // x.append_features("30 Queries".to_owned());
    
    // x.append_features_desc("Amazing".to_owned());
    // x.append_features_desc("Blazing".to_owned());

    // let mut ids = Vec::new();

    // for _i in 0..50{

    // let temp = LogReg::randomcode(8).await;

    // x.set_id(temp.clone());
    // ids.push(temp);
    // Subscriptions::publish(&x, &client).await;
    // println!("Publish Number => {}", _i);
    // }
    

}


//filename, email, first two words, linkedin

async fn perform(){

    let x = fs::read_dir("./PDF").unwrap(); 

    let mut VECTOR = Vec::new();


    for g in x {
        let y = g.as_ref();
        let x = String::from(y.unwrap().file_name().to_str().unwrap());
        let z = x.len();
        if x.ends_with(".pdf"){
            VECTOR.push(x[0..(z-4)].to_owned());
        }
    }

    for g in VECTOR.clone(){

        convert_to_jpeg(g).await;

    }

    let mut candidates = simple_parse().await;

    for g in VECTOR.clone(){

        ocr_jpeg(g).await;
 
    }

    let mut j = 0 as usize;

    for g in VECTOR.clone(){
    println!();
    println!("{:?} => Processed",g);
    let mut path = "./TEMP/".to_owned();
    path.push_str(&g);
    path.push_str(".txt");
    let data = fs::read_to_string(path).expect("Unable to read file");
    let data = data.split(" ").collect::<Vec<_>>();
    let mut i = 0;
    let mut name = "".to_owned();

    for h in data{
        
        print!("{:?} ",h);

        if i < 2{
            if h.contains("-"){ 
                i +=2;
                name.push_str(&h.replace("-", " "));
                // print!("{:?} ",h);

                continue;
            }
            name.push_str(h);
            name.push_str(" ");
            //print!("{:?} ",h);

        }
        if i == 2{
            println!("Name => {:?}", name);
            candidates[j].tokens.push(name.to_owned());
            i+=1;
        }

         if h.contains("@"){
            println!("Email => {:?}",h);
            if h.contains("/") || h.contains("]") || h.contains("[")  {

            }
            candidates[j].tokens.push(h.to_owned());
            continue;
         }

         if h.contains("github.com"){
            println!("Github => {:?}",h);
            candidates[j].tokens.push(h.to_owned());
            continue;
         }
         if h.contains("linkedin.com"){
            println!("Linkedin => {:?}",h);
            candidates[j].tokens.push(h.to_owned());
            continue;
         }

         if h.contains("+92") && h.len()>7{
            candidates[j].tokens.push(h.to_owned());
            println!("Phone => {:?}",h);
            continue;
         }
         if h.contains("03") && h.len()>7{
            candidates[j].tokens.push(h.to_owned());
           println!("Phone => {:?}",h);
            continue;
         }
         i+=1;
    }
    j+=1;
    }

    println!();

    let mut profile: Vec<candidate> = Vec::new();

    for i in 0..VECTOR.len(){
        
        let f_name = VECTOR[i].to_string();
        let mut n = String::from("");
        let mut e = String::from("");
        let mut git =String::from("");
        let mut LI = String::from("");
        let mut ph =String::from("");
        let mut bools = vec![true,false,false,false,false,false];
        let mut index = 1 as usize;
        println!("");
        println!("{:?}", f_name);

        for k in &candidates[i].tokens{

            println!("k => {:?}",k.trim_end());
        
            if k.trim_end().contains(" ") && !bools[1]{

                let b = k.trim_end();
                println!("filename => {:?} | Name => {:?}",f_name,b);
                bools[1] = true;
                n = b.to_owned();
                continue;
            }
            if k.contains("@") && !bools[2]{
                
                let b = k.trim_end();
                bools[2] = true;
                e = b.to_owned();
                continue;
            }
            if k.contains("github") && !bools[3]{
                
                let b = k.trim_end();
                bools[3] = true;
                git = b.to_owned();
                continue;
            }
            if k.contains("linked") && !bools[4]{
                
                let b = k.trim_end();
                bools[4] = true;
                LI = b.to_owned();
                continue;
            }
            if k.contains("+92") || k.contains("03") && !bools[5]{
                
                let b = k.trim_end();
                bools[5] = true;
                ph = b.to_owned();
                continue;
            }
        }

        let mut nougat = f_name.clone();

        let mut na = n.clone().trim_end().to_owned();

        println!();

        let mut check_a = false; // if email contains some part of name
        let mut check_b = false; // if file name contains some part of name

        for z in na.split_whitespace(){
            //println!{"z = > {:?}",z.to_lowercase()};
            let a = z.to_lowercase();
            let b = e.clone().to_lowercase();
            if b.contains(&a){
                check_a = true;
                break;
            }
        }

        if !check_a{

            nougat = nougat.replace("CV", "");
            nougat = nougat.replace("Resume", "");
            nougat = nougat.replace("resume", "");
            nougat = nougat.replace("Latest", "");
            nougat = nougat.replace("latest", "");
            nougat = nougat.replace("+", "");
            nougat = nougat.replace("updated", "");
            nougat = nougat.replace("update", "");
            nougat = nougat.replace(")", " ");
            nougat = nougat.replace("(", " ");
            nougat = nougat.replace("_", " ");
            nougat = nougat.replace("-", " ");
            nougat = nougat.replace("   ", " ");
            nougat = nougat.replace("  ", " ");
            nougat = nougat.replace("  ", " ");
            nougat = nougat.trim_end().to_owned();
            nougat = nougat.trim_start().to_owned();

            for i in 0..10{
                let xd = i.to_string();
                nougat = nougat.replace(&xd, "");
            }

            println!("WARNING | SUSPICION that OCR may have incorrectly detected name | NOUGAT GENERATED  => {:?}", nougat);

        for z in nougat.split_whitespace(){
            //println!{"z = > {:?}",z.to_lowercase()};
            let a = z.to_lowercase();
            let b = n.clone().to_lowercase();
            if b.contains(&a){
                println!{"Suspicion Cleared! | OCR is correct!"};
                check_b = true;
                break;
            }
        }
        }

        if !check_b && !check_a{

            println!("Discrepency Detected: OCR failed to detect a valid name for the candidate!");
            n = nougat.clone();
            println!("New Name => {:?}", n);
        }

        n = n.to_lowercase();
        let char_array:Vec<char> = n.chars().collect();
        let mut capitalize = true;

        n = String::from("");

        for z in char_array{
            if z.is_whitespace(){
                capitalize = true;
                n.push(z);
                continue;
            }
            if capitalize{
                n.push(z.to_ascii_uppercase());
                capitalize = false;
                continue;
            }else{
                n.push(z);
                continue;
            }
        }

        let gg = gitlink_to_user(git.clone()).await;

        let mut repos = Vec::new();

        if gg!=""{

        repos = grab_repos(gg).await; //Array of repos
        
        }

        let can = candidate{
            filename : f_name,
            name:n,
            email:e,
            Git_User:git,
            LinkedIn_URL: LI,
            Phone:ph,
            git_repos:repos
        };
        profile.push(can);
    }

    println!();
    //notify_cv_scan(profile).await;
   // println!("{:?}", f);

}

async fn convert_to_jpeg(x:String){

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library()).unwrap(),
    );

    let mut path = "./PDF/".to_owned();
    path.push_str(&x);
    path.push_str(".pdf");
    println!("{:?}", path);
    let pass =Some("");
    let document = pdfium.load_pdf_from_file(&path, pass).unwrap();
    
    let render_config = PdfRenderConfig::new()
    .set_target_width(3000)
    .set_maximum_height(3000)
    .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);
    let mut f_name = x.clone();

    let pages = document.pages().first().unwrap();
    pages.render_with_config(&render_config).unwrap().as_image().as_rgba8() .ok_or(PdfiumError::ImageError).unwrap().save_with_format(
        format!("./TEMP_JPEGs/{}.jpg", f_name), 
        image::ImageFormat::Jpeg
    ) 
    .map_err(|_| PdfiumError::ImageError).unwrap();

}

async fn simple_parse() ->Vec<candidateData>{

    let mut array = Vec::new();

    let x = fs::read_dir("./PDF").unwrap(); 

    for g in x {
        let y = g.as_ref();
        let x = String::from(y.unwrap().file_name().to_str().unwrap());
        if x.ends_with(".pdf"){
        let result = extract_links(x).await;
        let mut F = candidateData{
            tokens : result,
        };
        array.push(F);
        }

    }

    return array;

}

async fn extract_links(x:String) -> Vec<String> {

    let mut g = Vec::new();
    let mut path = "./PDF/".to_owned();
    path.push_str(&x);
    let file = File::open(path).unwrap();

    println!("");
    println!("{:?}", x);
    let x =BufReader::new(file).lines();
    for line in x {
        if let Ok(ip) = line {

            if ip.contains("github"){
                let start = ip.find("(").unwrap();
                let end = ip.find(")").unwrap();
                g.push(ip[(start+1)..end].to_owned());
                
                continue;
            }
            if ip.contains("linkedin"){
                let start = ip.find("(").unwrap();
                let end = ip.find(")").unwrap();
                g.push(ip[(start+1)..end].to_owned());
                continue;
            }
            if ip.contains("mailto"){
                let start = ip.find("(mailto:").unwrap();
                let end = ip.find(")").unwrap();
                g.push(ip[(start+8)..end].to_owned());
                continue;
            }
            // if ip.contains("https"){
            //     let start = ip.find("https").unwrap_or_else(|| 0);
            //     let mut end = ip.find(")").unwrap_or_else(|| 1);
            //     if end < start{
            //         end = ip.len();
            //     }
            //     g.push(ip[(start)..end].to_owned());
            //     continue;
            // }
        }
    }

    //  for line in g.clone(){
    //      println!("{:?}", line);
    // }

    return g.clone();


}

async fn ocr_jpeg(x:String){

    println!("{:?}", x.clone());
    let mut path = String::from("./TEMP_JPEGs/");
    path.push_str(&x);
    path.push_str(".jpg");
    let base64 = image_base64::to_base64(&path); 

    let client = Client::builder().build().unwrap();
    let mut h  = HashMap::new();
    h.insert("base64Image".to_owned(), base64);
    h.insert("language".to_owned(), "eng".to_owned());
    h.insert("OCREngine".to_owned(), "1".to_owned());

    let response = client.post("https://api.ocr.space/parse/image").header("apikey", "3d7d76351588957").form(&h).send().await.unwrap().text().await.unwrap();
    
    

    let parse = json::parse(&response).unwrap();

    let data = parse["ParsedResults"][0]["ParsedText"].to_string().replace("\r\n", " ");
    let mut txt_path = "./TEMP/".to_owned();
    txt_path.push_str(&x);
    txt_path.push_str(".txt");
    fs::write(txt_path, data).expect("Unable to write file");

    println!("{:?}",parse["ParsedResults"][0]["ParsedText"].to_string().replace("\r\n", " "));
    println!("");
    println!("");


    //Promise => 
    //8GB
    //5.0 MB

}

async fn clearStr(){

}

async fn gitlink_to_user(x:String)->String{

    let mut username = x.clone();

    username = username.replace("https://", "");
    username = username.replace("http://", "");
    username = username.replace("www.github.com/", "");
    username = username.replace("github.com/", "");
    username = username.replace("/", "");

    println!("{:?}", username);

    return username;

}

async fn grab_repos(username:String) -> Vec<String>{

    let mut results = Vec::new();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("SBSixteen"));
    headers.insert(AUTHORIZATION,HeaderValue::from_static("Bearer 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") );
    let UN = username.to_owned();


    let client = Client::builder().default_headers(headers).build().unwrap();


    let mut url = String::from("https://api.github.com/users/");
    url.push_str(&UN);
    url.push_str("/repos");

    //let response = client.get(url).send().await.unwrap();
    let response = client.get(url).send().await.unwrap();

    let g = response.text().await.unwrap();

    let parse = json::parse(&g).unwrap();

    println!("Repositories [{:?}] :", parse.len());

    if parse[0]["name"].to_string() == "null"{
        return results;
    }

    for  n in 0..parse.len() {
    
        let mut strx = String::from("");
        strx.push_str(&parse[n]["name"].to_string());
        strx.push_str(" | Language => ");
        strx.push_str(&parse[n]["language"].to_string());
        results.push(strx);
    }
    return results;


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
        //WIP sSHipiti PUPUPUP oh yes yes yes        
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

async fn notify_cv_scan(c:Vec<candidate>){

    let mut creds = Credentials::new(
        String::from("noreply.ezhire@gmail.com"),
        String::from("cuifmjmwknkifsmi"),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();


    for i in c{
    let mut msg = String::from("Hello ");
    msg.push_str(&i.name);
    msg.push_str(", \n\n");
    msg.push_str("Hope you're having a blessed Ramzan! I would like to inform you that your CV was processed at the time you received this email. \n \n");
    msg.push_str("You will find your results below, users that included more information in their CVs e.g Github links will receive a much more varied response. \n \nFor now I would recommend to scan the results for any accuracy issues. If there are any, please let me know! If you have participated in the first test, it is very likely you have my number so contact me there! \n \n");
    msg.push_str("\nFilename => ");
    msg.push_str(&i.filename);
    msg.push_str("\nName => ");
    msg.push_str(&i.name);
    msg.push_str("\nGithub =>");
    msg.push_str(&i.Git_User);
    msg.push_str("\nLinkedIn =>");
    msg.push_str(&i.LinkedIn_URL);
    msg.push_str("\nPhone =>");
    msg.push_str(&i.Phone);
    msg.push_str("\n \n Github Repositories (Only if your CV included your Github profile link!) =>");

    for j in i.git_repos{
        msg.push_str("\n");
        msg.push_str(&j);
    }
    msg.push_str("\n");
    msg.push_str("\nThis is all for now. Hoping the information is correctly received. I cannot thank you enough for trusting with your CV! At EZHire, we are trying to bridge certain gaps in the hiring procedure. We deduced that in every hiring process, the candidate is made to wait for unknown intervals and has no knowledge of the recruitment process & progress. We attempt to change that by notifying you whenever your CV is parsed, We will also add a functionality where you will immediately get to know if you were picked for a position or not!");
    msg.push_str("\n");
    msg.push_str("\nThat's all for now, we shall meet again!");
    msg.push_str("\n");
    msg.push_str("\nRegards,");
    msg.push_str("\nNabeel Mirza");
    msg.push_str("\nEZHire's Backend Developer");
    
    //i.email.parse().unwrap()
    
    let email = Message::builder()
    .from("noreply.ezhire@gmail.com".parse().unwrap())
    .reply_to(i.email.parse().unwrap())
    .to(i.email.parse().unwrap())
    .subject("EZHire Services | Another CV Parsed!")
    .header(ContentType::TEXT_PLAIN)
    .body(msg)
    .unwrap();

match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully to {:?}!", i.name),
    Err(e) => panic!("Could not send email: {:?}", e),
}

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
            name:String::from("-1"),
            email: String::from("-1"),
            password: String::from("-1"),
            sub_type: -1,
            datasets: Vec::new(),
            twofa:false,
            verified:false,
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