use std::ops::Add;
use std::fmt;

fn main() {
    operator_overloading();
    methods_with_same_name();
    supertraits();
    external_traits_on_external_types();
}

/*
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

RHS=Self: this syntax is called default type parameters. The RHS generic type 
parameter (short for “right hand side”) defines the type of the rhs parameter in 
the add method. If we don’t specify a concrete type for RHS when we implement 
the Add trait, the type of RHS will default to Self, which will be the type 
we’re implementing Add on.
*/
fn operator_overloading() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    /*
    When we implement Add for Point, we use the default for RHS because we 
    want to add two Point instances.
    */
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 });

    struct Millimeters(u32);
    struct Meters(u32);
    /*
    Here we customize the RHS type rather than using the default.
    */
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }        
}

fn methods_with_same_name() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("methods_with_same_name: This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("methods_with_same_name: Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("methods_with_same_name: *waving arms furiously*");
        }
    }    
    /*
    Because the fly method takes a self parameter, if we had two types that both 
    implement one trait, Rust could figure out which implementation of a trait 
    to use based on the type of self.
    */
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("methods_with_same_name: Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("methods_with_same_name: puppy")
        }
    }

    println!("methods_with_same_name: A baby dog is called a {}", Dog::baby_name());
    // The next line results in a compilation error: "type annotations required: 
    // cannot resolve `_...`"
    //println!("A baby dog is called a {}", Animal::baby_name());
    /*
    To disambiguate and tell Rust that we want to use the implementation of 
    Animal for Dog, we need to use fully qualified syntax.
    */
    println!("methods_with_same_name: A baby dog is called a {}", <Dog as Animal>::baby_name());

}

/*
Sometimes, you might need one trait to use another trait’s functionality. In 
this case, you need to rely on the dependent trait also being implemented. The 
trait you rely on is a supertrait of the trait you’re implementing. For example, 
let’s say we want to make an OutlinePrint trait with an outline_print method 
that will print a value framed in asterisks. In the implementation of 
outline_print, we want to use the Display trait’s functionality. Therefore, we 
need to specify that the OutlinePrint trait will work only for types that also 
implement Display and provide the functionality that OutlinePrint needs. We can 
do that in the trait definition by specifying OutlinePrint: Display. 
*/
fn supertraits() {
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("supertraits: {}", "*".repeat(len + 4));
            println!("supertraits: *{}*", " ".repeat(len + 2));
            println!("supertraits: * {} *", output);
            println!("supertraits: *{}*", " ".repeat(len + 2));
            println!("supertraits: {}", "*".repeat(len + 4));
        }
    }
    /*
    The first compile of this code results in error: "supertraits::Point` 
    doesn't implement `std::fmt::Display"
    */
    struct Point {
        x: i32,
        y: i32,
    }
    impl OutlinePrint for Point {}    

    /*
    To fix the compiler error, we implement Display on Point and satisfy the 
    onstraint that OutlinePrint requires
    */
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let coord = Point {x: 0, y: 0};
    println!("supertraits: Point is {}", coord);
}

/*
The orphan rule that states we’re allowed to implement a trait on a type as long 
as either the trait or the type are local to our crate. It’s possible to get 
around this restriction using the newtype pattern, which involves creating a new 
type in a tuple struct.
*/
fn external_traits_on_external_types() {
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("external_traits_on_external_types: w = {}", w);
    
}