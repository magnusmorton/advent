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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
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
