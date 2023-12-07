use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::HashMap;

// 3 planes of 9, remove yourself => 26
const NEIGHBOURS: [(i64, i64, i64); 26] = [
    // front plane
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
    // back plane
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    // middle plane
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    //(0, 0, 0),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
];

#[derive(Debug)]
struct Space3 {
    space: HashMap<(i64, i64, i64), char>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Space3 {
    pub fn new() -> Space3 {
        Space3 {
            space: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        }
    }

    fn from_lines(lines: &Vec<String>) -> Space3 {
        let mut space = HashMap::new();
        let mut max_y = 0;
        let mut max_x = 0;
        for (y, line) in lines.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                space.insert((x as i64, y as i64, 0), chr);
                max_x = x;
            }
            max_y = y;
        }

        Space3 {
            space: space,
            max_x: max_x as i64,
            max_y: max_y as i64,
            max_z: 0,
            min_x: 0,
            min_y: 0,
            min_z: 0,
        }
    }

    fn get(&self, x: i64, y: i64, z: i64) -> char {
        if let Some(pos) = self.space.get(&(x, y, z)) {
            return *pos;
        }

        return '.';
    }

    fn put(&mut self, x: i64, y: i64, z: i64, chr: char) {
        self.space.insert((x, y, z), chr);
        if chr == '#' {
            if x < self.min_x {
                self.min_x = x;
            }
            if x > self.max_x {
                self.max_x = x;
            }
            if y < self.min_y {
                self.min_y = y;
            }
            if y > self.max_y {
                self.max_y = y;
            }
            if z < self.min_z {
                self.min_z = z;
            }
            if z > self.max_z {
                self.max_z = z;
            }
        }
    }

    fn active_neighbours(&self, x: i64, y: i64, z: i64) -> u64 {
        let mut sum = 0;
        for pos in NEIGHBOURS.iter() {
            if self.get(x + pos.2, y + pos.1, z + pos.0) == '#' {
                sum += 1;
            }
        }
        sum
    }

    fn active_cubes(&self) -> u64 {
        self.space
            .values()
            .filter(|p| *p == &'#')
            .collect::<Vec<&char>>()
            .len() as u64
    }

    fn cycle(&mut self) {
        let mut new_space = Space3::new();

        for z in self.min_z - 1..self.max_z + 2 {
            for y in self.min_y - 1..self.max_y + 2 {
                for x in self.min_x - 1..self.max_x + 2 {
                    let num_active_neighbours = self.active_neighbours(x, y, z);
                    if self.get(x, y, z) == '#' {
                        if num_active_neighbours == 2 || num_active_neighbours == 3 {
                            new_space.put(x, y, z, '#');
                        } else {
                            new_space.put(x, y, z, '.');
                        }
                    } else if num_active_neighbours == 3 {
                        new_space.put(x, y, z, '#');
                    } else {
                        new_space.put(x, y, z, '.');
                    }
                }
            }
        }

        *self = new_space
    }
}

fn solve1(filename: &str) -> u64 {
    let mut space = Space3::from_lines(&lines_from_file(filename));
    for _ in 0..6 {
        space.cycle()
    }

    space.active_cubes()
}

// https://stackoverflow.com/questions/45990454/generating-all-possible-combinations-of-characters-in-a-string/45990492
const NEIGHBOURS_4D: [(i64, i64, i64, i64); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    //(0, 0, 0, 0),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

#[derive(Debug)]
struct Space4 {
    space: HashMap<(i64, i64, i64, i64), char>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
    min_w: i64,
    max_w: i64,
}

impl Space4 {
    pub fn new() -> Space4 {
        Space4 {
            space: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
        }
    }

    fn from_lines(lines: &Vec<String>) -> Space4 {
        let mut space = HashMap::new();
        let mut max_y = 0;
        let mut max_x = 0;
        for (y, line) in lines.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                space.insert((x as i64, y as i64, 0, 0), chr);
                max_x = x;
            }
            max_y = y;
        }

        Space4 {
            space,
            max_x: max_x as i64,
            max_y: max_y as i64,
            max_z: 0,
            min_x: 0,
            min_y: 0,
            min_z: 0,
            min_w: 0,
            max_w: 0,
        }
    }

    fn get(&self, x: i64, y: i64, z: i64, w: i64) -> char {
        if let Some(pos) = self.space.get(&(x, y, z, w)) {
            return *pos;
        }

        return '.';
    }

    fn put(&mut self, x: i64, y: i64, z: i64, w: i64, chr: char) {
        self.space.insert((x, y, z, w), chr);
        if chr == '#' {
            if x < self.min_x {
                self.min_x = x;
            }
            if x > self.max_x {
                self.max_x = x;
            }
            if y < self.min_y {
                self.min_y = y;
            }
            if y > self.max_y {
                self.max_y = y;
            }
            if z < self.min_z {
                self.min_z = z;
            }
            if z > self.max_z {
                self.max_z = z;
            }
            if w < self.min_w {
                self.min_w = w;
            }
            if w > self.max_w {
                self.max_w = w;
            }
        }
    }

    fn active_neighbours(&self, x: i64, y: i64, z: i64, w: i64) -> u64 {
        let mut sum = 0;
        for pos in NEIGHBOURS_4D.iter() {
            if self.get(x + pos.3, y + pos.2, z + pos.1, w + pos.0) == '#' {
                sum += 1;
            }
        }
        sum
    }

    fn active_cubes(&self) -> u64 {
        self.space
            .values()
            .filter(|p| *p == &'#')
            .collect::<Vec<&char>>()
            .len() as u64
    }

    fn cycle(&mut self) {
        let mut new_space = Space4::new();

        for w in self.min_w - 1..self.max_w + 2 {
            for z in self.min_z - 1..self.max_z + 2 {
                for y in self.min_y - 1..self.max_y + 2 {
                    for x in self.min_x - 1..self.max_x + 2 {
                        let num_active_neighbours = self.active_neighbours(x, y, z, w);
                        let chr = if self.get(x, y, z, w) == '#' {
                            if num_active_neighbours == 2 || num_active_neighbours == 3 {
                                '#'
                            } else {
                                '.'
                            }
                        } else if num_active_neighbours == 3 {
                            '#'
                        } else {
                            '.'
                        };
                        new_space.put(x, y, z, w, chr);
                    }
                }
            }
        }

        *self = new_space
    }
}

fn solve2(filename: &str) -> u64 {
    let mut space = Space4::from_lines(&lines_from_file(filename));
    for _ in 0..6 {
        space.cycle()
    }

    space.active_cubes()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("{}", solve1("../input/2020/day17.txt"));
    println!("{}", solve2("../input/2020/day17.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 112);
        assert_eq!(solve2("example.txt"), 848);
    }
}
