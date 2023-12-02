
use std::collections::HashSet;


fn update_tail(head: (i32, i32), tail: (i32,i32)) -> (i32, i32) {
    let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

    match (dx, dy) {
        (2, 0) => (tail.0 + 1, tail.1),
        (0, 2) => (tail.0, tail.1 +1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (0, -2) => (tail.0, tail.1 -1),
        (2, 1) => (tail.0+1, tail.1 +1),
        (1, 2) => (tail.0+1, tail.1 +1),
        (-2, 1) => (tail.0 -1, tail.1 +1),
        (-1, 2) => (tail.0 -1, tail.1+1),
        (2, -1) => (tail.0 +1, tail.1 -1),
        (1, -2) => (tail.0 +1, tail.1 -1),
        (-2, -1) => (tail.0 -1, tail.1 -1),
        (-1, -2) => (tail.0 -1, tail.1 -1),
        (2, 2) => (tail.0 +1, tail.1 +1 ),
        (-2, -2) => (tail.0-1, tail.1-1),
        (2, -2) => (tail.0+1, tail.1 -1),
        (-2, 2) => (tail.0 -1, tail.1 +1),
        (_,_) => tail
    }
}

fn main() {
    let path = "input.txt";
    let file = std::fs::read_to_string(path).unwrap();
    a(&file);
    b(&file);
}

fn a(file: &String) {
    let mut head = (0,0);
    let mut tail = (0,0);
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert(tail);

    for line in file.lines() {
        let parts:Vec<&str> = line.split_ascii_whitespace().collect();
        let mut vec = (0,0);
        let step = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "R" => {
                vec = (1,0);
            },
            "L" => {
                vec = (-1,0);
            },
            "D" => {
                vec = (0,-1);
            },
            "U" => {
                vec = (0,1);
            },
            _ => println!("error")
        }
        for i in 0..step {
            head.0 += vec.0;
            head.1 += vec.1;
            tail = update_tail(head, tail);
            visited.insert(tail);
        }
    }
    println!("visited {}", visited.len())
}

fn b(file: &String) {
    let mut knots: [(i32, i32); 10] = [(0,0); 10];
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));

    for line in file.lines() {
        let parts:Vec<&str> = line.split_ascii_whitespace().collect();
        let mut vec = (0,0);
        let step = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "R" => {
                vec = (1,0);
            },
            "L" => {
                vec = (-1,0);
            },
            "D" => {
                vec = (0,-1);
            },
            "U" => {
                vec = (0,1);
            },
            _ => println!("error")
        }
        for _ in 0..step {
            knots[0].0 += vec.0;
            knots[0].1 += vec.1;
            for i in 1..10 {
                knots[i] = update_tail(knots[i-1], knots[i]);
            }
            //println!("{:?}", knots[2]);
            visited.insert(knots[9]);
        }
    }
    println!("visited {}", visited.len())
}
