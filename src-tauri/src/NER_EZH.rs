use std::{collections::HashMap, fs::{self, File}, hash::Hash, io::{self,BufReader,Write, BufRead}};
// use std::io::{self, BufRead};

use regex::Regex;
// use reqwest::Client;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use sha256::digest;

use crate::CloudStorage_EZH::{context_path_generator,return_dir_count,return_dir_contents, self};
use reqwest::{header::{USER_AGENT, HeaderMap, HeaderValue, AUTHORIZATION}, Client};
use crate::Single_Column::{caller};

#[tauri::command]
pub async fn ner_caller( email:String, filename:String ,dataset_name:String){

    let owner = email.clone();

    // generate_PDF_queue_report(owner.clone(), true).await;
    // jpeg2txt(owner.clone()).await;

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("SBSixteen"));
    headers.insert(AUTHORIZATION,HeaderValue::from_static("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") );

    let client = Client::builder().default_headers(headers).build().unwrap();


    let mut candidates: Vec<Candidate> = Vec::new();

    for i in 0..return_dir_count(owner.clone(), "TEMP_TXT".to_string()).await{
        candidates.push(Candidate::new().await);
        println!("Candidate #{} is generated.", (i+1));
    }

    for i in 0..return_dir_count(owner.clone(), "TEMP_TXT".to_string()).await{
    let temp = return_dir_contents(owner.clone(), "TEMP_TXT".to_string());

    let contexts = generate_contexts(owner.clone(), temp.clone()).await;

    let mut temple = Template::new();
    temple.generate_from_default("languages.txt".to_string());
    temple.change_name("Junior Software Developer".to_string());

    let inst: Result<Vec<String>, io::Error>=buf_reader(contexts[i].clone()).await;
    
    // caller(contexts[i].clone()).await;
    candidates[i].set_name(temp[i].clone());
    candidates[i].generate_tokens(contexts[i].clone()).await;
    candidates[i].generate_skills(&temple).await;
    candidates[i].generate_institute(inst.unwrap()).await;
    candidates[i].generate_workexp(contexts[i].clone()).await;
    candidates[i].clear_tokens();

    // println!("{:#?}", candidates[i]);

    let x = sender{
        name:candidates[i].name.clone(),
        phone:candidates[i].phone.clone(),
        skills:candidates[i].skills.clone(),
        institutes:candidates[i].institutes.clone(),
        workexp:candidates[i].workexp.clone(),
        git_repos:candidates[i].git_repos.clone(),
        github:candidates[i].github.clone(),
        git_username:candidates[i].git_username.clone(),
        linkedin:candidates[i].linkedin.clone(),
    };
    
    let ind = temp[i].trim_end_matches(".txt");
    let client = Client::builder().build().unwrap();

    let url = url_generator_files(email.clone(),dataset_name.clone(),ind.to_string()).await;

    let r1 = client.get(&url).send().await.unwrap();

    let result = r1.text().await.unwrap(); 
    let _response = client.put(&url).body(serde_json::to_string(&x).unwrap().replace("\\", "")).send().await.unwrap();

    }
}

async fn url_generator_files(email:String,dataset_name:String,filename:String) -> String{

    let mut url = String::from("");

    url.push_str("https://rust-testezh-default-rtdb.europe-west1.firebasedatabase.app/Users/");
    url.push_str(&sha256::digest(email.to_owned()));
    url.push_str("/Datasets/");
    url.push_str(dataset_name.as_str());
    url.push_str("/Candidates/");
    url.push_str(filename.as_str());
    url.push_str(".json");

    println!("{:?}",url);
    return url;

}

pub async fn buf_reader(mut ocr: String) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open("Templates\\Assets\\institutes.txt")?;
    let mut associations = Vec::<String>::new();

    // Create a vector to store the names
    let mut universities = Vec::new();

    // Read the file line by line and fetch the names
    for line in io::BufReader::new(file).lines() {
        if let Ok(name) = line {
            universities.push(name);
        }
    }

    ocr = ocr.to_lowercase();

    // Check for matches and add to associations vector
    for university in universities {
        if ocr.contains(&university.to_lowercase()) {
            associations.push(university);
        }
    }

    Ok(associations)
}

pub struct Template {
    name: String,
    tokens: Vec<String>,
}

impl Template {
    pub fn new() -> Template {
        return Template {
            name: String::new(),
            tokens: Vec::new(),
        };
    }
    
    pub fn generate_from_default(&mut self, interface:String){

        self.tokens = Vec::new();
        let mut path = "./Templates/Assets/".to_string();
        path.push_str(&interface);

        let f = File::open(&path).expect("Unable to open file");

        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.expect("Unable to read line");
            self.tokens.push(line);
        }

    }

    fn append_tokens(&mut self, input: String) {
        let mut data: Vec<String> = input.split(",").map(str::to_string).collect();

        self.tokens.append(&mut data);
    }

    pub fn change_name(&mut self, input: String) {
        self.name = input;
    }

    fn pull_tokens_ref(&mut self) -> &Vec<String> {
        return self.tokens.as_ref();
    }

    fn pull_tokens(&mut self) -> Vec<String> {
        return self.tokens.clone();
    }

    pub fn size(&self) -> usize {
        return self.tokens.len();
    }
}

// struct candidate {
//     experience: Vec<experience>,
// }

#[derive(Serialize, Deserialize, Debug , Clone)]
pub struct experience {
    name: Vec<String>,
    years: Vec<String>,
}

impl experience {
    pub async fn new() ->experience{
        return experience{
            name:Vec::new(),
            years:Vec::new(),
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct sender{
    //Personal Info
    name: String,
    phone: String,

    //Crux
    skills: HashMap<String,i32>,
    institutes:Vec<String>,
    workexp: experience,
    git_repos: Vec<Git_Repo_Info>,

    //Socials
    github: String,
    git_username: String,
    linkedin: String,

    //metadata
    // template:String,
    // tokens:Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    //Personal Info
    name: String,
    phone: String,

    //Crux
    skills: HashMap<String,i32>,
    institutes:Vec<String>,
    workexp: experience,
    git_repos: Vec<Git_Repo_Info>,

    //Socials
    github: String,
    git_username: String,
    linkedin: String,

    //metadata
    template:String,
    tokens:Vec<String>,
}

impl Candidate {
    pub async fn new() -> Candidate {
        return Candidate {
            name: String::new(),
            phone: String::new(),
            skills: HashMap::new(),
            institutes:Vec::new(),
            workexp: experience { name: Vec::new(), years: Vec::new() },
            git_repos: Vec::new(),
            github: String::new(),
            git_username: String::new(),
            linkedin: String::new(),
            template:String::from("NONE"),
            tokens:Vec::new()
        };
    }
    // pub async fn generate_workexp(&mut self, titles: Vec<String>, exp_num: Vec<String>) {
        pub async fn generate_workexp(&mut self, ocr:String) {
        let z = caller(ocr).await;
        // caller(contexts[i].clone()).await;
        let x = experience {
            name: z.0,
            years: z.1,
        };
        self.workexp=x;
    }

    fn get_phone(&mut self,regex:&Regex, context:String){

        let temp = context.replace(" ", "-");

        let mat:Vec<String> = regex.find_iter(&context).map(|digits| digits.as_str().to_owned()).collect();

        for i in mat{

            if self.phone.len() < i.len(){

                self.phone = i;

            }

        }

    }

    pub fn set_name(&mut self, input: String) {
        self.name = input;
    }
    pub fn get_name(&mut self) ->String{
        return self.name.clone();
    }
    fn set_phone(&mut self, input: String) {
        self.phone = input;
    }
    fn set_skill(&mut self, input: HashMap<String,i32>) {
        self.skills = input;
    }
    fn append_skill(&mut self, input:String){

        if self.skills.contains_key(&input){
            self.skills.insert(input.clone(), self.skills.get(&input).unwrap() + 1);
            return;
        }else{
            self.skills.insert(input.clone(), 1);
        }

    }

    // fn set_workexp(&mut self, input: Vec<String>) {
    //     self.workexp = input;
    // }
    fn set_git_url(&mut self, input: String) {
        self.github = input;
        self.identify_git_username();
    }

    fn fetch_git_from_context(&mut self){

        for i in &self.tokens{

                if i.to_lowercase().contains("github.com"){
                    //println!("{} | {}", i,j);
                   self.github = i.to_string();
                   self.identify_git_username();
                   return;
                }
        }

    }

    fn set_linkedin_url(&mut self, input: String) {
        self.linkedin = input;
    }
    fn identify_git_username(&mut self) {
        let mut temp = self.github.clone().to_lowercase();
        temp = temp.replace("https://", "");
        temp = temp.replace("http://", "");
        temp = temp.replace("www.github.com/", "");
        temp = temp.replace("github.com/", "");
        temp = temp.replace("/", "");
        self.git_username = temp;
    }
    fn append_git_repo(&mut self, input: Git_Repo_Info) {
        self.git_repos.push(input);
    }
    pub async fn generate_git_report(&mut self, repos:Vec<String>, client:&Client){

        for i in repos{
            let mut temp = Git_Repo_Info::automatic(client, self.git_username.clone(), i).await;
            self.append_git_repo(temp);
        }

    }
    pub async fn generate_skills(&mut self, template:&Template){
        
        let d = self.tokens.clone();
        let e = template.tokens.clone();

        let mut temp = String::new();

        for i in &d{
            for j in &e{

                temp.push_str(&format!("{} | {} \r\n", i,j));
                if i.to_lowercase() == j.to_lowercase(){
                    //println!("{} | {}", i,j);
                    self.append_skill(i.clone());
                }
            }
        }
        // fs::write("./ali.txt", temp).expect("Unable to write file");
    }



    pub async fn generate_institute(&mut self, inst:Vec<String>){
        self.institutes=inst;
    }

    pub async fn generate_tokens(&mut self,context:String){

        let mut temp = context.replace(",", "");

        let p = temp.split_whitespace();

        for g in p{
            self.tokens.push(g.to_string());
        }

    }

    pub fn clear_tokens(&mut self){
        self.tokens = Vec::new();
    }

}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Git_Repo_Info {
    name: String,
    description: String,
    url: String,
    //https://api.github.com/repos/{candidate.git_username}/{git_repos[i]}/languages
    score: HashMap<String, i32>,
}

impl Git_Repo_Info {
    pub fn new() -> Git_Repo_Info {
        return Git_Repo_Info {
            name: String::new(),
            description: String::new(),
            url: String::new(),
            score: HashMap::new(),
        };
    }

    pub async fn automatic(client:&Client,owner:String, repo:String) -> Git_Repo_Info{

        let url = generate_repo_info(owner.clone(), repo.clone());
        let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();
        let repo_info:Value = serde_json::from_str(&r1).unwrap();
        let mut answer = Git_Repo_Info::new();

        answer.name = repo_info["name"].as_str().unwrap().to_string();

        if repo_info["description"].is_null(){
            answer.description = "DNE (Does Not Exist)".to_string();
        }else{
        answer.description = repo_info["description"].as_str().unwrap().to_string();
        }
        answer.url = repo_info["html_url"].as_str().unwrap().to_string();

        let url = generate_repo_languages(owner.clone(), repo.clone());
        let r2 = client.get(&url).send().await.unwrap().text().await.unwrap();

        let repo_languages:Value = serde_json::from_str(&r2).unwrap();

        answer.hyper_score(repo_languages);

        return answer;

    }
    fn hyper_score(&mut self, input:serde_json::Value){

        let mut data:String = input.to_string();
        data = data.replace("{", "");
        data = data.replace("}", "");
        
        let mut temp:Vec<String> = data.split(",").map(str::to_string).collect();
        let mut real = HashMap::new();
    
        for i in 0..temp.len(){
            temp[i] = temp[i].replace("\"", "");
        }
    
        for i in temp{
            println!("{}", &i);
            let index = i.find(":").unwrap_or(1892);

            if index == 1892{
                continue;
            }

            let key = i[0..index].to_string();
            
            let val = i[index+1..i.len()].to_string();
            //println!("{} => {}", &key, &val);
            real.insert(key, val.parse::<i32>().unwrap());
        }

        self.score = real;

        //hogaya kaam behen ke loray

    }
    fn append_score(&mut self, key:String, value:i32){

        if self.score.contains_key(&key){
            self.score.insert(key.clone(),value + self.score.get(&key).unwrap());
        }else{
            self.score.insert(key.clone(),value);
        }

    }
}

//Detect Personal Info
//Name : Done
//Phone : Done

//Detect Work Experience

//Detect Skills

//Templates

//Miscellenous

//GitHub wrapper functions
fn generate_info_url(user:String)-> String{

    let mut url = String::from("https://api.github.com/users/");
    url.push_str(&user);
    return url;

}

fn generate_repos_url(user:String)-> String{
    let mut url = String::from("https://api.github.com/users/");
    url.push_str(&user);
    url.push_str("/repos");
    return url
}

fn generate_repo_info(user:String, repo:String)-> String{
    let mut url = String::from("https://api.github.com/repos/");
    url.push_str(&user);
    url.push_str("/");
    url.push_str(&repo);
    return url
}

fn generate_repo_languages(user:String,repo:String)-> String{
    let mut url = String::from("https://api.github.com/repos/");
    url.push_str(&user);
    url.push_str("/");
    url.push_str(&repo);
    url.push_str("/languages");
    return url;
}

pub async fn generate_git_report(user:String){
    //generate_repos_url(user)
}

pub async fn generate_contexts(owner:String,arr:Vec<String>) -> Vec<String>{

    let mut data = Vec::new();

    for i in arr{
        let mut path = context_path_generator(owner.clone());
        path.push_str(&i);
        data.push(fs::read_to_string(&path).expect("Unable to read file"));
    }
    return data;
}

pub async fn process_one_cv(owner:String, name:String){

    // let mut path = CloudStorage_EZH::path_generator(owner.clone());
    // path.push_str("\\.files\\");
    // path.push_str(&name);

    // let bytes = std::fs::read(&path).unwrap();

    // let mut path = CloudStorage_EZH::path_generator(owner.clone());
    // path.push_str("\\TEMP_PDF\\");
    // path.push_str(&name);
    
    // fs::write(path, bytes).unwrap();

    let mut data = Candidate::new().await;

    // CloudStorage_EZH::generate_PDF_queue_report(owner.clone(), true).await;

    let mut txtname = name[0..name.len()-4].to_string();
    let mut nougat = txtname.clone();
    txtname.push_str(".txt");

    let mut path = CloudStorage_EZH::path_generator(owner.clone());
    path.push_str("\\TEMP_TXT\\");
    path.push_str(&txtname);
    let context = fs::read_to_string(&path).expect("Unable to read file");

    let re = Regex::new(r"\+?\(?\d*\)? ?\(?\d+\)?\d*([\s./-]?\d{2,})+").unwrap();

    let mut template = Template::new();

    template.change_name("Software Developer Junior".to_string());
    template.generate_from_default("languages.txt".to_string());

    data.generate_tokens(context.clone()).await;
    data.generate_skills(&template).await;
    data.fetch_git_from_context();
    data.clear_tokens();
    data.get_phone(&re, context.clone());

    if data.github !=""{

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("SBSixteen"));
        headers.insert(AUTHORIZATION,HeaderValue::from_static("token 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") );
        let client = Client::builder().default_headers(headers).build().unwrap();
        let url = format!("https://api.github.com/users/{}",data.git_username);

        println!("{}", &url);

        let r1 = client.get(&url).send().await.unwrap().text().await.unwrap();
        println!("{}", &r1);

        let info:Value = serde_json::from_str(&r1).unwrap();
        
        let x= info["name"].as_str().unwrap().to_string();

        data.set_name(x);

        let repos_url = info["repos_url"].as_str().unwrap().to_string();

        let r1 = client.get(&repos_url).send().await.unwrap().text().await.unwrap();
        let info:Value = serde_json::from_str(&r1).unwrap();

        let x = info.as_array().unwrap();

        let mut temp = Vec::new();

        for i in x{
            temp.push(i["name"].as_str().unwrap().to_string());
        }
        
        data.generate_git_report(temp, &client).await;

    }else{

        nougat = nougat.replace("CV", "");
        nougat = nougat.replace("Resume", " ");
        nougat = nougat.replace("resume", " ");
        nougat = nougat.replace("Latest", " ");
        nougat = nougat.replace("latest", " ");
        nougat = nougat.replace("+", " ");
        nougat = nougat.replace("updated", " ");
        nougat = nougat.replace("update", " ");
        nougat = nougat.replace(")", " ");
        nougat = nougat.replace("(", " ");
        nougat = nougat.replace("_", " ");
        nougat = nougat.replace("-", " ");
        nougat = nougat.replace("   ", " ");
        nougat = nougat.replace("  ", " ");
        nougat = nougat.replace("  ", " ");
        nougat = nougat.trim_end().to_owned();
        nougat = nougat.trim_start().to_owned();

        data.set_name(nougat);
    }

    template.change_name("Software Developer (Junior)".to_string());
    template.generate_from_default("institutes.txt".to_string());

    data.template = template.name;

    let smolcontext = context.to_lowercase();

    for i in template.tokens{
        if smolcontext.contains(&i.to_lowercase()){
            data.institutes.push(i);
        }
    }

    println!("{:#?}", data)

}




//API HALP
// Scoring => https://api.github.com/repos/{candidate.git_username}/{git_repos[i].name}/languages
// User Fetch => https://api.github.com/users/{candidate.git_username}