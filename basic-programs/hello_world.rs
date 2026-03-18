fn main (){
    let mut x: u32 = 18;
    println!("X is {x}");
    x += 10;
    println!("X is {x}");
    let mut tup: (i32, char, bool) = (15, 'f', false);
    let (x,y,z) = tup;
    tup.1='a';
    println!("tup elements are {x},{y},{z}");
    println!("tup elements are {0},{1},{2}", tup.0, tup.1, tup.2);


    println!("{}",arr);
    let arr: [bool; 5] = [false,true,false,true, false];
    println!("{}",arr);
    let arr: [bool; 5] = [false; 5];
    println!("{}",arr);
    

}
