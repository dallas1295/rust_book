use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // let's take a users input into consideration and generate the fibonacci numbers to the nth
    // value give from the users.

    println!("Welcome to the fibonacci number counter!");
    println!("Please input to what nth you'd like to generate until.");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading line.");

    println!(
        "You've selected {} as the number you'd like to count until",
        n
    );

    println!("Okay let's count!");
    thread::sleep(Duration::from_secs(1));

    let n: u64 = n.trim().parse().expect("Please enter a valid number");

    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    for _ in 0..n {
        let next = &a + &b;
        println!("{}", a);

        a = b;
        b = next;
    }
}
