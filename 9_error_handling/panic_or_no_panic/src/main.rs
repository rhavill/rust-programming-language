/*
Rust’s error handling features are designed to help you write more robust code. 
The panic! macro signals that your program is in a state it can’t handle and 
lets you tell the process to stop instead of trying to proceed with invalid or 
incorrect values. The Result enum uses Rust’s type system to indicate that 
operations might fail in a way that your code could recover from. You can use 
Result to tell code that calls your code that it needs to handle potential 
success or failure as well. Using panic! and Result in the appropriate 
situations will make your code more reliable in the face of inevitable 
problems.
*/

fn main() {
    validation();
}

/*
This is not an ideal solution: if it was absolutely critical that the program 
only operated on values between 1 and 100, and it had many functions with this 
requirement, having a check like this in every function would be tedious (and 
might impact performance).

loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}
*/

/*
Instead, we can make a new type and put the validations in a function to create 
an instance of the type rather than repeating the validations everywhere. That 
way, it’s safe for functions to use the new type in their signatures and 
confidently use the values they receive.
*/
fn validation() {
    #[derive(Debug)]
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // The next line causes compilation to fail, because the value fails validation:
    // let guess = Guess::new(101);
    let guess = Guess::new(100);
    println!("Guess is {:?}", guess.value());
}