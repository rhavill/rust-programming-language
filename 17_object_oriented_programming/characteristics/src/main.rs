/*
OOP languages share certain common characteristics, namely objects, 
encapsulation, and inheritance. The Gang of Four book defines OOP this way:
Object-oriented programs are made up of objects. An object packages both data 
and the procedures that operate on that data. The procedures are typically 
called methods or operations. Using this definition, Rust is object oriented: 
structs and enums have data, and impl blocks provide methods on structs and 
enums.

Inheritance is a mechanism whereby an object can inherit from another object’s 
definition, thus gaining the parent object’s data and behavior without you 
having to define them again. If a language must have inheritance to be an 
object-oriented language, then Rust is not one. There is no way to define a 
struct that inherits the parent struct’s fields and method implementations. 

You choose inheritance for two main reasons. One is for reuse of code: you can 
implement particular behavior for one type, and inheritance enables you to reuse 
that implementation for a different type. You can share Rust code using default 
trait method implementations instead. 

The other reason to use inheritance relates to the type system: to enable a 
child type to be used in the same places as the parent type. This is also called 
polymorphism, which means that you can substitute multiple objects for each 
other at runtime if they share certain characteristics. Rust instead uses 
generics to abstract over different possible types and trait bounds to impose 
constraints on what those types must provide. This is sometimes called bounded 
parametric polymorphism.
*/

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut collection = AveragedCollection{list: vec![], average: 0.0};
    collection.add(2);
    collection.add(4);
    collection.add(6);
    collection.remove();
    println!("average is {}.", collection.average());
    println!("collection is {:?}.", collection);
}
