/*
Ownership Rules
1) Each value in Rust has a variable that’s called its owner.
2) There can only be one owner at a time.
3) When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    simple_scope();
    copy_bind();
    move_example();
    clone_example();
    stack_only_copy();
    ownership_and_functions();
    return_values_and_scope();
    return_value_ownership_transfer();
}

fn simple_scope() { // s is not valid here, it’s not yet declared
                        
    let s = "hello";   // s is valid from this point forward

    println!("The value of s in simple_scope is {}.", s);

} // this scope is now over, and s is no longer valid

fn copy_bind() {
    /*
    Bind the value 5 to x; then make a copy of the value in x and bind it to y.
    We now have two variables, x and y, and both equal 5. This is indeed what
    is happening, because integers are simple values with a known, fixed size,
    and these two 5 values are pushed onto the stack.
    */
    let x = 5;
    let y = x;

    println!("In copy_bind, x is {} and y is {}.", x, y);
}

fn move_example() {
    /*
    Because Rust also invalidates the first variable, instead of being called a
    shallow copy, it’s known as a move. In this example, we would say that s1 
    was moved into s2. With only s2 valid, when it goes out of scope, it alone
    will free the memory. This allows us to avoid a problem that occurs in
    other languages (like C): when s2 and s1 go out of scope, they will both 
    try to free the same memory. This is known as a double free error. Rust
    avoids the double free error by invalidating the first variable with a
    "move".
    */
    let s1 = String::from("hello");
    let s2 = s1;

    println!("In move_example, s2 is {}.", s2);
    // Uncommenting the next println line results in compiler error, because s1 
    // was invalidated by a "move".
    // println!("{}, world!", s1);
}

fn clone_example() {
    /*
    If we do want to deeply copy the heap data of the String, not just the 
    stack data, we can use a common method called clone. 
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // The next line works just fine, because the heap data does get copied.
    println!("In clone_example, s1 is {} and s2 is {}.", s1, s2);
}

fn stack_only_copy() {
    /*
    Types such as integers that have a known size at compile time are stored 
    entirely on the stack, so copies of the actual values are quick to make.
    There’s no difference between deep and shallow copying here, so calling 
    clone wouldn’t do anything different from the usual shallow copying and we 
    can leave it out.
    */
    let x = 5;
    let y = x;

    println!("In stack_only_copy, x = {}, y = {}.", x, y);
}

fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("In ownership_and_functions, x is {}.", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("In takes_ownership, some_string is {}.", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("In makes_copy, some_integer is {}.", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn return_value_ownership_transfer() {
    /*
    Taking ownership and then returning ownership with every function is a bit 
    tedious. What if we want to let a function use a value but not take 
    ownership? It’s quite annoying that anything we pass in also needs to be 
    passed back if we want to use it again, in addition to any data resulting 
    from the body of the function that we might want to return as well.

    It’s possible to return multiple values using a tuple. But this is too much 
    ceremony and a lot of work for a concept that should be common. Luckily for 
    us, Rust has a feature for this concept, called references.
    */
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("In return_value_ownership_transfer, the length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}