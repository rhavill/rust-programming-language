use std::fmt::Display;

fn main() {
    dangling_references();
    lifetime_annotation_syntax();
    function_signature_lifetime_annotations();
    lifetime_annotation_restrictions();
    struct_lifetime_annotations();
    lifetime_elision();
    lifetime_annotations_in_methods();
    static_lifetime();
    generic_trait_params_trait_bounds_and_lifetimes();
}

fn dangling_references() {
    /*
    // This code fails to compile with "borrowed value does not live long
    // enough" error, because x will be out of scope when the inner scope ends.
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
    */  

    // In this case, the error may be corrected by placing r and x in the same scope:
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("In dangling_references, r = {}.", r); //
}                         // ---------+

fn lifetime_annotation_syntax<'a>() {
    let a: &i32 = &1;        // a reference
    let b: &'a i32 = &2;     // a reference with an explicit lifetime
    let mut c: &'a i32 = &3; // a mutable reference with an explicit lifetime
    println!("In lifetime_annotation_syntax, a={}, b={}, c={}.", a, b, c);
    let x: &i32 = a;
    let y: &i32 = b;
    // let mut z: &i32 = c;
    c = &4;

    println!("In lifetime_annotation_syntax, a={}, b={}, c={}, x={}, y={}.", a, b, c, x, y);
}

fn function_signature_lifetime_annotations() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("In function_signature_lifetime_annotations, the longest string is '{}'.", result);
}

/* 
This function fails to compile with the following error: "missing lifetime
specifier". The function's return type contains a borrowed value, but the 
signature does not say whether it is borrowed from `x` or `y`. When we’re 
defining this function, we don’t know the concrete values that will be passed 
into this function, so we don’t know whether the if case or the else case will 
execute. We also don’t know the concrete lifetimes of the references that will 
be passed in.

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_annotation_restrictions() {
    /*
    This code failes to compile with error, "`string2` does not live long 
    enough". In order for result to be valid for the println! statement, string2 
    would need to be valid until the end of the outer scope. Rust knows this 
    because we annotated the lifetimes of the function parameters and return 
    values using the same lifetime parameter 'a.

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("In lifetime_annotation_restrictions, the longest string is '{}'.", result);
    */

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("In lifetime_annotation_restrictions, the longest string is '{}'.", result);
    }
}

fn struct_lifetime_annotations() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("In struct_lifetime_annotations, i is {:?}.", i);
}

fn lifetime_elision() {
    let first_word = first_word("Hello there.");
    println!("In lifetime_elision, first_word is '{}'.", first_word)
}

fn first_word(s: &str) -> &str {
    /*
    In early versions (pre-1.0) of Rust, this code wouldn’t have compiled 
    because every reference needed an explicit lifetime. At that time, the 
    function signature would have been written like this:

    fn first_word<'a>(s: &'a str) -> &'a str {}

    After writing a lot of Rust code, the Rust team found that Rust programmers 
    were entering the same lifetime annotations over and over in particular 
    situations. These situations were predictable and followed a few 
    deterministic patterns. The developers programmed these patterns into the 
    compiler’s code so the borrow checker could infer the lifetimes in these 
    situations and wouldn’t need explicit annotations.

    The patterns programmed into Rust’s analysis of references are called the 
    lifetime elision rules. These aren’t rules for programmers to follow; 
    they’re a set of particular cases that the compiler will consider, and if 
    your code fits these cases, you don’t need to write the lifetimes 
    explicitly.

    Rule #1 - Each parameter that is a reference gets its own lifetime 
    parameter. In other words, a function with one parameter gets one lifetime 
    parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two 
    separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so 
    on.

    Rule #2 - If there is exactly one input lifetime parameter, that lifetime is 
    assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    Rule #3 - If there are multiple input lifetime parameters, but one of them 
    is &self or &mut self because this is a method, the lifetime of self is 
    assigned to all output lifetime parameters. This third rule makes methods 
    much nicer to read and write because fewer symbols are necessary.
    */
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn lifetime_annotations_in_methods() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    /*
    The lifetime parameter declaration after impl and its use after the type 
    name are required, but we’re not required to annotate the lifetime of the 
    reference to self because of the first elision rule.
    */
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
 
    /*
    Here is an example where the third lifetime elision rule applies There are 
    two input lifetimes, so Rust applies the first lifetime elision rule and 
    gives both &self and announcement their own lifetimes. Then, because one of 
    the parameters is &self, the return type gets the lifetime of &self, and all 
    lifetimes have been accounted for.
    */
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
   let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("In lifetime_annotations_in_methods, level is {}.", i.level());
    println!("In lifetime_annotations_in_methods announce_and_return_part:");
    i.announce_and_return_part("Hello!");
}

/*
A static reference can live for the entire duration of the program. All string 
literals have the 'static lifetime, which we can annotate as follows:  Before 
specifying 'static as the lifetime for a reference, think about whether the 
reference you have actually lives the entire lifetime of your program or not. 
You might consider whether you want it to live that long, even if it could. Most 
of the time, the problem results from attempting to create a dangling reference 
or a mismatch of the available lifetimes. In such cases, the solution is fixing 
those problems, not specifying the 'static lifetime.
*/
fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
    println!("In static_lifetime, s is {}", s);
}

fn generic_trait_params_trait_bounds_and_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    println!("generic_trait_params_trait_bounds_and_lifetimes:");
    let result = longest_with_an_announcement(string1.as_str(), string2, 32);
    println!("longest is: '{}'", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}