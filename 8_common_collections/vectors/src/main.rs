fn main() {
    create_new_vectors();
    update_vector();
    drop_vector();
    read_elements();
    iterating_values();
    multiple_types_with_enum();
}

fn create_new_vectors() {
    /* We added a type annotation here. Because we aren’t inserting any values 
    into this vector. For now, know that the Vec<T> type provided by the 
    standard library can hold any type, and when a specific vector holds a 
    specific type, the type is specified within angle brackets.
    */
    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    println!("In create_new_vectors, v is  {:?}.", v);
    println!("In create_new_vectors, v2 is  {:?}.", v2);

}

fn update_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("In update_vector, v is  {:?}.", v);
}

fn drop_vector() {
    let v = vec![1, 2, 3, 4];

    println!("In drop_vector, v is  {:?}.", v);

} // <- v goes out of scope and is freed here

fn read_elements() {
    /*
    two ways to get the third element are by using & and [], which gives us a 
    reference, or by using the get method with the index passed as an argument, 
    which gives us an Option<&T>.
    */
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("read_elements: The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("read_elements: The third element is {}", third),
        None => println!("read_elements: There is no third element."),
    }

    // This will cause the program to panic because it references a nonexistent 
    // element.
    // let does_not_exist = &v[100];

    let does_not_exist = v.get(100);
    println!("In read_elements, does_not_exist is {:?}.", does_not_exist);

    /*
    Recall the rule that states you can’t have mutable and immutable references 
    in the same scope.
    */
    let mut v2 = vec![1, 2, 3, 4, 5];

    #[allow(unused_variables)]
    let first = &v2[0];

    v2.push(6);

    /*
    Uncommenting the next line causes the program to crash. This error is due 
    to the way vectors work: adding a new element onto the end of the vector 
    might require allocating new memory and copying the old elements to the new 
    space, if there isn’t enough room to put all the elements next to each 
    other where the vector currently is. In that case, the reference to the 
    first element would be pointing to deallocated memory.
    */
    // println!("read_elements: The first element is: {}", first);
}

fn iterating_values() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("iterating_values: {}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }

    println!("iterating_values: v2 is {:?}", v2);
}

fn multiple_types_with_enum() {
    /*
    Rust needs to know what types will be in the vector at compile time so it 
    knows exactly how much memory on the heap will be needed to store each 
    element. A secondary advantage is that we can be explicit about what types 
    are allowed in this vector. If Rust allowed a vector to hold any type, 
    there would be a chance that one or more of the types would cause errors 
    with the operations performed on the elements of the vector. Using an enum 
    plus a match expression means that Rust will ensure at compile time that 
    every possible case is handled, as discussed in Chapter 6.

    When you’re writing a program, if you don’t know the exhaustive set of 
    types the program will get at runtime to store in a vector, the enum 
    technique won’t work. Instead, you can use a trait object, which we’ll 
    cover in Chapter 17.
    */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("multiple_types_with_enum: row is {:?}", row);

    let row2 = vec![
        SpreadsheetCell::Text(String::from("dog")),
        SpreadsheetCell::Text(String::from("hen")),
        SpreadsheetCell::Int(31),
        SpreadsheetCell::Float(13.13),
    ];

    println!("multiple_types_with_enum: row2 is {:?}", row2);
}