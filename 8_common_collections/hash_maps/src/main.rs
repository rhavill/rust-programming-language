// The type HashMap<K, V> stores a mapping of keys of type K to values of type 
// V.
use std::collections::HashMap;

fn main() {
    creating_hash_maps();
    ownership();
    accessing_values();
    updating();
}

fn creating_hash_maps() {
    // This HashMap has keys of type String and values of type i32.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("In creating_hash_maps, scores is: {:?}", scores);

    // Another way of constructing a hash map is by using the collect method 
    // on a vector of tuples, where each tuple consists of a key and its value.
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("In creating_hash_maps, scores2 is: {:?}", scores2);
}

fn ownership() {
    // For types that implement the Copy trait, like i32, the values are copied 
    // into the hash map. For owned values like String, the values will be 
    // moved and the hash map will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("In ownership, map is: {:?}", map);
}

fn accessing_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // The result is wrapped in Some because get returns an Option<&V>
    println!("In accessing_values, score is: {:?}", score);

    println!("In accessing_values, scores are iterated like this:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn updating() {
    // Overwriting a value:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("In updating, scores is: {:?}", scores);

    // Only inserting a value if the key does not already have a value:
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("In updating, scores2 is: {:?}", scores2);

    // Updating a value based on the old value:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // The or_insert method actually returns a mutable reference (&mut V) 
        // to the value for this key.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("In updating, map is: {:?}", map);

}