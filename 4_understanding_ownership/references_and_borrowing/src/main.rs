/*
1) At any given time, you can have either one mutable reference or any number of 
immutable references.

2) References must always be valid.
*/

fn main() {
    references();
    mutable_references();
    allowed_mutable_reference_scope();
    dangling_reference();
}

fn references() {
    /*
    Here is how you would define and use a calculate_length function that has a 
    reference to an object as a parameter instead of taking ownership of the 
    value. The ampersands are references, and they allow you to refer to some 
    value without taking ownership of it.
    */
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("In references, the length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    /*
    We call having references as function parameters borrowing. As in real 
    life, if a person owns something, you can borrow it from them. When you’re 
    done, you have to give it back.
    */
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

/* 
Un-commenting the following function will cause a compiler error. Just as 
variables are immutable by default, so are references. We’re not allowed to 
modify something we have a reference to.
*/
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("In mutable_references, s is {}.", s);

    let r1 = &mut s;
    println!("In mutable_references, r1 is {}.", r1);
    // Mutable references have one big restriction: you can have only one 
    // mutable reference to a particular piece of data in a particular scope.
    //  The following code will fail because r1 and r2 are still in scope (If 
    // r1 were removed from the println statement, r1 would be out of scope and
    // there would be no errors.):
    // let r2 = &mut s;
    // println!("In mutable_references, r1 is {} and r2 is {}.", r1, r2);

    // A similar rule exists for combining mutable and immutable references. 
    // We also cannot have a mutable reference while we have an immutable one. 
    // Users of an immutable reference don’t expect the values to suddenly 
    // change out from under them! This code results in an error:
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn allowed_mutable_reference_scope() {
    /*
    A reference's scope starts from where it is introduced and continues 
    through the last time that reference is used. For instance, this code will 
    compile because the last usage of the immutable references occurs before 
    the mutable reference is introduced:
    */
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("In allowed_mutable_reference_scope, r1 is {} and r2 is {}.", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("In allowed_mutable_reference_scope, r3 is {}.", r3);
}

fn dangling_reference() {
    /*
    In languages with pointers, it’s easy to erroneously create a dangling 
    pointer, a pointer that references a location in memory that may have been 
    given to someone else, by freeing some memory while preserving a pointer to 
    that memory. In Rust, by contrast, the compiler guarantees that references 
    will never be dangling references: if you have a reference to some data, 
    the compiler will ensure that the data will not go out of scope before the 
    reference to the data does.
    */
    // let reference_to_nothing = dangle();

    let s = no_dangle();
    println!("In dangling_reference, s is {}.", s);
}

/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}