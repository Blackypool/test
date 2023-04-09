use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut range_of_numbers_str: String = String::new();
    stdout().write("Enter rang of numbers: ".as_bytes()).unwrap();
    stdout().flush().unwrap();
    stdin().read_line(&mut range_of_numbers_str).unwrap();
    let start: Instant = Instant::now();
    let range_of_numbers: u128 = range_of_numbers_str.trim().parse().unwrap();
    let mut simple_numbers: String = String::new();
    for i in 1..range_of_numbers {
        let mut flag: bool = true;
        for j in 2..=((i as f64).sqrt() as u128 + 1) {
            if i % j == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            simple_numbers += &format!("{i} ");
        }
    }
    let mut bytes_written = 0;
    while bytes_written < simple_numbers.len() {
        bytes_written += stdout().write(&simple_numbers.as_bytes()[bytes_written..]).unwrap();
    }
    let duration: Duration = start.elapsed();
    stdout().write(format!("\n{:?}", duration).as_bytes()).unwrap();
}

