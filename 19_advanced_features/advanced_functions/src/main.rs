fn main() {
    function_pointers();
    closure_traits();
    initializer_functions();
    returning_closures();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/*
Functions coerce to the type fn (with a lowercase f), not to be confused with 
the Fn closure trait. The fn type is called a function pointer. 
*/
fn function_pointers() {
    let answer = do_twice(add_one, 5);
    println!("function_pointers: The answer is: {}", answer);
}

/*
Unlike closures, fn is a type rather than a trait, so we specify fn as the 
parameter type directly rather than declaring a generic type parameter with one 
of the Fn traits as a trait bound.

Function pointers implement all three of the closure traits (Fn, FnMut, and 
FnOnce), so you can always pass a function pointer as an argument for a function 
that expects a closure. It’s best to write functions using a generic type and 
one of the closure traits so your functions can accept either functions or 
closures.
*/
fn closure_traits() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        // We could use a closure, like this:
        // .map(|i| i.to_string())
        // Or we could name a function as the argument to map instead of the 
        // closure, like this:
        .map(ToString::to_string)
        .collect();

    println!("closure_traits list_of_strings={:?}.", list_of_strings);
}

fn initializer_functions() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20)
        .map(Status::Value)
        .collect();    

    println!("initializer_functions list_of_statuses={:?}.", list_of_statuses);
}

/*
This function causes compiler error: "the size for values of type `(dyn 
std::ops::Fn(i32) -> i32 + 'static)` cannot be known at compilation time". 
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
*/

/*
The error above references the Sized trait again! Rust doesn’t know how much 
space it will need to store the closure. We saw a solution to this problem 
earlier. We can use a trait object:
*/
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returning_closures() {
    let plus_one = returns_closure();
    println!("returning_closures: plus_one(1)={}.", plus_one(1));
}