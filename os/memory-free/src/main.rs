use std::env;

const ONE_MB_IN_BYTES: usize = 1024 * 1024;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Must be provided 2 arguments one of which is number of memory for test in MB: ./memory-free []")
    }

    let x = args[1].parse::<usize>();

    match x {
        Err(err) => panic!("Cannot parse second argument, please enter positive number {}", err),
        Ok(n) => {
            let mut array = vec![0u8; n * ONE_MB_IN_BYTES];

            println!("The number of elements in the array is {}", array.len());
            
            loop {
                for i in array.iter_mut() {
                    *i = i.wrapping_add(1);
              }

            }
        }
    }
}
