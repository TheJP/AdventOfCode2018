extern crate itertools;

use itertools::Itertools;

const SERIAL: i64 = 7165;
const DIM: usize = 300;

fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let power_level = (rack_id * y + serial) * rack_id;
    return (power_level / 100) % 10 - 5;
}

fn find_best(serial: i64) -> (usize, usize) {
    let mut grid = vec![vec![0; DIM]; DIM];
    for (x, y) in (0..DIM).cartesian_product(0..DIM) {
        grid[y][x] = power_level(x as i64, y as i64, serial);
    }

    let mut max = -100;
    let mut max_position = (DIM, DIM);
    for (x, y) in (0..DIM - 2).cartesian_product(0..DIM - 2) {
        let sum: i64 = (0..3).cartesian_product(0..3)
            .map(|(dx, dy)| grid[y + dy][x + dx]).sum();
        if sum > max {
            max = sum;
            max_position = (x, y);
        }
    }

    max_position
}

fn main() {
    let (x, y) = find_best(SERIAL);
    println!("{},{}", x, y);
}

#[cfg(test)]
mod test {
    use power_level;
    use find_best;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_level(3, 5, 8));
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn test_find_best() {
        assert_eq!((33, 45), find_best(18));
        assert_eq!((21, 61), find_best(42));
    }
}
