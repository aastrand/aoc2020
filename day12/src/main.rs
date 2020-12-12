use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const DIRECTIONS: [char; 4] = ['E', 'S', 'W', 'N'];

#[derive(Debug)]
struct Move {
    direction: char,
    amount: i64,
}

#[derive(Debug)]
struct Ship {
    facing_direction: usize,
    x: i64,
    y: i64,
}

impl Ship {
    fn rotate(&mut self, rotation: char, amount: i64) {
        let mut facing = self.facing_direction as i64;
        match rotation {
            'L' => facing -= amount / 90,
            'R' => facing += amount / 90,
            _ => panic!("Unkown rotation: {}", rotation),
        }
        self.facing_direction = ((facing + 4) % 4) as usize;
    }

    fn direction(&self) -> char {
        DIRECTIONS[self.facing_direction]
    }
}

#[derive(Debug)]
struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn rotate(&mut self, rotation: char, amount: i64) {
        for _ in 0..amount / 90 {
            let (x, y) = match rotation {
                // counter clock
                'L' => (self.y, -self.x),
                // clock
                'R' => (-self.y, self.x),
                _ => panic!("Unkown rotation: {}", rotation),
            };
            self.x = x;
            self.y = y;
        }
    }
}

fn parse_moves(lines: &Vec<String>) -> Vec<Move> {
    lines
        .iter()
        .map(|l| Move {
            direction: l[0..1].chars().next().unwrap(),
            amount: l[1..].parse::<i64>().unwrap(),
        })
        .collect()
}

fn to_offset(direction: char, amount: i64) -> (i64, i64) {
    match direction {
        'N' => (0, -amount),
        'S' => (0, amount),
        'W' => (-amount, 0),
        'E' => (amount, 0),
        _ => panic!("Unsupported direction: {}", direction),
    }
}

fn solve1(filename: &str) -> i64 {
    let moves = parse_moves(&lines_from_file(filename));
    let mut ship = Ship {
        facing_direction: 0,
        x: 0,
        y: 0,
    };

    for m in moves {
        let should_move = match m.direction {
            // moves that rotate
            'L' | 'R' => {
                ship.rotate(m.direction, m.amount);
                None
            }
            // rest
            'N' | 'S' | 'E' | 'W' => Some(m.direction),
            // forward
            'F' => Some(ship.direction()),
            m => panic!("Unknown move: {}", m),
        };

        if let Some(direction) = should_move {
            let offset = to_offset(direction, m.amount);
            ship.x += offset.0;
            ship.y += offset.1;
        }
        //println!("ship: {:?}, move: {:?}", ship, m);
    }

    ship.x.abs() + ship.y.abs()
}

fn solve2(filename: &str) -> i64 {
    let moves = parse_moves(&lines_from_file(filename));
    let mut ship = Ship {
        facing_direction: 0,
        x: 0,
        y: 0,
    };
    let mut waypoint = Waypoint { x: 10, y: -1 };

    for m in moves {
        match m.direction {
            // moves that rotate
            'L' | 'R' => waypoint.rotate(m.direction, m.amount),
            // rest
            'N' | 'S' | 'E' | 'W' => {
                let offset = to_offset(m.direction, m.amount);
                waypoint.x += offset.0;
                waypoint.y += offset.1;
            }
            // forward
            'F' => {
                ship.x += waypoint.x * m.amount;
                ship.y += waypoint.y * m.amount;
            }
            m => panic!("Unknown move: {}", m),
        };
        //println!("ship: {:?}, waypoint: {:?}, move: {:?}", ship, waypoint, m);
    }

    ship.x.abs() + ship.y.abs()
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
        assert_eq!(solve1("example.txt"), 25);
        assert_eq!(solve2("example.txt"), 286);
    }
}
