#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    /*
    The if let syntax lets you combine if and let into a less verbose way to 
    handle values that match one pattern while ignoring the rest. Consider 
    this code that matches on an Option<u8> value but only wants to execute 
    code if the value is 3.
    */
    // let some_u8_value = Some(0u8);
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Instead, we could write this in a shorter way using if let. 
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count is {}", count);

    let coin2 = Coin::Quarter(UsState::Alabama);
    let coin3 = Coin::Penny;
    let coin4 = Coin::Nickel;
    let coin5 = Coin::Dime;
    let mut count2 = 0;
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count2 += 1;
    }
    println!("count2 is {}", count2);

    println!("The values for coin3, coin4 and coin5 are: {:?}, {:?} and {:?}.", coin3, coin4, coin5)
}
