fn main() {
    for_loop();
    map();
}

fn for_loop() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("In for_loop, got: {}", val);
    }
}

fn map() {
    // The map method produces another iterator.
    let v1: Vec<i32> = vec![1, 2, 3];
    // The next line doesn't do anything and gets a warning from the compier,
    // because iterator adaptors are lazy, and we need to consume the iterator.
    // v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    println!("In map, v2 is: {:?}", v2);
}
