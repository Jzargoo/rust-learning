use std::io;
fn main() {
    println!("Enter the measurment(c/f), standard is (c)elsius");
    
    let mut  measur= String::new();
    
    io::stdin()
    .read_line(&mut measur)
    .expect("cannot read a line from standard input");

    let measur: bool =  ! (measur.starts_with("f") && measur.len() == 1);

    let mut temperature = String::new();
    let temperature: i32 = loop{

        println!("Enter the value of temperature");
        
        io::stdin()
        .read_line(&mut temperature)
        .expect("cannot read a line from standard input");
        
        if temperature.trim().parse::<i32>().is_ok() {
            break temperature.trim().parse().expect("Please type a number")
        }
        
        println!("Please type a number")
    };

    println!("The converted value from {} in {} to {} in {}",
        temperature, 
        if measur {"Celsius"} else {"fahrenheit"}, 
        determine_converted_value(temperature, measur),
        if measur {"fahrenheit"} else {"Celsius"}
    )
}

fn determine_converted_value(x: i32, is_celsius :bool) -> i32 {
    if is_celsius {
        (x as f32 * 1.8) as i32 + 32
    } else {
        (x as f32 / 1.8) as i32 - 32
    }
}
