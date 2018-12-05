use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let mut reader = BufReader::new(f);

    let mut polymer = String::new();
    reader.read_to_string(&mut polymer)?;
    println!("start: {}", polymer.len());

    polymer = polymer.trim().to_string();
    let mut stack = String::new();

    while !polymer.is_empty() {
        let l = polymer.pop();
        let r = stack.pop();
        match (l, r) {
            (Some(l), Some(r)) if l != r && l.eq_ignore_ascii_case(&r) => {}
            (Some(l), Some(r)) => {
                stack.push(r);
                stack.push(l);
            }
            (Some(l), None) => { stack.push(l); }
            (None, Some(r)) => { stack.push(r); }
            (None, None) => {}
        }
    }

    println!("{}", stack.len());
    println!("{}", stack);

    Ok(())
}