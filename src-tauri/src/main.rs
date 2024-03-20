// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::GenericImageView;
use std::path::Path;

fn main() {
    /* let img = image::open("src/test.jpeg").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    img.save("src/test.gif").unwrap(); */

    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
