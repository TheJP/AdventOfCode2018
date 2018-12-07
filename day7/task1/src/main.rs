extern crate regex;

use std::io;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    // Step P must be finished before step O can begin.
    let re = Regex::new(r"Step (?P<from>[A-Z]) must be finished before step (?P<to>[A-Z]) can begin").unwrap();

    let size = 'Z' as usize - 'A' as usize + 1;
    let mut graph = vec![Vec::new(); size];
    let mut in_degree = vec![0; size];

    for line in reader.lines().map(|l| l.unwrap()) {
        let captures = re.captures(line.trim()).unwrap();
        let (from, to): (char, char) = (captures["from"].parse().unwrap(), captures["to"].parse().unwrap());
        graph[from as usize - 'A' as usize].push(to as usize - 'A' as usize);
        in_degree[to as usize - 'A' as usize] += 1;
    }

    for _ in 0..size {
        let (from, _) = in_degree.iter().enumerate().find(|(_, &degree)| degree == 0).unwrap();
        for to in &graph[from] {
            in_degree[*to] -= 1;
        }
        in_degree[from] = -1;
        let from = ('A' as u8 + from as u8) as char;
        print!("{}", from);
    }

    Ok(())
}