use pdfium_render::prelude::*;

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

//PDF OCR