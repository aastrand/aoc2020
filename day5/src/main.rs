use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::HashSet;

struct Range {
    lo: u64,
    hi: u64,
}

fn reduce(range: &mut Range, op: char) -> &Range {
    match op {
        // front, left
        'F' | 'L' => range.hi = (range.lo + range.hi) / 2,
        // back, right
        'B' | 'R' => range.lo = ((range.lo + range.hi) / 2) + 1,
        _ => panic!("Unknown op: {}", op),
    }

    range
}

fn code_to_id(line: &str) -> u64 {
    let mut row = Range { lo: 0, hi: 127 };
    for op in line[0..7].chars() {
        reduce(&mut row, op);
        //println!("{} {} {}", op, row.lo, row.hi);
    }
    let mut col = Range { lo: 0, hi: 7 };
    for op in line[7..10].chars() {
        reduce(&mut col, op);
        //println!("{} {} {}", op, col.lo, col.hi);
    }
    (row.lo * 8) + col.hi
}

fn solve1(filename: &str) -> u64 {
    lines_from_file(filename)
        .iter()
        .map(|line| code_to_id(&line))
        .max()
        .unwrap()
}

fn solve2(filename: &str) -> u64 {
    let mut seats: HashSet<u64> = HashSet::new();

    for row in 0..128 {
        for col in 0..8 {
            seats.insert(row * 8 + col);
        }
    }

    for line in lines_from_file(filename) {
        seats.remove(&code_to_id(&line));
    }

    *seats.iter().filter(|&x| *x > 100 && *x < 900).max().unwrap()
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("{}", solve1("../input/2020/day5.txt"));
    println!("{}", solve2("../input/2020/day5.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 820);
        //assert_eq!(solve2("example.txt"), 336);
    }

    #[test]
    fn test_reduce() {
        let mut range = Range { lo: 0, hi: 127 };

        /*
        For example, consider just the first seven characters of FBFBBFFRLR:

        Start by considering the whole range, rows 0 through 127.
        F means to take the lower half, keeping rows 0 through 63.
        B means to take the upper half, keeping rows 32 through 63.
        F means to take the lower half, keeping rows 32 through 47.
        B means to take the upper half, keeping rows 40 through 47.
        B keeps rows 44 through 47.
        F keeps rows 44 through 45.
        The final F keeps the lower of the two, row 44.
        */
        reduce(&mut range, 'F');
        assert_eq!(range.lo, 0);
        assert_eq!(range.hi, 63);

        reduce(&mut range, 'B');
        assert_eq!(range.lo, 32);
        assert_eq!(range.hi, 63);

        reduce(&mut range, 'F');
        assert_eq!(range.lo, 32);
        assert_eq!(range.hi, 47);

        reduce(&mut range, 'B');
        assert_eq!(range.lo, 40);
        assert_eq!(range.hi, 47);

        reduce(&mut range, 'B');
        assert_eq!(range.lo, 44);
        assert_eq!(range.hi, 47);

        reduce(&mut range, 'F');
        assert_eq!(range.lo, 44);
        assert_eq!(range.hi, 45);

        reduce(&mut range, 'F');
        assert_eq!(range.lo, 44);
        assert_eq!(range.hi, 44);
    }

    #[test]
    fn test_reduce_col() {
        let mut range = Range { lo: 0, hi: 7 };

        /*
        For example, consider just the last 3 characters of FBFBBFFRLR:

        Start by considering the whole range, columns 0 through 7.
        R means to take the upper half, keeping columns 4 through 7.
        L means to take the lower half, keeping columns 4 through 5.
        The final R keeps the upper of the two, column 5.
        */
        reduce(&mut range, 'R');
        assert_eq!(range.lo, 4);
        assert_eq!(range.hi, 7);

        reduce(&mut range, 'L');
        assert_eq!(range.lo, 4);
        assert_eq!(range.hi, 5);

        reduce(&mut range, 'R');
        assert_eq!(range.lo, 5);
        assert_eq!(range.hi, 5);
    }

    #[test]
    fn test_code_to_id() {
        assert_eq!(code_to_id("FBFBBFFRLR"), 357);
        assert_eq!(code_to_id("BFFFBBFRRR"), 567);
        assert_eq!(code_to_id("FFFBBBFRRR"), 119);
        assert_eq!(code_to_id("BBFFBBFRLL"), 820);
    }
}
