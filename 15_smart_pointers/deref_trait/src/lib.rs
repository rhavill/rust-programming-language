/*
Implementing the Deref trait allows you to customize the behavior of the 
dereference operator, "*". By implementing Deref in such a way that a smart 
pointer can be treated like a regular reference.
*/



#[cfg(test)]
mod tests {
    use std::ops::Deref;
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    #[test]
    fn simple_dereference() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn box_dereference() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn custom_smart_pointer() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn implicit_deref_coercion() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}

/*
Similar to how you use the Deref trait to override the * operator on immutable 
references, you can use the DerefMut trait to override the * operator on 
mutable references.

Rust does deref coercion when it finds types and trait implementations in three 
cases:

1) From &T to &U when T: Deref<Target=U>
2) From &mut T to &mut U when T: DerefMut<Target=U>
3) From &mut T to &U when T: Deref<Target=U>
*/