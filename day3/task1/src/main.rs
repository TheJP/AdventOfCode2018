extern crate regex;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

struct Rectangle {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    //#1 @ 483,830: 24x18
    let re = Regex::new(
        r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();

    let mut fabric = vec![vec![0; 2000]; 2000];

    for line in reader.lines().map(|l| l.expect("Error while reading lines")) {
        let captures = re.captures(line.trim()).unwrap();
        let rectangle = Rectangle {
            id: captures["id"].parse().unwrap(),
            left: captures["left"].parse().unwrap(),
            top: captures["top"].parse().unwrap(),
            width: captures["width"].parse().unwrap(),
            height: captures["height"].parse().unwrap(),
        };

        for y in rectangle.top..rectangle.top + rectangle.height {
            for x in rectangle.left..rectangle.left + rectangle.width {
                let x = x as usize;
                let y = y as usize;
                fabric[y][x] = if fabric[y][x] == 0 { rectangle.id } else { -1 };
            }
        }
    }

    let result = fabric.iter()
        .flat_map(|row| row)
        .filter(|&value| *value == -1)
        .count();

    println!("{}", result);

    Ok(())
}
