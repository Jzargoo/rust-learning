mod tasks;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        panic!("Must be provided only 3 arguments: ./io input output");
    }

    match std::fs::read(
        args.get(1).unwrap()
    ) {
        
        Err(error) => panic!("There was an error during reading a file {}", error),
        
        Ok(bytes) => {
            if let Err(we) = 
                    std::fs::write(args.get(2).unwrap(), bytes) {
                        panic!("There was an error during writing into a file {}", we)
            };
        }
    }
}
