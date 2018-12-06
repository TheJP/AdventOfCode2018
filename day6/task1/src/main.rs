extern crate regex;

use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::io::BufRead;
use std::io;

const DIM: i32 = 400;

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

    //let mut plain = vec![vec![0; DIM as usize]; DIM as usize];
    let mut area = vec![Some(0); points.len()];

    for y in 0..DIM {
        for x in 0..DIM {
            let (i1, min1, _, min2) = points.iter().enumerate().fold((0, 3 * DIM, 0, 3 * DIM), |(i1, min1, i2, min2), (i, (px, py))| {
                let d = (px - x).abs() + (py - y).abs();
                if d < min1 { (i, d, i1, min1) } else if d < min2 { (i1, min1, i, d) } else { (i1, min1, i2, min2) }
            });

            if min1 < min2 {
                area[i1] = if x == 0 || x == DIM - 1 || y == 0 || y == DIM - 1 { None } else { area[i1].map(|a| a + 1) }
            }
        }
    }

    println!("{}", area.iter().filter_map(|&a| a).max().unwrap());

    Ok(())
}
