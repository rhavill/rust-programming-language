use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    simple_message_passing();
    sending_multiple_values();
    multiple_producers();
}

fn simple_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        /*
        Uncommenting the next line causes compiler error, "borrow of moved 
        value: `val`". The send function takes ownership of its parameter, and 
        when the value is moved, the receiver takes ownership of it. This stops 
        us from accidentally using the value again after sending it.
        */
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("simple_message_passing got: {}", received);
}

fn sending_multiple_values() {
    /*
    In the main thread, we’re not calling the recv function explicitly anymore: 
    instead, we’re treating rx as an iterator. For each value received, we’re 
    printing it. When the channel is closed, iteration will end.
    */
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("sending_multiple_values got: {}", received);
    }    
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("multiple_producers got: {}", received);
    }    
}