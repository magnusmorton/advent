use std::fs::File;
fn main() {
    let path = "test.txt";
    let mut file = std::fs::read_to_string(path).unwrap();
    let mut head = (0,0);
    let mut tail = (0,0);

    for line in file.lines() {
        println!("{}", line);
        let parts:Vec<&str> = line.split_ascii_whitespace().collect();
        match parts[0] {
            "R" => println!("right"),
            "L" => println!("left"),
            "D" => println!("Down"),
            "U" => println!("up"),
            _ => println!("error")
        }
    }
}
