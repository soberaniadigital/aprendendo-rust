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
pub fn search(string:&str,contents:& str)->Vec<String>{
        
        let mut result:Vec<String>=Vec::new();
        for line in contents.lines(){
            if line.contains(string){
                result.push(String::from(line));
            }
        }
        result
    }
pub fn search_insensitive(string:&str,contents:& str)->Vec<String>{
    let string=string.to_lowercase();
    let mut result:Vec<String>=Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&string) {
            result.push(String::from(line));
        }
    }
    result
} 
pub fn run(config: & Config)->Result<Vec<String>,Box<dyn Error>> {
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
mod test{

    use super::*;
    #[test]
    fn case_sensitive(){
        let string="duct";
        let contents="\
Rust:
safe,fast,productive.
pick three.
Duct tape";
        assert_eq!(vec!["safe,fast,productive."],search(string,contents));
    }
    #[test]
    fn case_insensitive(){
        let string="rUsT";
        let contents="\
Rust:
safe,fast,productive.
pick three.
Trust me.";
assert_eq!(vec!["Rust:","Trust me."],search_insensitive(string, contents))
    }
    
}