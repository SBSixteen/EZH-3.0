use std::fs;
use std::process::Command;
use base64::{encode};
use base64::{decode};

//This niger listened to 100% of the instructions

fn front_FiletoString() {
sevenz_rust::compress_to_path("Create boids simulation in 2d, simi.txt",
"Create boids simulation in 2d, simi(7z-type).7z")
.expect("compress ok");

// convert to base64String 
                    let file_data = fs::read("Create boids simulation in 2d, simi(7z-type).7z").expect("failed to read temporary file");

                    // Encode the file data as a Base64 string
                    let base64_string = encode(&file_data);

                    // Print the Base64 string to the console
                    println!("Base64 string: {}", base64_string);

}

fn back_StringtoFile(){
//  convert from base64 String

let file_data = decode(base64_string).unwrap();

// Write the binary data to a file
fs::write("Create boids simulation in 2d, simi(converted-back).7z", &file_data).expect("failed to write file");

sevenz_rust::decompress_file("Create boids simulation in 2d, simi(converted-back).7z", "Dan-done.txt").expect("complete");
}


