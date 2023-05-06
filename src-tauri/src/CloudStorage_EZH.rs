use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use pdfium_render::{
    prelude::{PdfBitmapRotation, Pdfium, PdfiumError},
    render_config::PdfRenderConfig, document,
};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::{fs, collections::HashMap, time::Instant};

#[derive(Serialize, Deserialize)]
pub struct EZH_File {
    name: String,
    base64: String,
}

// Write file to server
pub async fn write_file(owner: String, b64: String, filename: String) {
    let mut path = path_generator(owner);
    path.push_str("/");
    path.push_str(&filename);

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let result = CUSTOM_ENGINE.decode(b64).unwrap();
    fs::write(path, result).expect("Unable to write file");
}

// Fetch file from server
pub async fn return_file(owner: String, filename: String) -> String {
    let path = file_path_generator(owner.clone(), filename.clone());
    let data = fs::read(path).expect("Unable to write file");

    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    let result = CUSTOM_ENGINE.encode(data);

    let val = EZH_File {
        name: filename,
        base64: result,
    };

    return serde_json::to_string(&val).unwrap();
}

// Delete file from server
pub async fn delete_file(owner: String, filename: String) {
    let path = file_path_generator(owner, filename);
    let _f = fs::remove_file(path).unwrap();
}

//Generate Filepath UUID
pub async fn generate_dir(email: String) {
    let mut path = "./cloudStorage/".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    fs::create_dir_all(path).unwrap();

    generate_temp_path(u_dir.clone());
}

fn generate_temp_path(hash: String) {
    let mut path = "./cloudStorage/".to_string();
    path.push_str(&hash);
    path.push_str("/TEMP_PDF");
    fs::create_dir_all(path).unwrap();

    let mut path = "./cloudStorage/".to_string();
    path.push_str(&hash);
    path.push_str("/TEMP_JPEG");
    fs::create_dir_all(path).unwrap();

    let mut path = "./cloudStorage/".to_string();
    path.push_str(&hash);
    path.push_str("/TEMP_TXT");
    fs::create_dir_all(path).unwrap();
}

pub fn path_generator(email: String) -> String {
    let mut path = "./cloudStorage/".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    return path;
}

pub fn file_path_generator(email: String, file: String) -> String {
    let mut path = "./cloudStorage/".to_string();
    let u_dir = sha256::digest(email);
    path.push_str(&u_dir);
    path.push_str("/");
    path.push_str(&file);
    return path;
}

pub async fn generate_PDF_queue_report(account: String, proceed: bool) {
    let hash = sha256::digest(account.clone());

    let mut path = String::from("./cloudStorage/");
    path.push_str(&hash);
    path.push_str("/TEMP_PDF/");

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

    let mut path = String::from("./cloudStorage/");
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
            format!("./cloudStorage/{}/TEMP_JPEG/{}.jpg", hash,f_name), 
            image::ImageFormat::Jpeg
        ).unwrap();
        //add delays here
        let elapsed = now.elapsed();
        println!("{} has been succesfully converted to JPEG | Time Elapsed : {:.4?} milliseconds", &f_name, elapsed);
    }

    jpeg2txt(hash).await;

}

pub async fn jpeg2txt(hash:String){
    
    println!("");
    println!("Dislaimer : OCR Time varies on internet speed");

    

    let mut path = String::from("./cloudStorage/");
    path.push_str(&hash);
    path.push_str("/TEMP_JPEG/");

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

        let response = client.post("https://api.ocr.space/parse/image").header("apikey", "3d7d76351588957").form(&h).send().await.unwrap().text().await.unwrap();
        let parse = json::parse(&response).unwrap();

        let data = parse["ParsedResults"][0]["ParsedText"].to_string().replace("\r\n", " ");
        let mut txt_path = format!("./cloudStorage/{}/TEMP_TXT/{}.txt", &hash, &f_name);
        fs::write(txt_path, data).expect("Unable to write file");
        //add delays here
        let elapsed = now.elapsed();
        println!("{} has been OCR'd! | Time Elapsed : {:.4?} milliseconds", &f_name, elapsed)
    }


}