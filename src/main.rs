use std::io::{stdin, stdout, Write};

fn main(){
    let mut name: String = String::new();
    stdin().read_line(&mut name).unwrap();
    stdout().write(format!("Hello {name}!\n").as_bytes()).unwrap();
    stdout().flush().unwrap();
}
