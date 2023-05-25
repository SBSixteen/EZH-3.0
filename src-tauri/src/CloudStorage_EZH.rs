#![allow(warnings)]

use rand::Rng;
use reqwest::{header::{USER_AGENT, HeaderMap, HeaderValue, AUTHORIZATION}, Client};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use sha256::digest;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

use crate::SMTP_EZH;
// use std::fs;
use std::{process::Command, path};
use base64::{encode};
use base64::{decode};


use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use pdfium_render::{
    prelude::{PdfBitmapRotation, Pdfium, PdfiumError},
    render_config::PdfRenderConfig, document,
};

use std::{fs, collections::HashMap, time::Instant};
// use std::fs::File;
#[derive(Serialize, Deserialize, Debug)]
pub struct EZH_FileBank {

    name: Vec<String>,
    size: Vec<f32>,
    date: Vec<String>,
    filetype: Vec<String>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct EZH_FileBank_kekw {

    name: String,
    size: String,
    date: String,
    filetype: String,

}


#[derive(Serialize, Deserialize, Debug)]
struct Confirmation {
    value: bool,
    response: String,
}

#[derive(Serialize, Deserialize)]
pub struct EZH_File2 {
    name: String,
    base64: String,
    filename:String,
}

#[derive(Serialize, Deserialize)]
pub struct test {
    message : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dataset{
    dataset_name:String,
    deadline:String,
    // Folder:Vec<>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DL_FILE{
    name:String,
    bytes: Vec<u8>
    // Folder:Vec<>,
}

#[tauri::command]
pub async fn create_dataset(email:String, dataset_name:String, deadline:String ){
    
    let x =  Dataset{
        dataset_name:dataset_name.clone(),
        deadline:deadline.clone(),
        // Folder:Vec<>,
    };

    let client = Client::builder().build().unwrap();

    let url = url_generator_dataset(email.clone(),dataset_name.clone()).await;

    let r1 = client.get(&url).send().await.unwrap();

    let result = r1.text().await.unwrap(); 
        let _response = client.put(&url).body(serde_json::to_string(&x).unwrap().replace("\\", "")).send().await.unwrap();


}

async fn url_generator_dataset(email:String,dataset_name:String) -> String{

    let mut url = String::from("");

    url.push_str("https://rust-testezh-default-rtdb.europe-west1.firebasedatabase.app/Users/");
    url.push_str(&sha256::digest(email.to_owned()));
    url.push_str("/Datasets/");
    url.push_str(dataset_name.as_str());
    url.push_str(".json");

    return url;

}

use std::fs::File;
use std::io::{self, Write, BufRead};


pub async fn decode_save_zip(path: String, b64: String, filename: &str) -> io::Result<()> {
    // Base64 encoded string
    let base64_string = b64;

    // Decode the base64 string
    let decoded_data = decode(base64_string.as_bytes())
        .expect("Failed to decode base64 string.");

    // Specify the path where the zip folder will be written
    let file_path = format!("{}{}.zip", path, filename);
    let mut file = File::create(file_path)?;

    // Write the decoded data to the file
    file.write_all(&decoded_data)?;

    println!("Zip folder successfully created.");

    Ok(())
}

#[tauri::command]
pub async fn decode_save(path: String, b64: String, filename: &str) -> io::Result<()> {
    // Base64 encoded string
    let base64_string = b64;

    // Decode the base64 string
    let decoded_data = decode(base64_string).expect("Failed to decode base64 string.");

    // Determine the file extension based on the data
    let file_extension = "pdf";

    // Specify the path where the file will be written
    let file_path = format!("{}{}.{}", path, filename, file_extension);
    // println!("{:?}", file_path);
    let mut file = File::create(file_path)?;

    // Write the decoded data to the file
    file.write_all(&decoded_data)?;

    println!("File successfully created.");

    Ok(())
}
use std::path::{Path, PathBuf};
use zip::ZipArchive;
fn unzip_file(zip_path: &Path, dest_dir: &Path) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = file.sanitized_name();

        // Build the destination path by joining the destination directory
        // with the file path
        let dest_path = dest_dir.join(&file_path);

        // Ensure that the parent directory of the destination path exists
        if let Some(parent_dir) = dest_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        // Extract the file to the destination path
        let mut dest_file = File::create(dest_path)?;
        io::copy(&mut file, &mut dest_file)?;
    }

    Ok(())
}

// pub async fn unzip_file(zip_path: &Path, dest_dir: &Path) -> io::Result<()> {
//     let file = File::open(zip_path)?;
//     let mut archive = ZipArchive::new(file)?;
//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i)?;
//         let file_path = file.sanitized_name();
    
//         // Build the destination path by joining the destination directory
//         // with the file path
//         let dest_path = dest_dir.join(&file_path);
    
//         // Ensure that the parent directory of the destination path exists
//         if let Some(parent_dir) = dest_path.parent() {
//             fs::create_dir_all(parent_dir)?;
//         }
    
//         // Extract the file to the destination path
//         let mut dest_file = File::create(dest_path)?;
//         tokio::io::copy(&mut file, &mut dest_file).await?;
//     }
    
//     Ok(())
// }    
// Write file to server

use tokio::time::sleep;
use std::time::Duration;

//Abay kachray kisi banday ka function badla hai tou ussay bata tou de

// #[tauri::command]
// pub async fn write_file(b64: String, filename0: String, email:String) {
//     let mut path = generate_dir(email.clone()).await;
//     if(filename0.contains(".pdf")){
//         path =path.clone()+"\\TEMP_PDF\\";
//         let filename=filename0.trim_end_matches(".pdf");
//         decode_save(path.clone(), b64, &filename).await.unwrap();
//         generate_PDF_queue_report(email,true,"SubZero".to_string()).await;
//     }
//     else if(filename0.contains(".zip")){
//     println!("{:?}","zipper on");
//     let filename=filename0.trim_end_matches(".zip");
//     let mut path0=path.clone();
//     path0=path0+"\\TEMP_ZIP\\";
//     // recreates ZIP
//     decode_save_zip(path0.to_string(), b64, &filename).await.unwrap();
//     path0=path0+&filename0;
//     let zip_path = Path::new(&path0);
//     path=path+"\\TEMP_PDF\\";
//     let dest_dir = Path::new(&path);
//     // Unzip the file to the destination directory
//     unzip_file(zip_path, dest_dir);
//     sleep(Duration::from_secs(5)).await;
//     generate_PDF_queue_report(email,true,filename.to_string()).await;
//     }
//     // generate_PDF_queue_report(email,true).await;
// }

// pub async fn generate_PDF_queue_report(account: String, proceed: bool) {


//     let decoded_data = decode(base64_string).expect("Failed to decode base64 string.");
//     let file_extension = "pdf";
//     let file_path = "\\.{}".to_string() + file_extension;
//     let mut file = File::create("RG.pdf")?;

//     file.write_all(&decoded_data)?;

//     println!("File successfully created.");

//     Ok(())

// //nab
//     const CUSTOM_ENGINE: engine::GeneralPurpose =
//         engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

//     let result = CUSTOM_ENGINE.decode(b64).unwrap();
//     println!("{:?}",result);
//     fs::write(path, result).expect("Unable to write file");
// }


// Fetch file from server
// pub async fn return_file(owner: String, filename: String) -> String {
//     let path = file_path_generator(owner.clone(), filename.clone());
//     let data = fs::read(path).expect("Unable to write file");

//     const CUSTOM_ENGINE: engine::GeneralPurpose =
//         engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

//     let result = CUSTOM_ENGINE.encode(data);

//     let val = EZH_File {
//         name: filename,
//         base64: result,
//     };

//     return serde_json::to_string(&val).unwrap();
// }

// Delete file from server
pub async fn delete_file(owner: String, filename: String) {
    let path = file_path_generator(owner, filename);
    let _f = fs::remove_file(path).unwrap();
}

//Generate Filepath UUID
pub async fn generate_dir(email: String) -> String{
    let mut path = "..\\cloudStorage\\".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    fs::create_dir_all(path.clone()).unwrap();

    generate_temp_path(u_dir.clone());
    return path.clone();
}

fn generate_temp_path(hash: String) {
    let mut path = "..\\cloudStorage\\".to_string();
    path.push_str(&hash);
    path.push_str("\\TEMP_PDF");
    fs::create_dir_all(path).unwrap();

    let mut path = "..\\cloudStorage\\".to_string();
    path.push_str(&hash);
    path.push_str("\\TEMP_ZIP");
    fs::create_dir_all(path).unwrap();

    let mut path = "..\\cloudStorage\\".to_string();
    path.push_str(&hash);
    path.push_str("\\.files");
    fs::create_dir_all(path).unwrap();

    let mut path = "..\\cloudStorage\\".to_string();
    path.push_str(&hash);
    path.push_str("\\TEMP_JPEG");
    fs::create_dir_all(path).unwrap();

    let mut path = "..\\cloudStorage\\".to_string();
    path.push_str(&hash);
    path.push_str("\\TEMP_TXT");
    fs::create_dir_all(path).unwrap();
}

pub fn path_generator(email: String) -> String {
    let mut path = "..\\cloudStorage\\".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    return path;
}

pub fn file_path_generator(email: String, file: String) -> String {
    let mut path = "..\\cloudStorage\\".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    path.push_str("/");
    path.push_str(&file);
    return path;
}


pub async fn generate_PDF_queue_report(account: String, proceed: bool){
    let hash = sha256::digest(account.clone());

    let mut path = String::from("..\\cloudStorage\\");
    path.push_str(&hash);
    path.push_str("\\TEMP_PDF\\");

    let filepaths = fs::read_dir(&path).unwrap();

    let mut files = Vec::new();

    for paths in filepaths {
        files.push(paths.unwrap().file_name().to_str().unwrap().to_string());
    }

    println!(
        "{} files in queue for PDF processing.",
        fs::read_dir(&path).unwrap().count()
    );
    if proceed {
        pdf2jpeg(files, hash.clone(), path).await;
    }else{
        println!("Bool lock preventing from further processing")
    }

}

pub async fn pdf2jpeg(files: Vec<String>, hash: String, pdf_path: String) {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .unwrap(),
    );

    let mut path = String::from("..\\cloudStorage\\");
    path.push_str(&hash);
    path.push_str("/TEMP_JPEG/");

        println!(
        "{} JPEGs to be generated.",
        fs::read_dir(&path).unwrap().count()
    );

    let render_config = PdfRenderConfig::new()
        .set_target_width(3000)
        .set_maximum_height(3000)
        .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

        let pass =Some("");

    for i in &files{        
        let now = Instant::now();
        let mut pathx = pdf_path.clone();
        pathx.push_str(&i);
        
        let document = pdfium.load_pdf_from_file(&pathx, pass).unwrap();
        
        let mut f_name = i.clone();
        f_name =f_name.replace(".pdf", "");

        let pages = document.pages().first().unwrap();
        pages.render_with_config(&render_config).unwrap().as_image().as_rgba8() .ok_or(PdfiumError::ImageError).unwrap().save_with_format(
            format!("..\\cloudStorage\\{}/TEMP_JPEG/{}.jpg", hash,f_name), 
            image::ImageFormat::Jpeg
        ).unwrap();
        //add delays here
        let elapsed = now.elapsed();
        println!("{} has been succesfully converted to JPEG | Time Elapsed : {:.4?} milliseconds", &f_name, elapsed);
    }

        jpeg2txt(hash).await;

}

#[tauri::command]
pub async fn destroy_directory(email : String) {
    let hash = sha256::digest(email);
    let mut path = String::from("..\\cloudStorage\\");
    path.push_str(&hash);
    // let dir_path = "/path/to/directory";

    // Remove the directory and all its contents recursively
    match fs::remove_dir_all(path) {
        Ok(()) => println!("Directory successfully removed."),
        Err(err) => eprintln!("Failed to remove directory: {}", err),
    }
}

#[tauri::command]
pub async fn jpeg2txt(account:String){
    let hash = account;
    println!("");
    println!("Dislaimer : OCR Time varies on internet speed");

    

    let mut path = String::from("..\\cloudStorage\\");
    path.push_str(&hash);
    path.push_str("\\TEMP_JPEG\\");

    println!("{}",path);

    let filepaths = fs::read_dir(&path).unwrap();

            println!(
        "{} JPEGs to be processed.",
        fs::read_dir(&path).unwrap().count()
    );

    let mut files = Vec::new();

    for paths in filepaths {
        files.push(paths.unwrap().file_name().to_str().unwrap().to_string());
    }

    let client = Client::builder().build().unwrap();

    for i in files{

        let now = Instant::now();

        let mut fp = path.clone();
        fp.push_str(&i);

        let base64 = image_base64::to_base64(&fp); 

        let mut h  = HashMap::new();
        h.insert("base64Image".to_owned(), base64);
        h.insert("language".to_owned(), "eng".to_owned());
        h.insert("OCREngine".to_owned(), "1".to_owned());

        let mut f_name = i.clone();
        f_name = f_name.replace(".jpg", "");

        let response = client.post("https://api.ocr.space/parse/image").header("apikey", "3d7d76351588957").form(&h).send().await.
        unwrap().text()
        .await.unwrap();


    let parse = json::parse(&response).unwrap();

        let data = parse["ParsedResults"][0]["ParsedText"].to_string().replace("\r\n", " ");
        let mut txt_path = format!("..\\cloudStorage\\{}/TEMP_TXT/{}.txt", &hash, &f_name);
        fs::write(txt_path, data).expect("Unable to write file");
        //add delays here
        let elapsed = now.elapsed();
        println!("{} has been OCR'd! | Time Elapsed : {:.4?} milliseconds", &f_name, elapsed)
    }


}


pub fn context_path_generator(email: String) -> String {
    let mut path = "..\\cloudStorage\\".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    path.push_str("\\TEMP_TXT\\");
    return path;
}

pub fn return_dir_contents(owner:String, file_in_cloud:String) -> Vec<String>{

    let hash = sha256::digest(owner);
    let mut path = String::from("..\\cloudStorage\\");
    path.push_str(&hash);
    path.push_str(&format!("/{}/", file_in_cloud));
    let filepaths = fs::read_dir(&path).unwrap();

    let mut files = Vec::new();

    for paths in filepaths {
        files.push(paths.unwrap().file_name().to_str().unwrap().to_string());
    }

    return files;
}

pub async fn upload_file(bytes:Vec<u8>, name:String, owner:String) -> String{

    let mut path = path_generator(owner.clone());
    path.push_str("\\.files\\");
    path.push_str(&name);

    fs::write(path, bytes).unwrap();

    let answer = Confirmation{
        value:true,
        response:"File has been successfully updated.".to_string()
    };

    return serde_json::to_string(&answer).unwrap();

}

pub async fn download_file(name:String, owner:String) -> String{

    let mut path = path_generator(owner.clone());
    path.push_str("\\.files\\");
    path.push_str(&name);

    let bytes = std::fs::read(&path).unwrap();

    let answer = DL_FILE{
        name:name.clone(),
        bytes:bytes,
    };

    return serde_json::to_string(&answer).unwrap();

}


#[tauri::command]
pub async fn remove_file(name:String, owner:String){

    let mut path = path_generator(owner.clone());
    path.push_str("\\.files\\");
    path.push_str(&name);

    fs::remove_file(&path);

}

pub async fn return_dir_count(owner:String, file_in_cloud:String) -> usize{
    let hash = sha256::digest(owner);
    let mut path = String::from("..\\cloudStorage\\");
    path.push_str(&hash);
    path.push_str(&format!("/{}/", file_in_cloud));

    return fs::read_dir(&path).unwrap().count();
}

#[tauri::command]
pub async fn fetch_cloud_stats(owner:String) ->String{

    let mut path = path_generator(owner.clone());
    path.push_str("\\.files\\");

    let filepaths = fs::read_dir(&path).unwrap();

    let mut files = Vec::new();

    for paths in filepaths {
        files.push(paths.unwrap().file_name().to_str().unwrap().to_string());
    }

    let mut data = EZH_FileBank{
        name:Vec::new(),
        filetype:Vec::new(),
        size:Vec::new(),
        date:Vec::new(),
    };

    let mut tempvec = Vec::new();

    for i in files{

        let mut temp = path.clone();
        temp.push_str(&i);
        
        let x = fs::metadata(&temp).unwrap().created().unwrap();
        let datetime: DateTime<Utc> = x.into();

        let mut tempname:String = i.clone().chars().rev().collect();
        tempname = tempname[tempname.find(".").unwrap()+1..tempname.len()].to_string();
        tempname= tempname.clone().chars().rev().collect();

        let G = EZH_FileBank_kekw{
            
            name:tempname,
            filetype:Path::new(&temp).extension().unwrap().to_str().unwrap().to_uppercase().to_owned(),
            size:format!("{:.2} KB ", fs::metadata(&temp).unwrap().len() as f32/1024.0),
            date:datetime.format("%d/%m/%Y - %T").to_string()

        };

        tempvec.push(G);
        data.name.push(i.clone());
        data.filetype.push(Path::new(&temp).extension().unwrap().to_str().unwrap().to_owned());
        data.size.push(fs::metadata(&temp).unwrap().len() as f32/1024.0);
        data.date.push(datetime.format("%d/%m/%Y - %T").to_string());
    }

    return serde_json::to_string(&tempvec).unwrap();

}