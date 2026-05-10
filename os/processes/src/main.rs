use std::os::raw::c_void;

use libc::{O_RDWR, fork, nanosleep, open, timespec, write, getpid};

fn task_2 () {
    unsafe {
        let d = open("/home/jzargo/rust-applications/os/processes/race.txt\0".as_ptr() as *const i8, O_RDWR);

        let pid = fork();

        if pid > 0 {
            let parent = "parent";

            println!("File descriptor from parent: {}", d);

            write(d, parent.as_ptr() as *const c_void, parent.len());
        } else if pid==0 {
            let child = "child";

            println!("File descriptor from child: {}", d);
            
            write(d, child.as_ptr() as *const c_void, child.len());
        }
    }
}

fn task_1() {
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


fn task_3() {
    let pid;
    
    unsafe {
        pid = fork();
    }

    if pid == 0 {
        
        println!("Hello!")
        
        } else if pid > 0 {
        
            let time = timespec{
                tv_sec: 2,
                tv_nsec: 1000
            };

            let mut def_time = timespec {
                tv_nsec: 0,
                tv_sec: 0
            }; 

            unsafe {
                nanosleep(&time, &mut def_time);
            }
            
            println!("Goodbye!")
    }
}

pub fn main() {
    
    task_3();

    
}