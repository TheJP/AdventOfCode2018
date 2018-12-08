use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::slice::Iter;

struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<i32>,

    // Part 2
    data: i32,
}

fn construct_tree(numbers: &mut Iter<i32>, current: &mut Box<Node>) -> i32 {
    let children = *numbers.next().unwrap();
    let size = *numbers.next().unwrap();
    let mut sum = 0;

    for _ in 0..children {
        let mut child = Box::new(Node { children: Vec::new(), metadata: Vec::new(), data: 0 });
        sum += construct_tree(numbers, &mut child);
        current.children.push(child);
    }

    for _ in 0..size {
        let data = *numbers.next().unwrap();
        current.metadata.push(data);
        sum += data;
    }

    // Part 2
    if children <= 0 {
        current.data = sum;
    } else {
        current.data = current.metadata.iter()
            .filter_map(|&index| current.children.get(index as usize - 1))
            .map(|node| node.data).sum();
    }

    sum
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    let numbers = reader.lines()
        .map(|l| l.unwrap())
        .flat_map(|l| l.split_whitespace()
            .map(|token| token.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut tree = Box::new(Node { children: Vec::new(), metadata: Vec::new(), data: 0 });

    let sum = construct_tree(&mut numbers.iter(), &mut tree);

    println!("Sum: {}", sum);
    println!("Value: {}", tree.data);

    Ok(())
}