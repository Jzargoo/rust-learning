use std::collections::HashMap;


fn main() {
    let h = vec![1,2,3,5,5,5];
    let opt = median(&h);
    if opt.is_some() {
        println!("The median of the vector {}", opt.unwrap());
    } else {
        println!("The vector is empty");
    }

    let opt_mode = mode(&h);
    
    if opt_mode.is_some() {
        println!("A mode of the vector is {}", opt_mode.unwrap());
    } else {
        println!("The vector is empty");
    }
}

fn median(list: &Vec<i32>) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }
    let mut copy = list.clone();
    
    copy.sort();

    return Some(copy[list.len() / 2]);
}


fn mode (list: &Vec<i32>) -> Option<i32> {
    let mut count_map:HashMap<i32, u32> = HashMap::new();

    let mut top_element = None;

    let mut top_count = 0;

    for &el in list.iter() {
        let coe = count_map.entry(el).or_insert(0);

        *coe += 1;
        if *coe > top_count {
            top_element = Some(el);
            top_count = *coe;
        }
    }

    top_element
}