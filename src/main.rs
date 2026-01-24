use std::{env, fs::File, io::{Read, Write}};
use wasm_bindgen::prelude::*;
use gag::BufferRedirect;

mod built_in_functions;
mod debug;
mod evaluator;
mod library;
mod state;
mod tokenizer;

pub mod value;

use debug::EvaluatioError;
use evaluator::Evaluator;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>(); // Collect command line arguments
    if args.len() < 2 {
        EvaluatioError::new("Please provide a file to evaluate".to_string()).raise();
        return;
    }
    let file = &args[1]; // Get the file name from arguments
    if !file.ends_with(".ipl") {
        EvaluatioError::new("File must be a IPL file".to_string()).raise();
        return;
    }
    run_ipl_file(file);
}

fn run_ipl_file(file: &String) {
    let mut evaluator: Evaluator = Evaluator::new(); // Create a new evaluator for the file
    evaluator.ev_file(file); // Evaluate the file
}

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
        run_ipl_file(&temp_file_path.to_str().unwrap().to_string());
        redirect.read_to_string(&mut buffer).unwrap();
    }
    
    buffer
}