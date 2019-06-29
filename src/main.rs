use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Secret Number is {}", secret_number);
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read");
    let guess: u32 = guess.trim().parse().expect("Please Type a Number!");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You win"),
    }
}
