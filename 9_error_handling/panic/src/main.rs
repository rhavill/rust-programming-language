use std::io;
/*
By default, when a panic occurs, the program starts unwinding, which means Rust
walks back up the stack and cleans up the data from each function it encounters.
But this walking back and cleanup is a lot of work. The alternative is to 
immediately abort, which ends the program without cleaning up. Memory that the 
program was using will then need to be cleaned up by the operating system. If in
your project you need to make the resulting binary as small as possible, you can
switch from unwinding to aborting upon a panic by adding panic = 'abort' to the 
appropriate [profile] sections in your Cargo.toml file. For example, if you want 
to abort on panic in release mode, add this:

[profile.release]
panic = 'abort'
*/

fn main() {
    println!("Run this with \"RUST_BACKTRACE=1 cargo run\" to see a backtrace.");
    loop {
        println!("Input a number and press <Enter>:");
        println!("1 - to see a basic panic crash.");
        println!("2 - to see a buffer overread crash.");
        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match option {
            1 => basic_panic(),
            2 => buffer_overread(),
            _   => continue,
        }
    }
}

fn basic_panic() {
    panic!("crash and burn");
}

fn buffer_overread() {
    let v = vec![1, 2, 3];
    let overread = v[99];
    println!("I will crash before this is output: {}.", overread);
}
