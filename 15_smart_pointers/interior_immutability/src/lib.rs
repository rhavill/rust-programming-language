/*
Interior mutability is a design pattern in Rust that allows you to mutate data 
even when there are immutable references to that data; normally, this action is 
disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code 
inside a data structure to bend Rust’s usual rules that govern mutation and 
borrowing. We can use types that use the interior mutability pattern when we can 
ensure that the borrowing rules will be followed at runtime, even though the 
compiler can’t guarantee that. The unsafe code involved is then wrapped in a 
safe API, and the outer type is still immutable.

The advantage of checking the borrowing rules at runtime instead is that certain 
memory-safe scenarios are then allowed, whereas they are disallowed by the 
compile-time checks. The RefCell<T> type is useful when you’re sure your code 
follows the borrowing rules but the compiler is unable to understand and 
guarantee that.

There are situations in which it would be useful for a value to mutate itself in 
its methods but appear immutable to other code. Code outside the value’s methods 
would not be able to mutate the value. Using RefCell<T> is one way to get the 
ability to have interior mutability. But RefCell<T> doesn’t get around the 
borrowing rules completely: the borrow checker in the compiler allows this 
interior mutability, and the borrowing rules are checked at runtime instead. If 
you violate the rules, you’ll get a panic! instead of a compiler error.

Recap:

1) Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have 
single owners.

2) Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> 
allows only immutable borrows checked at compile time; RefCell<T> allows 
immutable or mutable borrows checked at runtime.

3) Because RefCell<T> allows mutable borrows checked at runtime, you can mutate 
the value inside the RefCell<T> even when the RefCell<T> is immutable.
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;
use std::cell::RefCell;
    struct MockMessenger {
        //sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
