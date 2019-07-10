const MAX_POINTS: u32 = 100_000;

fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    /* Shadowing is different from marking a variable as mut, because we’ll 
    get a compile-time error if we accidentally try to reassign to this 
    variable without using the let keyword. By using let, we can perform a few
    transformations on a value but have the variable be immutable after those 
    transformations have been completed. */
    let y = 5;
    
    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);
}
