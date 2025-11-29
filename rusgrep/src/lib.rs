use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

impl Config {
    pub fn new(args: &[String])-> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let string = args[1].clone();
        let file = args[2].clone();
        let sensitivity=env::var("CASE_SENSITIVITY").is_err();
        Ok(Config { string, file,sensitivity })
    }
}
pub struct InLine{
    pub line: i32,
    pub content:String
}
pub fn search(string:&str,contents:& str)->Vec<InLine>{
        let mut counter=1;
        let mut result:Vec<InLine>=Vec::new();
        for line in contents.lines(){
            counter+=1;
            if line.contains(string){
                result.push(InLine { line: counter, content:String::from(line) });
            }
        }
        result
    }
pub fn search_insensitive(string:&str,contents:& str)->Vec<InLine>{
    let mut counter=1;
    let string=string.to_lowercase();
    let mut result:Vec<InLine>=Vec::new();
    for line in contents.lines(){
        counter+=counter;
        if line.to_lowercase().contains(&string) {
            result.push(InLine { line: counter, content:String::from(line) });
        }
    }
    result
} 
pub fn run(config: & Config)->Result<Vec<InLine>,Box<dyn Error>> {
    let mut f = File::open(config.file.clone())?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results=if config.sensitivity{search(&config.string, &contents)}else {
        search_insensitive(&config.string, &contents)
    };

    Ok( results)
}
pub struct Config {
    pub string: String,
    pub file: String,
    pub sensitivity:bool
}
