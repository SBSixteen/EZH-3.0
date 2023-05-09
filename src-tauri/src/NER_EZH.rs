use std::{collections::HashMap, fs::{self, File}, hash::Hash, io::{BufReader, BufRead}};

use regex::Regex;
use reqwest::Client;
use serde_json::{json, Value};
use sha256::digest;

use crate::CloudStorage_EZH::context_path_generator;

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

#[derive(Debug)]
pub struct Candidate {
    //Personal Info
    name: String,
    phone: String,

    //Crux
    skills: HashMap<String,i32>,
    workexp: Vec<String>,
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
            workexp: Vec::new(),
            git_repos: Vec::new(),
            github: String::new(),
            git_username: String::new(),
            linkedin: String::new(),
            template:String::from("NONE"),
            tokens:Vec::new()
        };
    }

    fn get_phone(&mut self,regex:&Regex, context:String){

        let mat = regex.find(&context).unwrap();
        self.phone = mat.as_str().to_string();

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
    fn set_workexp(&mut self, input: Vec<String>) {
        self.workexp = input;
    }
    fn set_git_url(&mut self, input: String) {
        self.github = input;
        self.identify_git_username();
    }
    fn set_linkedin_url(&mut self, input: String) {
        self.linkedin = input;
    }
    fn identify_git_username(&mut self) {
        let mut temp = self.github.clone();
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
    pub async fn generate_git_report(&mut self){

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
        fs::write("./ali.txt", temp).expect("Unable to write file");
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

#[derive(Debug)]
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
        answer.description = repo_info["description"].as_str().unwrap().to_string();
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
            let index = i.find(":").unwrap();
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
//API HALP
// Scoring => https://api.github.com/repos/{candidate.git_username}/{git_repos[i].name}/languages
// User Fetch => https://api.github.com/users/{candidate.git_username}