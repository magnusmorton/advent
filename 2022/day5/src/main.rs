use std::str;

use regex::Regex;


fn part1(num:usize, from:usize, to:usize, stacks:&mut Vec<Vec<char>>) {
    for _ in 0..num {
        let moved = stacks[from-1].pop().unwrap();
        stacks[to-1].push(moved)
    }
}

fn part2(num:usize, from:usize, to:usize, stacks:&mut Vec<Vec<char>>) {
    let len = stacks[from-1].len();
    let mut moved: Vec<char> = stacks[from-1].drain(len-num..).collect();
    stacks[to-1].append(&mut moved);
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut count = 0;
    let mut indices:Vec<usize> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        if line.starts_with(" 1") {
            for (i, ch )in line.char_indices() {
                if ch.is_numeric() {
                    stacks.push(Vec::new());
                    indices.push(i);
                }   
            }
            break
        }
        count += 1;
    }

    for i in (0..count).rev() {
        for (idx, j) in indices.iter().enumerate() {
            let ch = lines[i].as_bytes()[*j] as char;
            if ch.is_alphabetic() {
                stacks[idx].push(ch);
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let iter = lines.iter();
    for line in iter.skip(count+2) {
        let caps = re.captures(line);
        match caps {
            Some(cap) => {
                let num:usize = cap.get(1).unwrap().as_str().parse().unwrap();
                let from:usize = cap.get(2).unwrap().as_str().parse().unwrap();
                let to:usize = cap.get(3).unwrap().as_str().parse().unwrap();
                //part1(num, from, to, &mut stacks);
                part2(num, from, to, &mut stacks);

            }
            None  => {
                
            }
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
    println!("")
}
