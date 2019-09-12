/*
We create a trait object by specifying some sort of pointer, such as a & 
reference or a Box<T> smart pointer, then the dyn keyword, and then specifying 
the relevant trait. We can use trait objects in place of a generic or concrete 
type. Wherever we use a trait object, Rust’s type system will ensure at compile 
time that any value used in that context will implement the trait object’s 
trait.
*/
pub trait Draw {
    fn draw(&self);
}

/*
Screen that holds a vector named components. This vector is of type 
Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a 
Box that implements the Draw trait.
*/
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

/*
A generic type parameter can only be substituted with one concrete type at a 
time, whereas trait objects allow for multiple concrete types to fill in for the 
trait object at runtime. 
*/
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("draw function called for Button.");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
