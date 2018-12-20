extern crate itertools;

use itertools::Itertools;

const SERIAL: i64 = 7165;
const DIM: usize = 300;

fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let power_level = (rack_id * y + serial) * rack_id;
    return (power_level / 100) % 10 - 5;
}

fn create_grid(serial: i64) -> Vec<Vec<i64>> {
    let mut grid = vec![vec![0; DIM + 1]; DIM + 1];
    for (x, y) in (1..DIM + 1).cartesian_product(1..DIM + 1) {
        grid[y][x] = power_level(x as i64, y as i64, serial);
    }

    // Create fast sum grid
    for y in 1..DIM + 1 {
        for x in 1..DIM + 1 {
            grid[y][x] += grid[y - 1][x] + grid[y][x - 1] - grid[y - 1][x - 1];
        }
    }
    grid
}

fn sum(grid: &Vec<Vec<i64>>, x1: usize, y1: usize, x2: usize, y2: usize) -> i64 {
    grid[y2][x2] - grid[y2][x1 - 1] - grid[y1 - 1][x2] + grid[y1 - 1][x1 - 1]
}

fn find_best_grid(grid: &Vec<Vec<i64>>, grid_size: usize) -> ((usize, usize), i64) {
    debug_assert!(grid_size > 0);
    debug_assert!(grid_size <= DIM);

    let mut max = -100;
    let mut max_position = (DIM + 1, DIM + 1);
    for (x, y) in (1..DIM + 1 - grid_size + 1).cartesian_product(1..DIM + 1 - grid_size + 1) {
        let sum = sum(&grid, x, y, x + grid_size - 1, y + grid_size - 1);
        if sum > max {
            max = sum;
            max_position = (x, y);
        }
    }

    (max_position, max)
}

fn find_best(serial: i64, grid_size: usize) -> (usize, usize) {
    let grid = create_grid(serial);
    let (coordinates, _) = find_best_grid(&grid, grid_size);
    coordinates
}

fn find_best_anysize(serial: i64) -> (usize, usize, usize) {
    let grid = create_grid(serial);
    let (((x, y), _), size) = (1..DIM + 1)
        .map(|grid_size| (find_best_grid(&grid, grid_size), grid_size))
        .max_by_key(|((_, max), _)| *max)
        .unwrap();

    (x, y, size)
}

fn main() {
    let (x, y) = find_best(SERIAL, 3);
    println!("Part 1: {},{}", x, y);
    let (x, y, size) = find_best_anysize(SERIAL);
    println!("Part 2: {},{},{}", x, y, size);
}

#[cfg(test)]
mod test {
    use power_level;
    use find_best;
    use find_best_anysize;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_level(3, 5, 8));
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn test_find_best() {
        assert_eq!((33, 45), find_best(18, 3));
        assert_eq!((21, 61), find_best(42, 3));
    }

    #[test]
    fn test_find_best_anysize() {
        assert_eq!((90, 269, 16), find_best_anysize(18));
        assert_eq!((232, 251, 12), find_best_anysize(42));
    }
}
