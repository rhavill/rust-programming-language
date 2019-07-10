fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("{}", fahrenheit_to_celsius(32.0));
    println!("{}", fahrenheit_to_celsius(78.0));
    println!("{}", celsius_to_fahrenheit(0.0));
    println!("{}", celsius_to_fahrenheit(22.0));

    println!("fib 5 = {}", fib(5));

    days_of_xmas();

}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fib(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n * fib (n - 1)
    }
}

fn ordinal_indicator(n: u64) -> String {
    let str: String = match n {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th"),
    };
    str
}

fn days_of_xmas() {
    let days = [
        "a partridge in a pear tree.",
        "two turtle doves and",
        "three french hens,",
        "four calling birds,",
        "five golden rings,",
        "six gees a laying,",
        "seven swans a swimming,",
        "eight maids a milking,",
        "nine ladies dancing,",
        "ten lords a leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming"
    ];

    for day in 1..13 {
        let ord = ordinal_indicator(day);
        println!("On the {}{} day of xmas, my true love sent to me:", day, ord);
        for gift in (1..day + 1).rev() {
            let i = gift as usize;
            println!("{}", days[i - 1]);
        }
        println!("");
    }
}