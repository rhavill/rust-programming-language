use traits::*;

fn main() {
    implement_trait();
    default_implementation();
    trait_as_parameter();
    bound_syntax_parameters();
    multiple_trait_bounds();
    where_clause();
    return_implementation();
    get_largest();
    trait_bond_conditional_methods();
}

fn implement_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("In implement_trait: 1 new tweet: {}", tweet.summarize());    
}

fn default_implementation() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("default_implementation: New article available! {}", article.summarize());
}

fn trait_as_parameter() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    println!("trait_as_parameter");
    notify(article);    
}

fn bound_syntax_parameters() {
    let tweet1 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("dog_ebooks"),
        content: String::from("woof!"),
        reply: false,
        retweet: false,
    };
    println!("bound_syntax_parameters");
    // Both parameters sent to bound_syntax_notify must have the same type to 
    // avoid compile error.
    bound_syntax_notify(tweet1, tweet2);
}

fn multiple_trait_bounds() {
    let email = Email {
        to: String::from("abc@example.com"),
        from: String::from("deg@example.com"),
        subject: String::from("hi"),
        body: String::from("hello there!")
    };
    multiple_trait_notify(email);
}

fn where_clause() {
    let email = Email {
        to: String::from("abc@example.com"),
        from: String::from("deg@example.com"),
        subject: String::from("hi"),
        body: String::from("hello there!")
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    multiple_trait_where_clause(article, email);    
}

fn return_implementation() {
    let tweet = returns_summarizable();
    println!("return_implementation tweet summarized: {}", tweet.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/*
You can only use impl Trait if you’re returning a single type. For example, 
this code that returns either a NewsArticle or a Tweet with the return type 
specified as impl Summary wouldn’t work:

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/

/*
This version of the largest function fails to compile, because when we made the 
largest function generic, it became possible for the list parameter to have 
types in it that don’t implement the Copy trait. Consequently, we wouldn’t be 
able to move the value out of list[0] and into the largest variable, resulting 
in an error.

To call this code with only those types that implement the Copy trait, we can 
add Copy to the trait bounds of T!
fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn get_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/*
If we don’t want to restrict the largest function to the types that implement 
the Copy trait, we could specify that T has the trait bound Clone instead of 
Copy. Then we could clone each value in the slice when we want the largest 
function to have ownership. Using the clone function means we’re potentially 
making more heap allocations in the case of types that own heap data like 
String, and heap allocations can be slow if we’re working with large amounts of 
data.

Another way we could implement largest is for the function to return a reference 
to a T value in the slice. If we change the return type to &T instead of T, 
thereby changing the body of the function to return a reference, we wouldn’t 
need the Clone or Copy trait bounds and we could avoid heap allocations.
*/

fn trait_bond_conditional_methods() {
    let p = Pair::new(1, 2);
    p.cmp_display()
}