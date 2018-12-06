extern crate regex;

use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::io::BufRead;
use std::io;

const DIM: i32 = 400;
const DISTANCE: i32 = 10000;

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let mut points = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let captures = re.captures(line.trim()).unwrap();
        let pair: (i32, i32) = (captures["x"].parse().unwrap(), captures["y"].parse().unwrap());
        points.push(pair);
    }

    let mut count = 0;

    for y in 0..DIM {
        for x in 0..DIM {
            let distance: i32 = points.iter().map(|(px, py)| (px - x).abs() + (py - y).abs()).sum();
            if distance < DISTANCE {
                count += 1;
            }
        }
    }

    println!("{}", count);

    Ok(())
}
