use std::{fs, collections::HashMap};

use pdfium_render::prelude::*;
use reqwest::Client;

//Convert to JPEG
fn convert_to_jpeg(x:String){

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

//filename, email, first two words, linkedin

//Convert to PNG

//Batch Convert to JPEG
async fn batch_convert_JPEG(paths:Vec<String>){

}

//Batch convert to PNG

//Convert with parameters

//Convert 

//OCR JPEG
async fn ocr_jpeg(x:String) -> String{

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
    fs::write(txt_path, data.clone()).expect("Unable to write file");

    println!("{:?}",parse["ParsedResults"][0]["ParsedText"].to_string().replace("\r\n", " "));

    return data;

}