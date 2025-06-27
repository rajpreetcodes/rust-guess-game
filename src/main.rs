use std::io;
use rand::Rng;
fn main() {
    println!("I will guess your number!");
    println!("Enter a number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut rng = rand::thread_rng();
    let guess: i32 = rng.gen_range(1..=100);
    println!("I guess your number is: {}", guess);
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    println!("You entered: {}", number);
}
