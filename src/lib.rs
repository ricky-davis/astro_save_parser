
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::str;
mod save;
use save::*;

#[wasm_bindgen]
#[allow(non_snake_case)]
#[no_mangle]
pub fn process_file(fileName: String, fileSize: u64, fileData: &[u8]) -> String{
    let mut newsave = Save::new_bytes(fileName, fileSize, fileData);
    println!("{:?}",newsave);
    newsave.load();
    
    format!("eeeeeee: {:?}",newsave)
}