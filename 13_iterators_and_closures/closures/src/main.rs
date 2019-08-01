use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    println!("x == y? {}", equal_to_x(y));

    /*
    FnOnce consumes the variables it captures from its enclosing scope, known 
    as the closure’s environment. To consume the captured variables, the closure 
    must take ownership of these variables and move them into the closure when 
    it is defined. The Once part of the name represents the fact that the 
    closure can’t take ownership of the same variables more than once, so it can 
    be called only once.

    FnMut can change the environment because it mutably borrows values.

    Fn borrows values from the environment immutably.

    If you want to force the closure to take ownership of the values it uses in 
    the environment, you can use the move keyword before the parameter list. 
    This technique is mostly useful when passing a closure to a new thread to 
    move the data so it’s owned by the new thread.
    */
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // The next line fails to compile, because x has been moved.
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

/*
The trait bounds on T specify that it’s a closure by using the Fn trait. Any 
closure we want to store in the calculation field must have one u32 parameter 
(specified within the parentheses after Fn) and must return a u32 (specified 
after the ->).
*/
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

// #[test]
// fn call_with_different_values() {
//     let mut c = Cacher::new(|a| a);

//     //let v1 = c.value(1);
//     let v2 = c.value(2);

//     assert_eq!(v2, 2);
// }