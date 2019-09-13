fn main() {
    matching_literals();
    matching_named_variables();
    multiple_patterns();
    matching_ranges();
    destructuring_structs();
    destructuring_enums();
    nested_structs_and_enums();
    destructuring_structs_and_tuples();
    ignoring_an_entire_value();
    ignoring_parts_of_a_value();
    ignore_named_var_with_underscore();
    ignoring_remaining_parts();
    match_guards();
    bindings();
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("matching_literals: one"),
        2 => println!("matching_literals: two"),
        3 => println!("matching_literals: three"),
        _ => println!("matching_literals: anything"),
    }    
}

/*
Named variables are irrefutable patterns that match any value, and we’ve used 
them many times in the book. However, there is a complication when you use 
named variables in match expressions. Because match starts a new scope, 
variables declared as part of a pattern inside the match expression will shadow 
those with the same name outside the match construct, as is the case with all 
variables. 
*/
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("matching_named_variables: Got 50"),
        /*
        Because we’re in a new scope inside the match expression, this is a new 
        y variable, not the y we declared at the beginning with the value 10. 
        This new y binding will match any value inside a Some, which is what we 
        have in x.
        */
        Some(y) => println!("matching_named_variables: matched, y = {:?}", y),
        _ => println!("matching_named_variables: default case, x = {:?}", x),
    }

    println!("matching_named_variables: at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("multiple_patterns: one or two"),
        3 => println!("multiple_patterns: three"),
        _ => println!("multiple_patterns: anything"),
    }    
}

fn matching_ranges() {
    let x = 5;
    match x {
        1...5 => println!("matching_ranges: one through five"),
        _ => println!("matching_ranges: something else"),
    }    

    let x = 'c';
    match x {
        'a'...'j' => println!("matching_ranges: early ASCII letter"),
        'k'...'z' => println!("matching_ranges: late ASCII letter"),
        _ => println!("matching_ranges: something else"),
    }    
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("destructuring_structs: a={}, b={}.", a, b);

    /*
    Because having variable names match the fields is common and because writing 
    let Point { x: x, y: y } = p; contains a lot of duplication, there is a 
    shorthand for patterns that match struct fields: you only need to list the 
    name of the struct field, and the variables created from the pattern will 
    have the same names.
    */  
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    println!("destructuring_structs: x={}, y={}.", x, y);

    /*
    We can also destructure with literal values as part of the struct pattern 
    rather than creating variables for all the fields.
    */
    match p {
        Point { x, y: 0 } => println!("destructuring_structs: On the x axis at {}", x),
        Point { x: 0, y } => println!("destructuring_structs: On the y axis at {}", y),
        Point { x, y } => println!("destructuring_structs: On neither axis: ({}, {})", x, y),
    }
}

fn destructuring_enums() {
    
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }   

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("destructuring_enums: The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "destructuring_enums: Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("destructuring_enums: Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "destructuring_enums: Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    } 
}

fn nested_structs_and_enums() {
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }    
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "nested_structs_and_enums: Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "nested_structs_and_enums: Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }    
}

fn destructuring_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    println!("destructuring_structs_and_tuples: feet={}, inches={}, x={}, y={}.", feet, inches, x, y);
}

fn foo(_: i32, y: i32) {
    println!("ignoring_an_entire_value(foo): This code only uses the y parameter: {}", y);
}

fn ignoring_an_entire_value() {
    foo(3, 4);
}

fn ignoring_parts_of_a_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("ignoring_parts_of_a_value: Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("ignoring_parts_of_a_value: setting is {:?}", setting_value); 

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("ignoring_parts_of_a_value: Some numbers: {}, {}, {}", first, third, fifth)
        },
    }   
}

fn ignore_named_var_with_underscore() {
    let s = Some(String::from("Hello!"));
    /*
    An unused variable starting with an underscore still binds the value, which 
    might take ownership of the value. The next line causes compiler error: "
    borrow of moved value: `s`". because the s value will still be moved 
    into _s, which prevents us from using s again. 
    */
    // if let Some(_s) = s {
    /*
    Using the underscore by itself doesn’t ever bind to the value. The next line
    will compile without any errors because s doesn’t get moved into "_".
    */
    if let Some(_) = s {
        println!("ignore_named_var_with_underscore: found a string");
    }

    println!("ignore_named_var_with_underscore: s={:?}", s);  
}

fn ignoring_remaining_parts() {
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("ignoring_remaining_parts: x is {}", x),
    }    

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("ignoring_remaining_parts: Some numbers: {}, {}", first, last);
        },
    }
}

/*
A match guard is an additional if condition specified after the pattern in a 
match arm that must also match, along with the pattern matching, for that arm to 
be chosen. 
*/
fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("match_guards: num less than five: {}", x),
        Some(x) => println!("match_guards: num {}", x),
        None => (),
    }    

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("match_guards: Got 50"),
        Some(n) if n == y => println!("match_guards: Matched, n = {:?}", n),
        _ => println!("match_guards: Default case, x = {:?}", x),
    }

    println!("match_guards: at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        /*
        The match condition states that the arm only matches if the value of x 
        is equal to 4, 5, or 6 and if y is true.
        */
        4 | 5 | 6 if y => println!("match_guards: 4, 5, or 6? yes"),
        _ => println!("match_guards: 4, 5, or 6? no"),
    }
}

/*
The at operator (@) lets us create a variable that holds a value at the same 
time we’re testing that value to see whether it matches a pattern. Using @ lets 
us test a value and save it in a variable within one pattern.
*/
fn bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}