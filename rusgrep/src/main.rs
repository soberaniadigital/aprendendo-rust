use std::env;
use std::process;
use rusgrep::{Config,run};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


   
    
    let contents=run(&config).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.string);
    println!("In file {}", config.file);
    
    for line in contents {
        println!("\x1b[32m{}\x1b[0m : \x1b[31m{}\x1b[0m",line.line,line.content)
    }
    
}

