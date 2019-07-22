fn main() {
    create_strings();
    updating_strings();
    indexing_into_strings();
    slicing_strings();
    iterating_strings();
}

fn create_strings() {
    #[allow(unused_mut)]
    let mut s = String::new();
    println!("In create_strings, s is '{}'.", s);

    let data = "initial contents";

    let s2 = data.to_string();
    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();
    let s4 = String::from("initial contents");
    println!("In create_strings, s2 is '{}'.", s2);
    println!("In create_strings, s3 is '{}'.", s3);
    println!("In create_strings, s4 is '{}'.", s4);
}

fn updating_strings() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("In updating_strings, s is '{}'.", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("In updating_strings, s1 is '{}'.", s1);
    println!("In updating_strings, s2 is '{}'.", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("In updating_strings, s3 is '{}'.", s3);

    /*
    The + operator uses the add method, whose signature looks something like 
    this:

    fn add(self, s: &str) -> String {
    */
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // note s4 has been moved here and can no longer be used
    println!("In updating_strings, s6 is '{}'.", s6);

    // If we need to concatenate multiple strings, the behavior of the + 
    // operator gets unwieldy:
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = s7 + "-" + &s8 + "-" + &s9;
    println!("In updating_strings, s10 is '{}'.", s10);

    // For more complicated string combining, we can use the format! macro:
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");
    let s14 = format!("{}-{}-{}", s11, s12, s13);
    println!("In updating_strings, s11 is '{}'.", s14);

}

fn indexing_into_strings() {
    // if you try to access parts of a String using indexing syntax in Rust, 
    // you’ll get an error.
    // let s1 = String::from("hello");
    // let h = s1[0];

    let len = String::from("Hola").len();
    println!("In indexing_into_strings, the length of 'Hola' is {} bytes.", len);

    let len2 = String::from("Здравствуйте").len();
    println!("In indexing_into_strings, the length of 'Здравствуйте' is {} bytes.", len2);

    /*
    If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored 
    as a vector of u8 values that looks like this:

    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    224, 165, 135]

    That’s 18 bytes and is how computers ultimately store this data. If we look 
    at them as Unicode scalar values, which are what Rust’s char type is, those 
    bytes look like this:

    ['न', 'म', 'स', '्', 'त', 'े']

    There are six char values here, but the fourth and sixth are not letters: 
    they’re diacritics that don’t make sense on their own. Finally, if we look 
    at them as grapheme clusters, we’d get what a person would call the four 
    letters that make up the Hindi word:

    ["न", "म", "स्", "ते"]

    Rust provides different ways of interpreting the raw string data that 
    computers store so that each program can choose the interpretation it 
    needs, no matter what human language the data is in.
    */
}

fn slicing_strings() {
    // You can use [] with a range to create a string slice containing 
    // particular bytes:
    let hello = "Здравствуйте";
    // Here, s will be a &str that contains the first 4 bytes of the string.
    // If we used &hello[0..1], rust would panic, because the first byte is not
    // a char boundary.
    let s = &hello[0..4];
    println!("In slicing_strings, s is '{}'.", s);

}

fn iterating_strings() {
    println!("In iterating_strings, the chars of the string 'नमस्ते' are iterated like this:");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // The bytes method returns each raw byte.
    println!("In iterating_strings, the bytes of the string 'नमस्ते' are iterated like this:");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}   