use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn build_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn count_char(chr: char, grid: &Vec<Vec<char>>) -> u64 {
    let width = grid.first().unwrap().len();
    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..width {
            if grid[y][x] == chr {
                sum += 1;
            }
        }
    }

    sum
}

const NEIGHBOURS: [(i64, i64); 8] = [
    // above
    (-1, -1),
    (-1, 0),
    (-1, 1),
    // beneath
    (1, -1),
    (1, 0),
    (1, 1),
    // right, left
    (0, 1),
    (0, -1),
];

fn num_adjecent(chr: char, y: usize, x: usize, grid: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    let height = grid.len() as i64;
    let width = grid.first().unwrap().len() as i64;

    for pos in NEIGHBOURS.iter() {
        let new_y = pos.0 + y as i64;
        let new_x = pos.1 + x as i64;
        if new_y >= 0 && new_y < height && new_x >= 0 && new_x < width {
            if grid[new_y as usize][new_x as usize] == chr {
                sum += 1;
            }
        }
    }

    sum
}

/*
If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
Floor (.) never changes; seats don't move, and nobody sits on the floor.
*/
fn mutate_grid(
    grid: &Vec<Vec<char>>,
    empty_mutation: &dyn Fn(usize, usize, &Vec<Vec<char>>) -> char,
    occupied_mutation: &dyn Fn(usize, usize, &Vec<Vec<char>>) -> char,
) -> Vec<Vec<char>> {
    let width = grid.first().unwrap().len();
    let mut new_grid = vec![];

    for y in 0..grid.len() {
        let mut new_row = vec![];
        for x in 0..width {
            let pos = match grid[y][x] {
                'L' => empty_mutation(y, x, &grid),
                '#' => occupied_mutation(y, x, &grid),
                c => c,
            };
            new_row.push(pos);
        }
        new_grid.push(new_row);
    }

    new_grid
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn empty_mutation_1(y: usize, x: usize, grid: &Vec<Vec<char>>) -> char {
    if num_adjecent('#', y, x, &grid) == 0 {
        '#'
    } else {
        'L'
    }
}

fn occupied_mutation_1(y: usize, x: usize, grid: &Vec<Vec<char>>) -> char {
    if num_adjecent('#', y, x, &grid) >= 4 {
        'L'
    } else {
        '#'
    }
}

fn solve1(filename: &str) -> u64 {
    let mut grid = build_grid(&lines_from_file(filename));
    //print_grid(&grid);

    loop {
        let new_grid = mutate_grid(&grid, &empty_mutation_1, &occupied_mutation_1);
        //print_grid(&new_grid);
        //println!("{}", count_char('#', &new_grid));
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }

    count_char('#', &grid)
}

fn num_visible_adjecent(chr: char, y: usize, x: usize, grid: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    let height = grid.len() as i64;
    let width = grid.first().unwrap().len() as i64;

    for pos in NEIGHBOURS.iter() {
        let mut new_y = pos.0 + y as i64;
        let mut new_x = pos.1 + x as i64;
        while new_y >= 0 && new_y < height && new_x >= 0 && new_x < width {
            let found = grid[new_y as usize][new_x as usize];
            if found != '.' {
                if found == chr {
                    sum += 1;
                }
                break;
            }
            new_y += pos.0;
            new_x += pos.1;
        }
    }

    sum
}

fn empty_mutation_2(y: usize, x: usize, grid: &Vec<Vec<char>>) -> char {
    if num_visible_adjecent('#', y, x, &grid) == 0 {
        '#'
    } else {
        'L'
    }
}

fn occupied_mutation_2(y: usize, x: usize, grid: &Vec<Vec<char>>) -> char {
    if num_visible_adjecent('#', y, x, &grid) >= 5 {
        'L'
    } else {
        '#'
    }
}

fn solve2(filename: &str) -> u64 {
    let mut grid = build_grid(&lines_from_file(filename));
    //print_grid(&grid);

    loop {
        let new_grid = mutate_grid(&grid, &empty_mutation_2, &occupied_mutation_2);
        //print_grid(&new_grid);
        //println!("{}", count_char('#', &new_grid));
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }

    count_char('#', &grid)
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
        assert_eq!(solve1("example.txt"), 37);
        assert_eq!(solve2("example.txt"), 26);
    }
}
