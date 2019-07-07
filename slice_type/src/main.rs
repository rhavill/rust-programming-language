/*
The concepts of ownership, borrowing, and slices ensure memory safety in Rust 
programs at compile time. The Rust language gives you control over your memory 
usage in the same way as other systems programming languages, but having the 
owner of data automatically clean up that data when the owner goes out of scope
means you don’t have to write and debug extra code to get this control.
*/

fn main() {
    string_position();
    string_slice();
    string_literals_are_slices();
    string_slice_parameters();
    other_slices();
}

fn string_position() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("In string_position, first word is {}.", &s[0..word]);
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // The following line passes compilation, but fails at runtime, ecause word 
    // isn’t connected to the state of s at all, word still contains the value 
    // 5.:
    // println!("In string_position, first word is {}.", &s[0..word]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slice() {
let mut s = String::from("hello world");

    let word = first_word_slice(&s); // word will get the value 5
    println!("In string_slice, first word is {}.", word);
    s.clear(); // this empties the String, making it equal to ""
    /*
    Using slice causes our error on the next line to fail at compile time
    instead of run time, because if we have an immutable reference to 
    something, we cannot also take a mutable reference. Because clear needs to
    truncate the String, it needs to get a mutable reference. Rust disallows 
    this, and compilation fails.
    */
    // println!("In string_slice, first word is {}.", word);
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_literals_are_slices() {
    /*
    The type of s here is &str: it’s a slice pointing to that specific point of
    the binary. This is also why string literals are immutable; &str is an 
    immutable reference.
    */
    let s = "Hello, world!";
    println!("in string_literals_are_slices, s is {}.", s);
}

/*
Knowing that you can take slices of literals and String values leads us to one 
more improvement on first_word, and that’s its signature:
*/
fn first_word_slice_parameters(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn string_slice_parameters() {
    /*
    If we have a string slice, we can pass that directly. If we have a String, 
    we can pass a slice of the entire String. Defining a function to take a 
    string slice instead of a reference to a String makes our API more general 
    and useful without losing any functionality:
    */
    let my_string = String::from("hello world");
    println!("in string_slice_parameters, my_string is {}.", my_string);

    // first_word_slice_parameters works on slices of `String`s
    let word = first_word_slice_parameters(&my_string[..]);
    println!("in string_slice_parameters, word is {}.", word);

    let my_string_literal = "hello world";

    // first_word_slice_parameters works on slices of string literals
    let word = first_word_slice_parameters(&my_string_literal[..]);
    println!("in string_slice_parameters, word is {}.", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_parameters(my_string_literal);
    println!("in string_slice_parameters, word is {}.", word);
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    /*
    This slice has the type &[i32]. It works the same way as string slices do, 
    by storing a reference to the first element and a length.
    */
    let slice = &a[1..3];
    println!("In other_slices, slice is [{},{}].", slice[0], slice[1]);
}