use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;


fn main() {
    macro_rules();
    derive_macro();
    attribute_like_macros();
    function_like_macros();
}

fn macro_rules() {
    #[macro_export]
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    let vec = my_vec![1, 2, 3];
    println!("macro_rules: vec is: {:?}", vec);    
}

fn derive_macro() {

    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();
}

fn attribute_like_macros() {
    /*
    Attribute-like macros are similar to custom derive macros, but instead of 
    generating code for the derive attribute, they allow you to create new 
    attributes.

    #[route(GET, "/")]
    fn index() {

    This #[route] attribute would be defined by the framework as a procedural 
    macro. The signature of the macro definition function would look like this:

    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

    Here, we have two parameters of type TokenStream. The first is for the 
    contents of the attribute: the GET, "/" part. The second is the body of the 
    item the attribute is attached to: in this case, fn index() {} and the rest 
    of the function’s body.

    */
}

/*
Function-like macros define macros that look like function calls. Similarly to 
macro_rules! macros, they’re more flexible than functions. Function-like macros 
take a TokenStream parameter and their definition manipulates that TokenStream 
using Rust code as the other two types of procedural macros do.
*/
fn function_like_macros() {
    /*
    An example of a function-like macro is an sql! macro that might be called 
    like so:

    let sql = sql!(SELECT * FROM posts WHERE id=1);

    This macro would parse the SQL statement inside it and check that it’s 
    syntactically correct, which is much more complex processing than a 
    macro_rules! macro can do. The sql! macro would be defined like this:

    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
    */

}