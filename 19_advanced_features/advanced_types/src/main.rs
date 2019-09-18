// use std::io::Error;
// use std::fmt;

fn main() {
    type_synonyms_with_type_aliases();
    never_type();
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