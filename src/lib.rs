use std::{fs::File, io::{Read, Write}};

mod built_in_functions;
mod debug;
mod evaluator;
mod library;
mod state;
mod tokenizer;
pub mod value;

use evaluator::Evaluator;

use wasm_bindgen::prelude::*;
use gag::BufferRedirect;


#[wasm_bindgen]
pub fn run_ipl_code(ipl_code: String) -> String {
    // Create temporary file
    let mut temp_file_path = std::env::temp_dir();
    temp_file_path.push("temp.ipl");

    if let Ok(mut temp_file ) = File::create(&temp_file_path){
        let _ = temp_file.write_all(ipl_code.as_bytes());
        let _ = temp_file.flush();
    }

    // Redirect stdout
    let mut buffer = String::new();
    {
        let mut redirect = BufferRedirect::stdout().unwrap();

        let mut evaluator: Evaluator = Evaluator::new(); // Create a new evaluator for the file
        evaluator.ev_file(temp_file_path.to_str().unwrap()); // Evaluate the file
        
        redirect.read_to_string(&mut buffer).unwrap();
    }
    
    buffer
}