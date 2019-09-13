fn main() {
    if_let_expressions();
    while_loop();
    for_loop();
    let_statement();
    function_parameters();
}

/*
The line if let Ok(age) = age introduces a new shadowed age variable that 
contains the value inside the Ok variant. This means we need to place the if age
> 30 condition within that block: we can’t combine these two conditions into if 
let Ok(age) = age && age > 30. The shadowed age we want to compare to 30 isn’t 
valid until the new scope starts with the curly bracket.

The downside of using if let expressions is that the compiler doesn’t check 
exhaustiveness, whereas with match expressions it does. If we omitted the last 
else block and therefore missed handling some cases, the compiler would not 
alert us to the possible logic bug.
*/
fn if_let_expressions() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("if_let_expressions: Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("if_let_expressions: Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("if_let_expressions: Using purple as the background color");
        } else {
            println!("if_let_expressions: Using orange as the background color");
        }
    } else {
        println!("if_let_expressions: Using blue as the background color");
    }
}

fn while_loop() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("while_loop: {}", top);
    }    
}

fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("for_looop: {} is at index {}", value, index);
    }    
}

fn let_statement() {
    let (x, y, z) = (1, 2, 3);
    println!("let_statemnt: x={}, y={}, z={}.", x, y, z);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location from function_parameters/print_coordinates: ({}, {})", x, y);
}

fn function_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
}