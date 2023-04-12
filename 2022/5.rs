use std::io;
use std::io::Read;
use std::str;

fn main() {
    let mut lines = Vec::new();
    io::stdin().read_to_end(&mut lines);
    println!("{:?}", str::from_utf8(&lines));
}
