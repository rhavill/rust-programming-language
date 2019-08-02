/*
All iterators implement a trait named Iterator that is defined in the standard 
library. The definition of the trait looks like this:

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

Item and Self::Item are defining an associated type with this trait. This code 
says implementing the Iterator trait requires that you also define an Item type, 
and this Item type is used in the return type of the next method. In other 
words, the Item type will be the type returned from the iterator.

The Iterator trait only requires implementors to define one method: the next 
method, which returns one item of the iterator at a time wrapped in Some and, 
when iteration is over, returns None.
*/
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// sum is a method that consumes an iterator
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Here we use filter with a closure that captures the shoe_size variable from 
// its environment to iterate over a collection of Shoe struct instances. It 
// will return only shoes that are the specified size.
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// Use the Iterator trait to create an iterator.
/*
We’ve shown that you can create an iterator by calling iter, into_iter, or 
iter_mut on a vector. You can create iterators from the other collection types 
in the standard library, such as hash map. You can also create iterators that do 
anything you want by implementing the Iterator trait on your own types.
*/
// This iterator counts from 1 to 5:
#[allow(dead_code)]
struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

/*
This function takes the values produced by an instance of Counter, pairs them 
with values produced by another Counter instance after skipping the first value, 
multiplies each pair together, keeping only those results that are divisible by 
3, and adds all the resulting values together:
*/
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}