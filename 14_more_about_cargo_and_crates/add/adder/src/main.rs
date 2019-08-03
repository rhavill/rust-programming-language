use add_one;
use add_two;

/*
To run the binary crate from the add directory, we need to specify which package 
in the workspace we want to use by using the -p argument and the package name 
with cargo run (from the top-level add directory):

$ cargo run -p adder
*/
fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("Hello, world! {} plus two is {}!", num, add_two::add_two(num));
}