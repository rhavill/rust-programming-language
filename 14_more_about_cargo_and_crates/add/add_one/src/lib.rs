//! # add_one
//!
//! `add_one` is a collection of utilities that only contains one simple 
//! function: add_one.

#[allow(unused_imports)]
use rand;

/*
We can generate the HTML documentation from this documentation comment by 
running cargo doc. This command runs the rustdoc tool distributed with Rust and 
puts the generated HTML documentation in the target/doc directory.

For convenience, running cargo doc --open will build the HTML for your current 
crate’s documentation (as well as the documentation for all of your crate’s 
dependencies) and open the result in a web browser. 

Another style of doc comment, //!, adds documentation to the item that contains 
the comments rather than adding documentation to the items following the 
comments. We typically use these doc comments inside the crate root file 
(src/lib.rs by convention) or inside a module to document the crate or the 
module as a whole.
*/


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}