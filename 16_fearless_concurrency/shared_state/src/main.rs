#[allow(unused_imports)]
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::thread;

/*
A mutex allows only one thread to access some data at any given time. To access 
the data in a mutex, a thread must first signal that it wants access by asking 
to acquire the mutex’s lock. The lock is a data structure that is part of the 
mutex that keeps track of who currently has exclusive access to the data.

Two rules for mutexes:

1) You must attempt to acquire the lock before using the data.

2) When you’re done with the data that the mutex guards, you must unlock the 
data so other threads can acquire the lock.
*/

fn main() {
    using_mutexes();
    multi_threaded_mutex_error();
    simpler_multi_threaded_mutex_error();
    multiple_ownership_multiple_threads();
    atomic_reference_counting();
}

fn using_mutexes() {
    let m = Mutex::new(5);

    {
        /*
        To access the data inside the mutex, we use the lock method to acquire 
        the lock. This call will block the current thread so it can’t do any 
        work until it’s our turn to have the lock.

        The call to lock returns a smart pointer called MutexGuard, wrapped in a 
        LockResult that we handled with the call to unwrap. The MutexGuard smart 
        pointer implements Deref to point at our inner data; the smart pointer 
        also has a Drop implementation that releases the lock automatically when 
        a MutexGuard goes out of scope.
        */
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("using_mutexes: m = {:?}", m);
}

fn multi_threaded_mutex_error() {
    // The commented code fails to compile with error: "capture of moved value: 
    //`counter`". See the simpler_multi_threaded_mutex_error function for
    // a more specific compiler error.
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("multi_threaded_mutex_error result: {}", *counter.lock().unwrap());
}

fn simpler_multi_threaded_mutex_error() {
    /* This fails to compile like the first function. In this case, the first 
    error message indicates that counter is moved into the closure for the 
    thread associated with handle. That move is preventing us from capturing 
    counter when we try to call lock on it and store the result in num2 in the 
    second thread! So Rust is telling us that we can’t move ownership of counter 
    into multiple threads. This was hard to see earlier because our threads were 
    in a loop, and Rust can’t point to different threads in different iterations 
    of the loop. Let’s fix the compiler error with a multiple-ownership method 
    discussed in Chapter 15.
    */
    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();

    //     *num += 1;
    // });
    // handles.push(handle);

    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();

    //     *num2 += 1;
    // });
    // handles.push(handle2);

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("simpler_multi_threaded_mutex_error result: {}", *counter.lock().unwrap());
}

/*
In Chapter 15, we gave a value multiple owners by using the smart pointer Rc<T> 
to create a reference counted value. Let’s do the same here and see what 
happens. We’ll wrap the Mutex<T> in Rc<T> in Listing 16-14 and clone the Rc<T> 
before moving ownership to the thread. Now that we’ve seen the errors, we’ll 
also switch back to using the for loop, and we’ll keep the move keyword with the
closure.    
*/
fn multiple_ownership_multiple_threads() {
    /*
    This function also fails to compile. Here are some important parts to focus 
    on: the first inline error says `std::rc::Rc<std::sync::Mutex<i32>>` cannot 
    be sent between threads safely. The reason for this is in the next important 
    part to focus on, the error message. The distilled error message says the 
    trait bound `Send` is not satisfied.

    Unfortunately, Rc<T> is not safe to share across threads. When Rc<T> manages 
    the reference count, it adds to the count for each call to clone and 
    subtracts from the count when each clone is dropped. But it doesn’t use any 
    concurrency primitives to make sure that changes to the count can’t be 
    interrupted by another thread. This could lead to wrong counts—subtle bugs 
    that could in turn lead to memory leaks or a value being dropped before 
    we’re done with it. What we need is a type exactly like Rc<T> but one that 
    makes changes to the reference count in a thread-safe way.
    */
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("multiple_ownership_multiple_threads result: {}", *counter.lock().unwrap());
}

/*
Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent 
situations. The a stands for atomic, meaning it’s an atomically reference 
counted type. Atomics are an additional kind of concurrency primitive that we 
won’t cover in detail here: see the standard library documentation for 
std::sync::atomic for more details. At this point, you just need to know that 
atomics work like primitive types but are safe to share across threads.
*/
fn atomic_reference_counting() {
   let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("atomic_reference_counting result: {}", *counter.lock().unwrap());
}

/*
You might have noticed that counter is immutable but we could get a mutable 
reference to the value inside it; this means Mutex<T> provides interior 
mutability, as the Cell family does. In the same way we used RefCell<T> in 
Chapter 15 to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to 
mutate contents inside an Arc<T>.

Another detail to note is that Rust can’t protect you from all kinds of logic 
errors when you use Mutex<T>. Recall in Chapter 15 that using Rc<T> came with 
the risk of creating reference cycles, where two Rc<T> values refer to each 
other, causing memory leaks. Similarly, Mutex<T> comes with the risk of creating 
deadlocks. These occur when an operation needs to lock two resources and two 
threads have each acquired one of the locks, causing them to wait for each other 
forever.
*/