use libc::{fork, getpid};


fn first() {
    println!("Hello, world!");
    let mut x = 100;
    let pid;
    unsafe {
        pid = fork();
    }

    if pid < 0 {
        panic!("There was a bug while fork was being invoked, pid {}", pid)
    } else if pid == 0 {
        unsafe {
            println!("Previous x is {} from {}", x, getpid());
            x = 150;
            println!("Now x is {} from {}", x, getpid());
        }
    } else {
        unsafe {
            println!("X is {} from {}", x, getpid());
            x = 120;
            println!("Now x is {} from {}", x, getpid());
        }
    }
}
