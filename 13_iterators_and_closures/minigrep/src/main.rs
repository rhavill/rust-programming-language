use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /*
    The env::args function returns an iterator! Rather than collecting the 
    iterator values into a vector and then passing a slice to Config::new, now 
    we’re passing ownership of the iterator returned from env::args to 
    Config::new directly.
    */
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
