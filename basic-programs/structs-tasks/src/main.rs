mod struct_tasks;

use crate::struct_tasks::BankAccount;
use crate::struct_tasks::Counter;

fn main() {
    let mut ba = BankAccount::new("John");

    ba.deposit(12);
    
    println!("{}'s bank account hold {}$", ba.owner(), ba.balance());

    let unsuccess = ba.widthdraw(20);

    println!("Operation of widthdraw 20$ was successful ? {}", unsuccess);

    let success = ba.widthdraw(10);
    
    println!("Operation of widthdraw 10$ was successful ? {}", success);


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
