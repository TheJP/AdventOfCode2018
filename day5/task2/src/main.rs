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

    let mut min = polymer.len();

    for c in 'a' as u8..'z' as u8 {
        let c = c as char;
        let mut polymer = polymer.to_string(); // copy polymer
        let mut stack = String::new();

        while !polymer.is_empty() {
            let l = polymer.pop();
            let r = stack.pop();
            match (l, r) {
                (Some(l), Some(r)) if l != r && l.eq_ignore_ascii_case(&r) => {}
                (Some(l), None) => { stack.push(l); }
                (Some(l), Some(r)) if r.eq_ignore_ascii_case(&c) => { stack.push(l); }
                (None, Some(r)) => { stack.push(r); }
                (Some(l), Some(r)) if l.eq_ignore_ascii_case(&c) => { stack.push(r); }
                (None, None) => {}
                (Some(l), _) if l.eq_ignore_ascii_case(&c) => {}
                (_, Some(r)) if r.eq_ignore_ascii_case(&c) => {}
                (Some(l), Some(r)) => {
                    stack.push(r);
                    stack.push(l);
                }
            }
        }

        if stack.len() < min {
            min = stack.len();
        }
    }

    println!("{}", min);

    Ok(())
}