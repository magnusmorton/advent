
use std::fs::File;
use std::collections::HashSet;


fn update_tail(head: (i32, i32), tail: (i32,i32)) -> (i32, i32) {
    let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);
    //println!("dx,dy{:?}", (dx, dy));

    match (dx, dy) {
        (2,0) => (tail.0 + 1, tail.1),
        (0,2) => (tail.0, tail.1 +1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (0, -2) => (tail.0, tail.1 -1),
        (2, 1) => (tail.0+1, tail.1 +1),
        (1, 2) => (tail.0+1, tail.1 +1),
        (-2,1) => (tail.0 -1, tail.1 +1),
        (-1, 2) => (tail.0 -1, tail.1+1),
        (2, -1) => (tail.0 +1, tail.1 -1),
        (1, -2) => (tail.0 +1, tail.1 -1),
        (-2, -1) => (tail.0 -1, tail.1 -1),
        (-1, -2) => (tail.0 -1, tail.1 -1),
        (_,_) => tail
    }
}

fn main() {
    let path = "input.txt";
    let file = std::fs::read_to_string(path).unwrap();
    a(file);
}

fn a(file: String) {
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert(tail);

    for line in file.lines() {
        println!("{}", line);
        let parts:Vec<&str> = line.split_ascii_whitespace().collect();
        let mut vec = (0,0);
        match parts[0] {
            "R" => {
                println!("right");
                let step = parts[1].parse::<i32>().unwrap();
                for i in 0..step {
                    head.0 += 1;
                    tail = update_tail(head, tail);
                    visited.insert(tail);

                }
            },
            "L" => {
                println!("left");
                let step = parts[1].parse::<i32>().unwrap();

                for i in 0..step {
                    head.0 -= 1;
                    tail = update_tail(head, tail);
                    visited.insert(tail);

                }
            },
            "D" => {
                println!("Down");
                let step = parts[1].parse::<i32>().unwrap();
                for i in 0..step {
                    head.1 -= 1;
                    tail = update_tail(head, tail);
                    visited.insert(tail);

                }
            },
            "U" => {
                println!("up");
                let step = parts[1].parse::<i32>().unwrap();
                for i in 0..step {
                    head.1 += 1;
                    tail = update_tail(head, tail);
                    visited.insert(tail);

                }
            },
            _ => println!("error")
        }

        println!("head, {:?}", head);
        println!("tail, {:?}", tail);
    }
    println!("visited {}", visited.len())
}
