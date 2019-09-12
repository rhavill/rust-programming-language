mod gui;
use gui::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("draw function called for SelectBox: width={}, height={}, options={:?}.",
            self.width, self.height, self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/*
You can only make object-safe traits into trait objects. Some complex rules 
govern all the properties that make a trait object safe, but in practice, only 
two rules are relevant. A trait is object safe if all the methods defined in the 
trait have the following properties:

1) The return type isn’t Self.
2) There are no generic type parameters.

The Self keyword is an alias for the type we’re implementing the traits or 
methods on. Trait objects must be object safe because once you’ve used a trait 
object, Rust no longer knows the concrete type that’s implementing that trait. 
If a trait method returns the concrete Self type, but a trait object forgets the 
exact type that Self is, there is no way the method can use the original 
concrete type. The same is true of generic type parameters that are filled in 
with concrete type parameters when the trait is used: the concrete types become 
part of the type that implements the trait. When the type is forgotten through 
the use of a trait object, there is no way to know what types to fill in the 
generic type parameters with.
*/