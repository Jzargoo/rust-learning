use std::{fs::OpenOptions, io::Read};




fn main() {
    let mut greeting_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("hello") {
            Ok(file) => file,
            Err(error) => panic!("Problem opening file: {:?}", error),
        };

    println!("{:?}", greeting_file);
    let mut content = String::new();

    
    match greeting_file.read_to_string(&mut content) {
        Ok(_) => println!("{}", content),
        Err(_) => panic!("Cannot read a file")
    };

}
