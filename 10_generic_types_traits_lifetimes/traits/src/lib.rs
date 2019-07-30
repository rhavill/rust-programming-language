// This version of the Summary trait has a method signature, instead of a 
// default implementaion of the summarize method:
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// This version of the Summary trait has a default implementaion of the 
// summarize method:
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This allows us to use te default implementation of summarize:
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// Uncommenting this would override the default implementation of summarize:
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
The trait bound syntax can express more complexity in other cases. If we wanted 
to force both parameters to have the same type, that’s only possible to express 
using a trait bound, like this:
*/
pub fn bound_syntax_notify<T: Summary>(item1: T, item2: T) {
    println!("bound_syntax_notify: Two Breaking newses! {} ... {}", item1.summarize(), item2.summarize());
}

pub trait TextDisplay {
    fn display(&self);
}

pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl Summary for Email {
    fn summarize_author(&self) -> String {
        format!("From: {}", self.from)
    }
}

impl TextDisplay for Email {
    fn display(&self) {
        println!("{}", self.body);
    }
}

// Multiple trait bounds: : we specify in the notify definition that item must 
// implement both Display and Summary.
pub fn multiple_trait_notify(item: impl Summary + TextDisplay) {
    println!("multiple_trait_notify:");
    item.display();
}

// The + syntax is also valid with trait bounds on generic types:
// pub fn multiple_trait_notify<T: Summary + Display>(item: T) {}



pub fn multiple_trait_where_clause<T, U>(t: T, u: U)
    where T: Summary,
          U: Summary + TextDisplay
{
    println!("multiple_trait_where_clause: {}...{}", t.summarize(), u.summarize());
    u.display();
}

/*
By using a trait bound with an impl block that uses generic type parameters, we 
can implement methods conditionally for types that implement the specified 
traits. For example, the type Pair<T> in Listing 10-16 always implements the new 
function. But Pair<T> only implements the cmp_display method if its inner type T 
implements the PartialOrd trait that enables comparison and the Display trait 
that enables printing.
*/
use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
We can also conditionally implement a trait for any type that implements another 
trait. Implementations of a trait on any type that satisfies the trait bounds 
are called blanket implementations and are extensively used in the Rust standard 
library. For example, the standard library implements the ToString trait on any 
type that implements the Display trait. The impl block in the standard library 
looks similar to this code:


impl<T: Display> ToString for T {
    // --snip--
}
Because the standard library has this blanket implementation, we can call the 
to_string method defined by the ToString trait on any type that implements the 
Display trait. For example, we can turn integers into their corresponding String 
values like this because integers implement Display:

let s = 3.to_string();
*/