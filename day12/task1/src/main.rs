extern crate regex;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::mem::swap;

const GENERATIONS: i32 = 20;
const PADDING: usize = GENERATIONS as usize * 2 + 10;

fn b(value: bool) -> usize {
    if value { 1 } else { 0 }
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    // initial state: .##..##.
    let initial_state_regex = Regex::new(r"initial state: (?P<state>[.#]*)").unwrap();

    // .#... => #
    let transition_regex = Regex::new(r"(?P<from>[.#]*) => (?P<to>[.#])").unwrap();

    let mut lines = reader.lines().map(|l| l.unwrap());

    let init = lines.next().unwrap();
    let init = &initial_state_regex.captures(&init).unwrap()["state"];

    let mut state = vec![false; 2 * PADDING + init.len()];
    for p in init.chars().enumerate().map(|(i, c)| (i + PADDING, c == '#')) {
        state[p.0] = p.1;
    }

    let mut tree = [[[[[false; 2]; 2]; 2]; 2]; 2];
    for line in lines.skip(1) {
        let captures = transition_regex.captures(&line).unwrap();
        let mut from = captures["from"].chars()
            .map(|c| b(c == '#'));
        let to = captures["to"].eq("#");
        tree[from.next().unwrap()][from.next().unwrap()]
            [from.next().unwrap()][from.next().unwrap()]
            [from.next().unwrap()] = to;
    }

    let mut state2 = vec![false; state.len()];
    for _ in 0..GENERATIONS {
        for i in 2..state.len() - 2 {
            state2[i] = tree[b(state[i - 2])]
                [b(state[i - 1])][b(state[i])]
                [b(state[i + 1])][b(state[i + 2])];
        }
        swap(&mut state, &mut state2);
    }

    println!("Count: {}", state.iter().enumerate()
        .filter(|(_, v)| **v)
        .map(|(v, _)| v as i32 - PADDING as i32)
        .sum::<i32>());

    Ok(())
}