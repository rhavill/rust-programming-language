use std::slice;
/*
Four actions with unsafe rust:

1) Dereference a raw pointer
2) Call an unsafe function or method
3) Access or modify a mutable static variable
4) Implement an unsafe trait

Unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety 
checks: if you use a reference in unsafe code, it will still be checked.
*/

fn main() {
    raw_pointer_dereference();
    call_unsafe_function();
    safe_abstraction_of_unsafe_code();
    calling_external_code();
    mutable_static_variables();
    unsafe_trait();
}

/*
Raw pointers can be immutable or mutable and are written as *const T and *mut T, 
respectively. The asterisk isn’t the dereference operator; it’s part of the type 
name. In the context of raw pointers, immutable means that the pointer can’t be 
directly assigned to after being dereferenced. Different from references and 
smart pointers, raw pointers:

1) Are allowed to ignore the borrowing rules by having both immutable and 
mutable pointers or multiple mutable pointers to the same location.
2) Aren’t guaranteed to point to valid memory.
3) Are allowed to be null.
4) Don’t implement any automatic cleanup.
*/
#[allow(unused_variables)]
fn raw_pointer_dereference() {
    /*
    Notice that we don’t include the unsafe keyword in this code. We can create 
    raw pointers in safe code; we just can’t dereference raw pointers outside an 
    unsafe block, as you’ll see in a bit. We’ve created raw pointers by using as 
    to cast an immutable and a mutable reference into their corresponding raw 
    pointer types. Because we created them directly from references guaranteed 
    to be valid, we know these particular raw pointers are valid, but we can’t 
    make that assumption about just any raw pointer.
    */
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    /*
    Trying to use arbitrary memory is undefined: there might be data at that 
    address or there might not, the compiler might optimize the code so there is 
    no memory access, or the program might error with a segmentation fault. 
    Usually, there is no good reason to write code like this, but it is 
    possible.
    */
    let address = 0x012345usize;
    let r = address as *const i32;

    /*
    Recall that we can create raw pointers in safe code, but we can’t 
    dereference raw pointers and read the data being pointed to. Next, we use 
    the dereference operator * on a raw pointer that requires an unsafe block.

    Creating a pointer does no harm; it’s only when we try to access the value 
    that it points at that we might end up dealing with an invalid value. Here
    we have *const i32 and *mut i32 raw pointers that both point to the same 
    memory location, where num is stored. If we instead tried to create an 
    immutable and a mutable reference to num, the code would not have compiled 
    because Rust’s ownership rules don’t allow a mutable reference at the same 
    time as any immutable references. With raw pointers, we can create a mutable 
    pointer and an immutable pointer to the same location and change data 
    through the mutable pointer, potentially creating a data race.
    */
    unsafe {
        println!("raw_pointer_dereference: r1 is: {}", *r1);
        println!("raw_pointer_dereference: r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {
}

fn call_unsafe_function() {
    // We must call the dangerous function within a separate unsafe block.
    unsafe {
        dangerous();
    }
}

/*
The safe method, split_at_mute, is defined on mutable slices: it takes one slice 
and makes it two by splitting the slice at the index given as an argument.

We can’t implement this function using only safe Rust. In this simplified
implementation, split_at_mut as a function rather than a method and only for 
slices of i32 values rather than for a generic type T. The assertion means that 
if we pass an index that is greater than the index to split the slice at, the 
function will panic before it attempts to use that index.
*/
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);
    /* 
    With the next line, the compiler fails with "cannot borrow `*slice` as 
    mutable more than once at a time". Rust’s borrow checker can’t understand 
    that we’re borrowing different parts of the slice; it only knows that we’re 
    borrowing from the same slice twice. Borrowing different parts of a slice is 
    fundamentally okay because the two slices aren’t overlapping, but Rust isn’t 
    smart enough to know this. When we know code is okay, but Rust doesn’t, it’s 
    time to reach for unsafe code.
    */
    // (&mut slice[..mid], &mut slice[mid..])
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }

}

fn safe_abstraction_of_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

/*
This function sets up an integration with the abs function from the C standard 
library. Functions declared within extern blocks are always unsafe to call from 
Rust code.
*/
extern "C" {
    fn abs(input: i32) -> i32;
}

fn calling_external_code() {
    unsafe {
        println!("calling_external_code: Absolute value of -3 according to C: {}", abs(-3));
    }
}

/*
We can also use extern to create an interface that allows other languages to 
call Rust functions. Instead of an extern block, we add the extern keyword and 
specify the ABI to use just before the fn keyword. We also need to add a 
#[no_mangle] annotation to tell the Rust compiler not to mangle the name of this 
function. 
*/
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

/*
In Rust, global variables are called static variables. Static variables are 
similar to constants. Static variables can only store references with the 
'static lifetime, which means the Rust compiler can figure out the lifetime.

A subtle difference is that values in a static variable have a fixed address in 
memory. Using the value will always access the same data. Constants, on the 
other hand, are allowed to duplicate their data whenever they’re used. Another 
difference between constants and static variables is that static variables can 
be mutable. Accessing and modifying mutable static variables is unsafe.
*/
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn mutable_static_variables() {
    add_to_count(3);

    unsafe {
        println!("mutable_static_variables: COUNTER: {}", COUNTER);
    }
}

/*
A trait is unsafe when at least one of its methods has some invariant that the 
compiler can’t verify.
*/
fn unsafe_trait() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}