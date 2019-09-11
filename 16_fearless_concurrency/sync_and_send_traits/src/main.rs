/*
The Send marker trait indicates that ownership of the type implementing Send 
can be transferred between threads. Almost every Rust type is Send, but there 
are some exceptions, including Rc<T>: this cannot be Send because if you cloned 
an Rc<T> value and tried to transfer ownership of the clone to another thread, 
both threads might update the reference count at the same time. For this reason, 
Rc<T> is implemented for use in single-threaded situations where you don’t want 
to pay the thread-safe performance penalty.

Therefore, Rust’s type system and trait bounds ensure that you can never 
accidentally send an Rc<T> value across threads unsafely. When we tried to do 
this in Listing 16-14, we got the error the trait Send is not implemented for 
Rc<Mutex<i32>>. When we switched to Arc<T>, which is Send, the code compiled.

Any type composed entirely of Send types is automatically marked as Send as 
well. Almost all primitive types are Send, aside from raw pointers.

The Sync marker trait indicates that it is safe for the type implementing Sync 
to be referenced from multiple threads. In other words, any type T is Sync if &T 
(a reference to T) is Send, meaning the reference can be sent safely to another 
thread. Similar to Send, primitive types are Sync, and types composed entirely 
of types that are Sync are also Sync.

The smart pointer Rc<T> is also not Sync for the same reasons that it’s not 
Send. The RefCell<T> type (which we talked about in Chapter 15) and the family 
of related Cell<T> types are not Sync. The implementation of borrow checking 
that RefCell<T> does at runtime is not thread-safe. The smart pointer Mutex<T> 
is Sync and can be used to share access with multiple threads as you saw in the 
“Sharing a Mutex<T> Between Multiple Threads” section.
*/

fn main() {
    println!("Hello, world!");
}
