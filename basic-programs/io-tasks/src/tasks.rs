fn sum() {
    let mut eval = String::new();
    
    let mut sum = 0;

    if let Err(error) = std::io::stdin().read_line(&mut eval) {
        println!("Error occured in reading input: {}", error);
        return;
    }

    for i in eval.trim().split_whitespace() {
        sum += i.parse::<i32>()
            .expect("Expected that every string separated by whitespace will be a valid number");
    }

    println!("The sum of numbers {} is {}", eval, sum);
}

fn translate(){
    let mut input = String::new();
    
    loop {

        if let Err(error) = std::io::stdin().read_line(&mut input) {
            println!("Could not read input {}", error);
            break;
        }

        if input.trim().is_empty() {break;}
        
        println!("{}", input.to_uppercase());
    }
}