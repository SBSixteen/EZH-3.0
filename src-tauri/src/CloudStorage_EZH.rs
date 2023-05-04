use std::fs;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EZH_File{
    name:String,
    base64:String,
}

// Write file to server
pub async fn write_file(owner:String, b64:String, filename:String){

    let mut path = path_generator(owner);
    path.push_str("/");
    path.push_str(&filename);

    const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let result = CUSTOM_ENGINE.decode(b64).unwrap();
    fs::write(path, result).expect("Unable to write file");

}

// Fetch file from server
pub async fn return_file(owner:String, filename:String) -> String{

    let path = file_path_generator(owner.clone(), filename.clone());
    let data = fs::read(path).expect("Unable to write file");

    const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let result = CUSTOM_ENGINE.encode(data);

    let val = EZH_File{
        name:filename,
        base64: result
    };

    return serde_json::to_string(&val).unwrap();

}

// Delete file from server
pub async fn delete_file(owner:String, filename:String){

    let path = file_path_generator(owner, filename);
    let _f = fs::remove_file(path).unwrap();

}


//Generate Filepath UUID
pub async fn generate_dir(email:String){

    let mut path = "./cloudStorage/".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    fs::create_dir_all(path).unwrap();
    println!("is file made lol?")

}

pub fn path_generator(email:String) -> String{

    let mut path = "./cloudStorage/".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    return path
}

pub fn file_path_generator(email:String, file:String) -> String{

    let mut path = "./cloudStorage/".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    path.push_str("/");
    path.push_str(&file);
    return path
}