use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let min = 1;
    let max = 10;

    println!("Guess the number from {} to {}!", min, max);

    let secret_number = rand::thread_rng().gen_range(min, max + 1);

    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
        // Instead of allowing the program to crash with a custom error on invalid input,
        // we can just ignore the invalid input.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}