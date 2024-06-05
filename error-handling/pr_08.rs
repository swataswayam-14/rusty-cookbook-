//A Shortcut for Propagating Errors: the ? Operator

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn main(){
    match read_username_from_file(){
        Ok(username) => println!("{}", username),
        Err(e) => println!("Unable to read username from the file : {}", e),
    }
}