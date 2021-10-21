use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    let mut conway_cube = BTreeMap::new();
    let reader = BufReader::new(io::stdin());
    let mut y = 0;
    for line in reader.lines() {
        for (i, c) in line?.chars().enumerate() {
            conway_cube.insert((i, y, 0), c);
        }
        y+=1;
    }
    println!("{:?}", &conway_cube);
    //let mut new_cube = BTreeMap::new();
    for n in 0..6 {
        for ((x,y,z), ch) in &conway_cube {
            println!("foo");
        }
    }
    Ok(())
}

