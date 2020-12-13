use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn solve1(filename: &str) -> i64 {
    0
}

fn solve2(filename: &str) -> i64 {
    0
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
