use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    East,
    West,
    Southwest,
    Southeast,
    Northwest,
    Northeast,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    dir: Direction,
    offset: (i64, i64),
}

lazy_static! {
    static ref MOVE_LOOKUP: HashMap<&'static str, Move> = [
        (
            "e",
            Move {
                dir: Direction::East,
                offset: (1, 0)
            }
        ),
        (
            "w",
            Move {
                dir: Direction::West,
                offset: (-1, 0)
            }
        ),
        (
            "sw",
            Move {
                dir: Direction::Southwest,
                offset: (0, -1)
            }
        ),
        (
            "se",
            Move {
                dir: Direction::Southeast,
                offset: (1, -1)
            }
        ),
        (
            "nw",
            Move {
                dir: Direction::Northwest,
                offset: (-1, 1)
            }
        ),
        (
            "ne",
            Move {
                dir: Direction::Northeast,
                offset: (0, 1)
            }
        ),
    ]
    .iter()
    .copied()
    .collect();
}

fn line_to_moves<'a>(line: &String) -> Vec<&'a Move> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut moves = vec![];
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            'e' => moves.push(MOVE_LOOKUP.get("e").unwrap()),
            'w' => moves.push(MOVE_LOOKUP.get("w").unwrap()),
            's' => {
                match chars[i + 1] {
                    'e' => moves.push(MOVE_LOOKUP.get("se").unwrap()),
                    'w' => moves.push(MOVE_LOOKUP.get("sw").unwrap()),
                    _ => panic!("Unexpected char: {}", chars[i]),
                }
                i += 1;
            }
            'n' => {
                match chars[i + 1] {
                    'e' => moves.push(MOVE_LOOKUP.get("ne").unwrap()),
                    'w' => moves.push(MOVE_LOOKUP.get("nw").unwrap()),
                    _ => panic!("Unexpected char: {}", chars[i]),
                }
                i += 1;
            }
            _ => panic!("Unexpected char: {}", chars[i]),
        }
        i += 1;
    }

    moves
}

fn parse_tiles(input: &Vec<String>) -> HashMap<(i64, i64), u64> {
    let mut tiles: HashMap<(i64, i64), u64> = HashMap::new();
    for line in input {
        let mut pos = (0, 0);
        for m in line_to_moves(line) {
            pos.0 += m.offset.0;
            pos.1 += m.offset.1;
        }
        let mut count = *tiles.get(&pos).unwrap_or(&0);
        count += 1;
        tiles.insert(pos, count);
    }
    tiles
}

fn solve1(filename: &str) -> u64 {
    let tiles = parse_tiles(&lines_from_file(filename));
    tiles.values().map(|v| v % 2).sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Color {
    White,
    Black,
}

impl Color {
    fn to_color(value: u64) -> Color {
        match value {
            0 => Color::White,
            1 => Color::Black,
            c => panic!("Unexpected color value: {}", c),
        }
    }
}

struct Tiles {
    tiles: HashMap<(i64, i64), u64>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Tiles {
    fn new(tiles: HashMap<(i64, i64), u64>) -> Tiles {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;

        for tile in tiles.keys() {
            if tile.0 < min_x {
                min_x = tile.0;
            }
            if tile.0 > max_x {
                max_x = tile.0;
            }
            if tile.1 < min_y {
                min_y = tile.1;
            }
            if tile.1 > max_y {
                max_y = tile.1;
            }
        }

        Tiles {
            tiles: tiles,
            min_x: min_x - 1,
            max_x: max_x + 1,
            min_y: min_y - 1,
            max_y: max_y + 1,
        }
    }

    fn color_of(&self, pos: &(i64, i64)) -> Color {
        let color = *self.tiles.get(pos).unwrap_or(&0) % 2;
        Color::to_color(color)
    }

    fn black_neighbours(&self, pos: (i64, i64)) -> u64 {
        let mut sum = 0;

        for neighbour in MOVE_LOOKUP.values() {
            if self.color_of(&(pos.0 + neighbour.offset.0, pos.1 + neighbour.offset.1))
                == Color::Black
            {
                sum += 1;
            }
        }

        sum
    }

    fn evolve(&mut self) {
        let mut new_tiles: HashMap<(i64, i64), u64> = self.tiles.clone();
        for y in self.min_y - 1..self.max_y + 2 {
            for x in self.min_x - 1..self.max_x + 2 {
                let color = self.color_of(&(x, y));
                let bn = self.black_neighbours((x, y));
                if color == Color::Black {
                    // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
                    if bn == 0 || bn > 2 {
                        new_tiles.insert((x, y), 0);
                    }
                } else {
                    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
                    if bn == 2 {
                        new_tiles.insert((x, y), 1);
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
                    }
                }
            }
        }

        self.tiles = new_tiles;
    }
}

fn solve2(filename: &str) -> u64 {
    let mut tiles = Tiles::new(parse_tiles(&lines_from_file(filename)));
    for day in 0..100 {
        tiles.evolve();
        println!(
            "Day {}: {}",
            day + 1,
            tiles.tiles.values().map(|v| v % 2).sum::<u64>()
        )
    }

    tiles.tiles.values().map(|v| v % 2).sum()
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
        assert_eq!(solve1("example.txt"), 10);
        assert_eq!(solve2("example.txt"), 2208);
    }

    #[test]
    fn test_line_to_moves() {
        // esenee identifies the tile you land on if you start at the reference tile
        // and then move one tile east, one tile southeast, one tile northeast, and one tile east
        let moves = line_to_moves(&"esenee".to_string());
        assert_eq!(moves.len(), 4);
        assert_eq!(moves[0].dir, Direction::East);
        assert_eq!(moves[1].dir, Direction::Southeast);
        assert_eq!(moves[2].dir, Direction::Northeast);
        assert_eq!(moves[3].dir, Direction::East);

        let mut pos = (0, 0);
        for m in &moves {
            pos.0 += m.offset.0;
            pos.1 += m.offset.1;
        }
        assert_eq!(pos, (3, 0));

        let moves = line_to_moves(&"nwwswee".to_string());
        // northwest, west, southwest, east, east
        assert_eq!(moves.len(), 5);
        assert_eq!(moves[0].dir, Direction::Northwest);
        assert_eq!(moves[1].dir, Direction::West);
        assert_eq!(moves[2].dir, Direction::Southwest);
        assert_eq!(moves[3].dir, Direction::East);
        assert_eq!(moves[4].dir, Direction::East);

        // a line like nwwswee flips the reference tile itself.
        let mut pos = (0, 0);
        for m in moves {
            pos.0 += m.offset.0;
            pos.1 += m.offset.1;
        }
        assert_eq!(pos, (0, 0));
    }
}
