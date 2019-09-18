// use std::io::Error;
// use std::fmt;

fn main() {
    type_synonyms_with_type_aliases();
    never_type();
    sized_trait();
}

fn type_synonyms_with_type_aliases() {
    /*
    Because Kilometers and i32 are the same type, we can add values of both 
    types and we can pass Kilometers values to functions that take i32 
    parameters. However, using this method, we don’t get the type checking 
    benefits that we get from the newtype pattern discussed earlier.
    */
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("type_synonyms_with_type_aliases: x + y = {}", x + y);

    /*
    A type alias makes this code more manageable by reducing the repetition.

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
    }
 
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }
    fn returns_long_type() -> Thunk {
        // --snip--
    }    
   */
 
    /*
    The Result<..., Error> is repeated a lot. As such, std::io has this type of 
    alias declaration.
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }

    Because this declaration is in the std::io module, we can use the fully 
    qualified alias std::io::Result<T>—that is, a Result<T, E> with the E filled 
    in as std::io::Error. The Write trait function signatures end up looking 
    like this:
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
    }
    */
}

fn never_type() {
    /*
    Rust has a special type named ! that’s known in type theory lingo as the 
    empty type because it has no values. We prefer to call it the never type 
    because it stands in the place of the return type when a function will never 
    return. Here is an example:
    fn bar() -> ! {
        // --snip--
    }
    This code is read as “the function bar returns never.” Functions that return 
    never are called diverging functions. We can’t create values of the type ! 
    so bar can never possibly return.

    continue has a ! value. That is, when Rust computes the type of guess, it 
    looks at both match arms, the former with a value of u32 and the latter with 
    a ! value. Because ! can never have a value, Rust decides that the type of 
    guess is u32.

    Expressions of type ! can be coerced into any other type. We’re allowed to 
    end this match arm with continue because continue doesn’t return a value.

    In this code, val has the type T and panic! has the type !, so the result of 
    the overall match expression is T. This code works because panic! doesn’t 
    produce a value; it ends the program. In the None case, we won’t be 
    returning a value from unwrap, so this code is valid.
    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }

    One final expression that has the type ! is a loop:
    print!("forever ");
    loop {
        print!("and ever ");
    }
    */
}

fn sized_trait() {
    /*
    To work with DSTs, Rust has a particular trait called the Sized trait to 
    determine whether or not a type’s size is known at compile time. This trait 
    is automatically implemented for everything whose size is known at compile 
    time. In addition, Rust implicitly adds a bound on Sized to every generic 
    function. That is, a generic function definition like this:

    fn generic<T>(t: T) {
        // --snip--
    }

    is actually treated as though we had written this:

    fn generic<T: Sized>(t: T) {
        // --snip--
    }

    By default, generic functions will work only on types that have a known size 
    at compile time. However, you can use the following special syntax to relax 
    this restriction:

    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
    */
}