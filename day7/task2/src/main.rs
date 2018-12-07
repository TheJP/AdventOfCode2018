extern crate regex;

use std::io;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::io::BufRead;
use std::collections::HashMap;

const TASK_TIME: usize = 60;
const WORKERS: i32 = 4;

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

    let mut tasks_done = 0;
    let mut time = 0;
    let mut finishes = HashMap::new();
    let mut occupied_workers = 0;
    while tasks_done < size {
        for from in in_degree.iter().enumerate()
            .filter(|(_, &degree)| degree == 0)
            .map(|(from, _)| from).collect::<Vec<_>>() {
            if occupied_workers > WORKERS { break; }

            in_degree[from] = -1;
            // Time it takes for a task: current time + constant time for each task + index of the task
            finishes.entry(time + TASK_TIME + from)
                .or_insert(Vec::new())
                .push(from);
            occupied_workers += 1;
        }

        if let Some(froms) = finishes.get(&time) {
            for &from in froms {
                for to in &graph[from] {
                    in_degree[*to] -= 1;
                }
                let from = ('A' as u8 + from as u8) as char;
                print!("{}", from);
                occupied_workers -= 1;
                tasks_done += 1;
            }
        }

        time += 1;
    }

    println!();
    println!("{}", time);

    Ok(())
}