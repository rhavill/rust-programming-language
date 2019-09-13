/*
Patterns that will match for any possible value passed are irrefutable. An 
example would be x in the statement let x = 5; because x matches anything and 
therefore cannot fail to match. Patterns that can fail to match for some 
possible value are refutable. An example would be Some(x) in the expression if 
let Some(x) = a_value because if the value in the a_value variable is None 
rather than Some, the Some(x) pattern will not match.
*/

fn main() {
    #[allow(unused_variables)]
    let some_option_value = Some(3);
    /*
    The next line causes the compiler error, "refutable pattern in local 
    binding: `None` not covered", because.
    */
    // let Some(x) = some_option_value;
    /*
    The next if/let statement causes a warning:  "irrefutable if-let pattern".
    */
    if let x = 5 {
        println!("{}", x);
}   ;
}
