fn a1() -> i32{
    let input = std::fs::read_to_string("input.txt").unwrap();
    let split = input.split("\r\n\r\n");
    let mut max = 0;
    for s in split {
        let lines = s.lines();
        let cals = lines.map(|l| l.parse::<i32>().unwrap());
        
        let tot = cals.sum();
        if tot > max {
            max = tot;
        }
    };
    return max;
}

fn a2() -> i32{
    let input = std::fs::read_to_string("input.txt").unwrap();
    let split = input.split("\r\n\r\n");
    let mut max = 0;
    for s in split {
        let lines = s.lines();
        let cals = lines.map(|l| l.parse::<i32>().unwrap());
        
        let tot = cals.sum();
        if tot > max {
            max = tot;
        }
    };
    return max;
}

fn main() {
    println!("{}", a1());
}
