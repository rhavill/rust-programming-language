use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;
use std::fs::File;

fn main() {
    loop {
        println!("Input a number and press <Enter>:");
        println!("1 - to see a generic error crash.");
        println!("2 - to match error and prevent crash.");
        println!("3 - to catch the error with unwrap_or_else and prevent crash.");
        println!("4 - to unwrap error and crash.");
        println!("5 - to crash with expect.");
        println!("6 - to propogate errors.");
        println!("7 - to propogate errors with ? operator.");
        println!("8 - to propogate errors with chained ? operator.");
        println!("9 - to propogate errors with fs::read_to_string.");
        println!("10 - to exit.");
        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match option {
            1 => generic_error(),
            2 => matching_error(),
            3 => avoid_match_with_closure(),
            4 => unwrap_crash(),
            5 => expect_crash(),
            6 => propogating_errors(),
            7 => question_mark_propogation(),
            8 => chained_question_mark_propogation(),
            9 => fs_read_to_string_propogation(),
            _   => break,
        }
        // delete hello.txt file if it exists
        // let f = fs::remove_file("hello.txt");
        // match f {
        //     Ok(file) => file,
        //     Err(_error) => {},
        // };
    }
}

#[allow(unused_variables)]
fn generic_error() {
    /*
    If we give f a type annotation that we know is not the return type of the 
    function and then try to compile the code, the compiler will tell us that 
    the types don’t match. The error message will then tell us what the type of 
    f is.
    */
    // let f: u32 = File::open("hello.txt");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

#[allow(unused_variables)]
fn matching_error() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };    
}

#[allow(unused_variables)]
fn avoid_match_with_closure() {
    /*
    In Chapter 13, you’ll learn about closures; the Result<T, E> type has many 
    methods that accept a closure and are implemented using match expressions. 
    Using those methods will make your code more concise. In Chapter 13, you’ll 
    learn about closures; the Result<T, E> type has many methods that accept a 
    closure and are implemented using match expressions. Using those methods 
    will make your code more concise. 
    */
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });   
}

#[allow(unused_variables)]
fn unwrap_crash() {
    /*
    unwrap is a shortcut method that is implemented just like the match 
    expression we wrote in Listing 9-4. If the Result value is the Ok variant, 
    unwrap will return the value inside the Ok. If the Result is the Err 
    variant, unwrap will call the panic! macro for us.
    */
    let f = File::open("hello.txt").unwrap();
}

#[allow(unused_variables)]
fn expect_crash() {
    /*
    Expect, which is similar to unwrap, lets us also choose the panic! error 
    message. Using expect instead of unwrap and providing good error messages 
    can convey your intent and make tracking down the source of a panic easier.
    */
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propogating_errors() {
    let username = read_username_from_file();
    match username {
        Ok(username) => println!("Got username: {}", username),
        Err(_error) => println!("Error getting usename, using default."),
    }; 
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn question_mark_propogation() {
    let username = read_username_from_file_shorthand();
    match username {
        Ok(username) => println!("Got username: {}", username),
        Err(_error) => println!("Error getting usename, using default."),
    };
}

fn read_username_from_file_shorthand() -> Result<String, io::Error> {
    /*
    If the value of the Result is an Ok, the value inside the Ok will get 
    returned from this expression, and the program will continue. If the value 
    is an Err, the Err will be returned from the whole function as if we had 
    used the return keyword so the error value gets propagated to the calling 
    code. 
    
    This is different from using match, because error values that have 
    the ? operator called on them go through the from function, defined in the 
    From trait in the standard library, which is used to convert errors from one
    type into another. When the ? operator calls the from function, the error 
    type received is converted into the error type defined in the return type of 
    the current function. This is useful when a function returns one error type 
    to represent all the ways a function might fail, even if parts might fail 
    for many different reasons. As long as each error type implements the from 
    function to define how to convert itself to the returned error type, the ? 
    operator takes care of the conversion automatically. 
    
    The ? at the end of the File::open call will return the value inside an Ok 
    to the variable f. If an error occurs, the ? operator will return early out 
    of the whole function and give any Err value to the calling code. The same 
    thing applies to the ? at the end of the read_to_string call.
    */
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn chained_question_mark_propogation() {
    let username = read_username_from_file_shorterhand();
    match username {
        Ok(username) => println!("Got username: {}", username),
        Err(_error) => println!("Error getting usename, using default."),
    };
}

fn read_username_from_file_shorterhand() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn fs_read_to_string_propogation() {
    let username = read_username_from_file_shortesthand();
    match username {
        Ok(username) => println!("Got username: {}", username),
        Err(_error) => println!("Error getting usename, using default."),
    };
}

fn read_username_from_file_shortesthand() -> Result<String, io::Error> {
    // The fs::read_to_string function that opens the file, creates a new 
    // String, reads the contents of the file, puts the contents into that 
    // String, and returns it. 
    fs::read_to_string("hello.txt")
}

/*
The main function is special, and there are restrictions on what its return type
must be. One valid return type for main is (), and conveniently, another valid 
return type is Result<T, E>, as shown here:
*/
// use std::error::Error;
// use std::fs::File;
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }
