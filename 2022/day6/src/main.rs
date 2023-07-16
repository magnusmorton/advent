use std::fs;
use std::collections::{VecDeque, HashMap};
fn main() {
    let input = fs::read_to_string("input.txt").expect("file couldn't be opened");
    let mut last4: VecDeque<char> = VecDeque::new();
    let mut counts = HashMap::new();
    let mut found:usize = 0;
    for (i, ch) in input.chars().enumerate() {
        if last4.len() >= 14 {
            if counts.values().all(|x| *x <= 1) {
                found = i;
                break
            }
            let del = last4.pop_front().unwrap();
            *counts.get_mut(&del).unwrap() -= 1;
        }
        last4.push_back(ch);
        *counts.entry(ch).or_insert(0) += 1;
    }
    println!("{}", found)
}
