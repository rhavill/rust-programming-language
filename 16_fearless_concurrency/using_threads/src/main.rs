use std::thread;
use std::time::Duration;

fn main() {
    /*
    Even though we tell the spawned thread to print until i is 9, it only got to 
    5 before the main thread shuts down. (This was the default behavior when not
    using the handle returned by the spawn function.)
    */

    // thread::spawn(|| {
    // The handle returned by spawn allows us to wait for threads to complete:
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("in new_thread: hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Calling join at this point causes all of the output from the spawned 
    // thread to appear before any of the output from the main thread.
    // handle.join().unwrap(); 

    for i in 1..5 {
        println!("in new_thread: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join at this point causes all of the output from the 1st spawned 
    // thread to appear interleaved with all of the output from the main thread.
    handle.join().unwrap();
    
    let v = vec![1, 2, 3];
    /* 
    The next line would cause compiler error: "closure may outlive the current 
    function, but it borrows `v`, which is owned by the current function". Rust 
    infers how to capture v, and because println! only needs a reference to v, 
    the closure tries to borrow v. However, there’s a problem: Rust can’t tell 
    how long the spawned thread will run, so it doesn’t know if the reference to 
    v will always be valid.
    */
    // thread::spawn(|| {

    /* 
    By adding the move keyword before the closure, we force the closure to 
    take ownership of the values it’s using rather than allowing Rust to infer 
    that it should borrow the values.
    */
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

}

