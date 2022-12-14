use std::io::Lines;

fn score_choice(token: &str) -> i32 {
    match token {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn score_round(opp: &str, me: &str) -> i32 {
  
    match (opp, me) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => 0,
    }
}
fn get_choice(opp: &str, me: &str) -> &'static str {
  
    match (opp, me) {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => "A",
    }
}

fn a1(input: String) {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let mut tokens = line.split(" ");
        let opp = tokens.next().unwrap();
        let me = tokens.next().unwrap();
        score += score_choice(me) + score_round(opp, me);
    }
    println!("{}", score);
}

fn a2(input: String) {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let mut tokens = line.split(" ");
        let opp = tokens.next().unwrap();
        let me = tokens.next().unwrap();
        let my_choice = get_choice(opp, me);
        score += score_choice(my_choice) + score_round(opp, my_choice);
    }
    println!("{}", score);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    //a1(input);
    a2(input);

    
}
