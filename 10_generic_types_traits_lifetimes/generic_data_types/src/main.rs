/*
Rust implements generics in such a way that your code doesn’t run any slower 
using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using 
generics at compile time. Monomorphization is the process of turning generic 
code into specific code by filling in the concrete types that are used when 
compiled.
*/
fn main() {
    same_functions_different_types();
    struct_definition();
    mixed_type_struct();
    method_definition();
    mixup_struct_types();
}

fn same_functions_different_types() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
The body of largest won’t work for all possible types that T could be. Because 
we want to compare values of type T in the body, we can only use types whose 
values can be ordered. To enable comparisons, the standard library has the 
std::cmp::PartialOrd
*/
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn struct_definition() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer is {:?}, float is: {:?}", integer, float);

    // This will fail to compile, because x and y must be the same type:
    // let wont_work = Point { x: 5, y: 4.0 };
}

fn mixed_type_struct() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("both_integer is {:?}, both_float is: {:?}, integer_and_float is: {:?}", both_integer, both_float, integer_and_float);
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn method_definition() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}, p.y={}", p.x(), p.y);

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let fp = Point { x: 1.5, y: 2.5 };
    println!("distance is {}", fp.distance_from_origin());

}

fn mixup_struct_types() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}