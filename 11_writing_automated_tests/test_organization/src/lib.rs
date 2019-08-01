pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

/*
Note that the internal_adder function is not marked as pub, but because tests 
are just Rust code and the tests module is just another module, you can bring 
internal_adder into a test’s scope and call it. 
*/
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}