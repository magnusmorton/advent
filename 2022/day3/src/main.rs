use std::collections::HashSet;
fn priority(c: char) -> u8 {
    if c.is_lowercase() {
        return c as u8 - 'a' as u8 + 1;
    } else {
        return c as u8 - 'A' as u8 + 27;
    }
}

fn p1(input: &str) {
    let mut res: u32= 0;
    for line in input.lines() {
        println!("{}", line);
        let (first, second) = line.split_at(line.len() / 2);
        let left_set: HashSet<char> = first.chars().collect();
        let common_slice  = second.chars().filter(|c| left_set.contains(c)).collect::<Vec<char>>();
        let common = common_slice.first().unwrap();
        res += priority(*common) as u32;
    }
    println!("{}", res);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    p1(&input);
}
