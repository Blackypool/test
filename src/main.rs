use std::io::{stdin, stdout, Write};

fn main(){
    let mut name: String = String::new();
    stdout().write("Enter your name: ").unwrap();
    stdout().flash().unwrap();
    stdin().read_line(&mut name).unwrap();
    name.pop();
    stdout().write(format!("Hello {name}!\n").as_bytes()).unwrap();
}
