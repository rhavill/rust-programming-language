fn main() {
    simple_match();
    bind_value_match();
    option_match();
    underscore_placeholder();
}

fn simple_match() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }    
    println!("In simple_match, a penny is worth {} cent(s).", value_in_cents(Coin::Penny));
    println!("In simple_match, a nickel is worth {} cent(s).", value_in_cents(Coin::Nickel));
    println!("In simple_match, a dime is worth {} cent(s).", value_in_cents(Coin::Dime));
    println!("In simple_match, a quarter is worth {} cent(s).", value_in_cents(Coin::Quarter));
}

fn bind_value_match() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    println!("In bind_value_match, a penny is worth {} cent(s).", value_in_cents(Coin::Penny));
    println!("In bind_value_match, a nickel is worth {} cent(s).", value_in_cents(Coin::Nickel));
    println!("In bind_value_match, a dime is worth {} cent(s).", value_in_cents(Coin::Dime));
    println!("In bind_value_match, an Alabama quarter is worth {} cent(s).", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("In bind_value_match, an Alaska quarter is worth {} cent(s).", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn option_match() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("In option_match, five is {:?}.", five);
    println!("In option_match, six is {:?}.", six);
    println!("In option_match, none is {:?}.", none);

    /*
    This version of our plus_one function that has a bug and won’t compile (We 
    didn’t handle the None case, so this code will cause a bug):
    */
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
}

fn underscore_placeholder() {
    // The _ pattern will match any value.
    let some_u8_value = 0u8;
    
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
