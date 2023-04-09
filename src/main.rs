use std::io::{stdout, Write};

fn main(){
    stdout().write("Hello Artem!\n".as_bytes()).unwrap();
    stdout().flush().unwrap();
}
