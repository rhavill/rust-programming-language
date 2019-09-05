/*
Boxes allow you to store data on the heap rather than the stack. What remains 
on the stack is the pointer to the heap data. 

Boxes don’t have performance overhead, other than storing their data on the 
heap instead of on the stack. But they don’t have many extra capabilities 
either. You’ll use them most often in these situations:

1) When you have a type whose size can’t be known at compile time and you want 
to use a value of that type in a context that requires an exact size.

2) When you have a large amount of data and you want to transfer ownership but 
ensure the data won’t be copied when you do so.

3) When you want to own a value and you care only that it’s a type that 
implements a particular trait rather than being of a specific type.

*/


// Compilation of this fails with the error: recursive type `List` has 
// infinite size.
// enum List {
//     Cons(i32, List),
//     Nil,
// }
// To get rid of the error, change the data structure to store the value 
// indirectly by storing a pointer to the value instead.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    box_syntax();
    recursive_types();
}

/*
Just like any owned value, when a box goes out of scope, as b does at the end 
of main, it will be deallocated.

Putting a single value on the heap isn’t very useful, so you won’t use boxes by 
themselves in this way very often. Having values like a single i32 on the stack, 
where they’re stored by default, is more appropriate in the majority of 
situations. 
*/
fn box_syntax() {
    let b = Box::new(5);
    println!("In box_syntax, b = {}", b);
}

/*
At compile time, Rust needs to know how much space a type takes up. One type 
whose size can’t be known at compile time is a recursive type, where a value 
can have as part of itself another value of the same type. Because this nesting 
of values could theoretically continue infinitely, Rust doesn’t know how much 
space a value of a recursive type needs. However, boxes have a known size, so by 
inserting a box in a recursive type definition, you can have recursive types.

Most of the time when you have a list of items in Rust, Vec<T> is a better 
choice to use. Other, more complex recursive data types are useful in various 
situations, but by starting with the cons list, we can explore how boxes let us 
define a recursive data type without much distraction.
*/
fn recursive_types() {
   
    // Compilation of this fails with the error: recursive type `List` has 
    // infinite size (See definition of List above).
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // To get rid of the error, change the data structure to store the value 
    // indirectly by storing a pointer to the value instead.
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("In recursive_types, list is  {:?}", list);
}