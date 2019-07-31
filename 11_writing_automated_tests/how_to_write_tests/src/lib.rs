#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    /*
    The requirements for this program haven’t been agreed upon yet, and we’re 
    pretty sure the Hello text at the beginning of the greeting will change. We 
    decided we don’t want to have to update the test when the requirements 
    change, so instead of checking for exact equality to the value returned from 
    the greeting function, we’ll just assert that the output contains the text 
    of the input parameter.
    */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    /*
    Writing tests so they return a Result<T, E> enables you to use the question 
    mark operator in the body of tests, which can be a convenient way to write 
    tests that should fail if any operation within them returns an Err variant.

    You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
    Instead, you should return an Err value directly when the test should fail.
    */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
