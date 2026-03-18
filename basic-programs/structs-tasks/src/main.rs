mod struct_tasks;

use crate::struct_tasks::BankAccount;
use crate::struct_tasks::Counter;
use crate::struct_tasks::Point;

fn main() {
    bank_stuff();
    counter_stuff();
    point_stuff();
}

fn point_stuff() {
    let p1 = Point {
            x: 1,
            y: 11
        };

    let p2 = Point {
        x: 10,
        y: -54
    };

    println!("The distance between p1 {:?} and p2 {:?} is approximately {}", p1, p2, p1.distance(&p2).round());
}

fn bank_stuff() {
        let mut ba = BankAccount::new("John");

    ba.deposit(12);
    
    println!("{}'s bank account hold {}$", ba.owner(), ba.balance());

    let unsuccess = ba.widthdraw(20);

    println!("Operation of widthdraw 20$ was successful ? {}", unsuccess);

    let success = ba.widthdraw(10);
    
    println!("Operation of widthdraw 10$ was successful ? {}", success);
}

fn counter_stuff(){
        let mut counter = Counter::new();

    counter.increment();
    counter.increment();
    counter.increment();
    counter.increment();
    counter.increment();

    println!("The counter was incremented and now it is {}", counter.get());

    counter.decrement();

    println!("The counter was decremented and now it is {}", counter.get());

    counter.reset();

    println!("The counter was reseted and now it is {}", counter.get());
}