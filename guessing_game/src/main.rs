use std::{io, cmp::Ordering};
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); 
    println!("The secret number is: {secret_number}");

    loop {
        println!("please input your guess");

        // get guess from cli
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //convert the guess to an num
        let guess: u32 = guess.trim().parse().expect("Please enter an number");
        println!("You guessed: {guess}");

        // compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too Big")
        }
        
    }

}
