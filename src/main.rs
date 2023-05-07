extern crate rand;

use std::io::stdin;
use rand::Rng;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);

    stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("{}", secret_number);

    while let Ok(num) = guess.trim().parse::<i32>() {
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
    }
}
