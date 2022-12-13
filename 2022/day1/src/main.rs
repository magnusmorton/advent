fn a1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let split = input.split("\r\n\r\n");
    for s in split {
        println!("fooo: {}", s);
        
    }
}

fn main() {
    a1();
}
