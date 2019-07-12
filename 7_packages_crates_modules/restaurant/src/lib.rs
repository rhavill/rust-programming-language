// These two use statements we had in Listing 2-4 in the Guessing Game bring 
// items from std into scope:
// use std::cmp::Ordering;
// use std::io;
// Instead, we can use nested paths to bring the same items into scope in one 
// line: 
// use std::{cmp::Ordering, io};

// We can also replace these two use statements:
// use std::io;
// use std::io::Write;
// With this use statement:
// use std::io::{self, Write};

// If we want to bring all public items defined in a path into scope, we can 
// specify that path followed by *, the glob operator:
// use std::collections::*;

mod front_of_house;

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// Specifying an absolute path with use.
// use crate::front_of_house::hosting;
// Specifying a relative path with use.
// use self::front_of_house::hosting;
// This technique is called re-exporting because we’re bringing an item 
// into scope but also making that item available for others to bring into
//  their scope.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}