use minigrep::Config;
use std::{env, process};

//limit the responsability of main to:
//  call the parsing logic
//  set up config
//  call a run function from lib.rs
//  handle error
fn main() {
    let args: Vec<String> = env::args().collect();

    //uses a closure and unwrap or else which allows to define behabiour for error variants, where
    //unwrap would simply panic
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the argument: {err}");
        process::exit(1);
    });

    //Equivalent to unwrap_or_else
    if let Err(e) = minigrep::run(config) {
        println!("App error: {e}");
        process::exit(1);
    }
}
