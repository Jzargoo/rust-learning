use std::{fs, io::ErrorKind};

use crate::config::{Config, parse_config};

mod config;

fn main() -> Result<(), String> {

    let args: Vec<String> = std::env::args().collect();

    let config = parse_config(&args)?;
    
    let resulted  = run(config)?;

    println!("{}", resulted.join("\n"));

    Ok(())
}

pub fn run(config: Config) -> Result<Vec<String>, String> {
    
    let content = fs::read_to_string(&config.file_path);

    if content.is_err() {
        let error = content.unwrap_err();
        let mut error_string: String = "Unpredictable error occured".to_string();  
        match error.kind() {
            ErrorKind::NotFound => error_string = "Requested file does not exist".to_string(),
            ErrorKind::PermissionDenied => error_string = "Access denied".to_string(),
            ErrorKind::Interrupted => error_string = "Reading was interrupted, try again later".to_string(),
            _ => {}
        }

        return Err(error_string);
    }

    let content = content.unwrap();
    
    let mut result = Vec::<String>::new();
    
    for i in content.split("\n"){
    
        if i.contains(&config.query) {
            result.push(i.to_string());
        }
    
    }
    
    Ok(result)
}