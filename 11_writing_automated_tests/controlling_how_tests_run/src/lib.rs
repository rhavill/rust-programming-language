/*
Some command line options go to cargo test, and some go to the resulting test 
binary. To separate these two types of arguments, you list the arguments that go 
to cargo test followed by the separator -- and then the ones that go to the test 
binary. Running cargo test --help displays the options you can use with cargo 
test, and running cargo test -- --help displays the options you can use after 
the separator --

If you don’t want to run the tests in parallel or if you want more fine-grained 
control over the number of threads used, you can send the --test-threads flag 
and the number of threads you want to use to the test binary. 

$ cargo test -- --test-threads=1

By default, if a test passes, Rust’s test library captures anything printed to 
standard output. If we want to see printed values for passing tests as well, we 
can disable the output capture behavior by using the --nocapture flag:

$ cargo test -- --nocapture

We can pass the name of any test function to cargo test to run only that test:

$ cargo test one_hundred

We can specify part of a test name, and any test whose name matches that value 
will be run. For example, because two of our tests’ names contain add, we can 
run those two by running:

$ cargo test add

Rather than listing as arguments all tests you do want to run, you can instead 
annotate the time-consuming tests using the ignore attribute to exclude them, by
putting "#[ignore]" above the test function:

If we want to run only the ignored tests, we can use:

$ cargo test -- --ignored
*/

#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
