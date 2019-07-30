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