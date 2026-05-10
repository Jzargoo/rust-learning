fn main() {
    println!("Hello, world!");
}

fn make_boxed_array(n: usize) -> Box<[u64]> {
    let mut array:Box<[std::mem::MaybeUninit<u64>]> = Box::new_uninit_slice(n);
    for i in 0..n {
        array[i].write(i as u64);
    }

    unsafe  {
        return array.assume_init();
    }
}