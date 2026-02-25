use rand::Rng; 
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret = rand::thread_rng().random_range(1..=100); 

    

    loop {
        println!("Enter your guess: ");
        
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line!");

        let guess: u32 = match guess
            .trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
