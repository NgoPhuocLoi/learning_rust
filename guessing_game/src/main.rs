use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game !");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number: {secret_number}");
    println!("Input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Please type number!");
    println!("Your guess: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("Correct!"),
        Ordering::Greater => println!("Too big!")
    }
}
