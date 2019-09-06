/*
Drop lets you customize what happens when a value is about to go out of scope.
Box<T> customizes Drop to deallocate the space on the heap that the box points 
to. In Rust, you can specify that a particular bit of code be run whenever a 
value goes out of scope, and the compiler will insert this code automatically.
*/

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    pretend_drop_example();
    early_drop();
}

#[allow(unused_variables)]
fn pretend_drop_example() {
    // Variables are dropped in the reverse order of their creation, so d is 
    // dropped before c.
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created in pretend_drop_example.");
}

fn early_drop() {
    /*
    Occasionally, you might want to clean up a value early. One example is when 
    using smart pointers that manage locks: you might want to force the drop 
    method that releases the lock to run so other code in the same scope can 
    acquire the lock. Rust doesn’t let you call the Drop trait’s drop method 
    manually; instead you have to call the std::mem::drop function provided by 
    the standard library if you want to force a value to be dropped before the 
    end of its scope.
    */
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created in early_drop.");
    // The next line fails compilation with the error: "explicit use of 
    // destructor method".
    // c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of early_drop.");
}
