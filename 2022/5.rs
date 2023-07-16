use std::io;
use std::io::Read;
use std::str;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    println!("{:?}", str::from_utf8(&lines));
}
