use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn checksum(line: &str) -> (u32, u32) {
    let mut counts = HashMap::new();
    for letter in line.chars() {
        counts.entry(letter)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    (if counts.values().any(|&c| c == 2) { 1 } else { 0 },
     if counts.values().any(|&c| c == 3) { 1 } else { 0 })
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    let mut checksum_twos: u32 = 0;
    let mut checksum_threes: u32 = 0;

    while reader.read_line(&mut buffer)? > 0 {
        let (twos, threes) = checksum(buffer.trim());
        checksum_twos += twos;
        checksum_threes += threes;
        buffer = String::new();
    }

    println!("Result {} ({}*{})", checksum_twos * checksum_threes, checksum_twos, checksum_threes);

    Ok(())
}