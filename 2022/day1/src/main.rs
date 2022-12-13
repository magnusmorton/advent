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
    let tots = split.map (|s| {
        let lines = s.lines();
        let cals = lines.map(|l| l.parse::<i32>().unwrap());
        
        let tot: i32 = cals.sum();
        return tot;
    });
    let mut tot_vec = tots.collect::<Vec<i32>>();
    tot_vec.sort();
    tot_vec.reverse();
    return tot_vec.iter().take(3).sum();
}

fn main() {
    println!("{}", a2());
}
