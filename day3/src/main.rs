use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get_grid(filename: &str) -> Vec<Vec<char>> {
    let contents = lines_from_file(filename);

    let height = contents.len();
    let input_width = contents[0].len();
    let extensions = ((height * 7) / input_width) + 1;
    let total_width = contents[0].len() * extensions;
    let mut grid = vec![vec!['.'; total_width]; height];

    for (y, line) in contents.iter().enumerate() {
        for extension in (0..extensions) {
            for (x, chr) in line.chars().enumerate() {
                //println!("{} {} {}", y, x + (extension * input_width), char);
                grid[y][x + (extension * input_width)] = chr;
            }
        }
    }

    grid
}

fn solve1(filename: &str) -> u64 {
    let grid = get_grid(filename);

    count_trees(&grid, 1, 3)
}

fn count_trees(grid: &Vec<Vec<char>>, down: usize, right: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut result = 0;

    while y < grid.len() {
        if grid[y][x] == '#' {
            result += 1;
        }

        x += right;
        y += down;
    }

    println!("{} {} {}", down, right, result);

    result
}

fn solve2(filename: &str) -> u64 {
    let contents = lines_from_file(filename);
    let grid = get_grid(filename);

    count_trees(&grid, 1, 1)
        * count_trees(&grid, 1, 3)
        * count_trees(&grid, 1, 5)
        * count_trees(&grid, 1, 7)
        * count_trees(&grid, 2, 1)
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("{}", solve1("input.txt"));
    println!("{}", solve2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 7);
        assert_eq!(solve2("example.txt"), 336);
    }
}
