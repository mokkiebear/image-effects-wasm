use std::io::Cursor;

use wasm_bindgen::prelude::wasm_bindgen;
// use web_sys::console;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use image::load_from_memory;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let base64_to_vector = STANDARD.decode(encoded_file).unwrap();

    let mut image = load_from_memory(&base64_to_vector).unwrap();
    image = image.grayscale();

    let mut buffer = Cursor::new(vec![]);
    image.write_to(&mut buffer, image::ImageOutputFormat::Png).unwrap();

    let encoded_image = STANDARD.encode(&buffer.into_inner());

    format!("data:image/png;base64,{}", encoded_image) // encoded_image
}