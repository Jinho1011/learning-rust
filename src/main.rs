extern crate rand;

use std::io::stdin;
use rand::{Rng, thread_rng};

fn main() {
    let mut guess = String::new();
    let secret_number = thread_rng().gen_range(1..101);

    stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let mut num = guess.trim().parse::<i32>().unwrap();

    println!("{}", secret_number);


    loop {
        if num < secret_number {
            println!("higher");
        } else if num > secret_number {
            println!("lower");
        } else {
            println!("correct!");
            break;
        }
        guess.clear();
        stdin().read_line(&mut guess)
            .expect("Failed to read line");
        num = guess.trim().parse().unwrap();
    }
}
