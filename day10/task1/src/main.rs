extern crate regex;

use std::io;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::io::BufRead;
use std::cmp::min;
use std::cmp::max;
use std::iter::FromIterator;

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn dimensions(points: &Vec<Point>) -> ((i32, i32), (i32, i32)) {
    let init = ((points[0].x, points[0].y), (points[0].x, points[0].y));
    points.iter().skip(1).fold(init, |((minx, miny), (maxx, maxy)), point| {
        ((min(minx, point.x), min(miny, point.y)),
         (max(maxx, point.x), max(maxy, point.y)))
    })
}

fn simulate(points: &mut Vec<Point>) {
    for point in points {
        point.x += point.vx;
        point.y += point.vy;
    }
}

fn show(points: &Vec<Point>, d: ((i32, i32), (i32, i32))) {
    let ((minx, miny), (maxx, maxy)) = d;
    let mut output = Box::new(vec![vec!['.'; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize]);
    for point in points {
        output[(point.y - miny) as usize][(point.x - minx) as usize] = '#';
    }

    for line in output.iter() {
        println!("{}", String::from_iter(line.iter()));
    }
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    // position=<-39892,  -9859> velocity=< 4,  1>
    let re = Regex::new(r"position=<(?P<x>[^,]*),(?P<y>[^>]*)> velocity=<(?P<vx>[^,]*),(?P<vy>[^>]*)>").unwrap();

    let mut points = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let captures = re.captures(&line).unwrap();
        points.push(Point {
            x: captures["x"].trim().parse().unwrap(),
            y: captures["y"].trim().parse().unwrap(),
            vx: captures["vx"].trim().parse().unwrap(),
            vy: captures["vy"].trim().parse().unwrap(),
        });
    }

    let mut count = 0;
    let mut d = dimensions(&points);
    loop {
        let previous = points.clone();
        simulate(&mut points);
        let d2 = dimensions(&points);
        if (d2.1).0 - (d2.0).0 > (d.1).0 - (d.0).0 &&
            (d2.1).1 - (d2.0).1 > (d.1).1 - (d.0).1 {
            show(&previous, d);
            break;
        }
        d = d2;
        count += 1;
    }

    println!("{}", count);

    Ok(())
}