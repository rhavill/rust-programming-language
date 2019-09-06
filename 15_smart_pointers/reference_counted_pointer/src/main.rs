/*
There are cases when a single value might have multiple owners. For example, in 
graph data structures, multiple edges might point to the same node, and that 
node is conceptually owned by all of the edges that point to it. A node 
shouldn’t be cleaned up unless it doesn’t have any edges pointing to it. The 
Rc<T> type keeps track of the number of references to a value which determines 
whether or not a value is still in use. If there are zero references to a value, 
the value can be cleaned up without any references becoming invalid.
*/

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

/*
Using Rc<T> allows a single value to have multiple owners, and the count ensures 
that the value remains valid as long as any of the owners still exist.
*/
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
// We need to add a use statement to bring Rc<T> into scope because it’s not in 
// the prelude.
use std::rc::Rc;


fn main() {
    sharing_data();
    cloning_increases_reference_count();
}

fn sharing_data() {
    // The following cause compilation to fail with: "use of moved value: `a`".
    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // The implementation of Rc::clone doesn’t make a deep copy of all the data 
    // like most types’ implementations of clone do. 
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("In sharing_data, b is {:?}.", b);
    println!("In sharing_data, c is {:?}.", c);
}

#[allow(unused_variables)]
fn cloning_increases_reference_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));    
}