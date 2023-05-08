extern crate rand;

use std::cmp::Ordering;
use std::io::stdin;
use rand::{Rng, thread_rng};

fn main() {
    let secret_number = thread_rng().gen_range(1..101);

    loop {
        print!("input your guess: ");

        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = guess.trim().parse().unwrap();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
