use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn solve1(filename: &str) -> i32 {
    let contents = lines_from_file(filename);
    let mut result = 0;

    for left in 0..contents.len() {
        for right in 0..contents.len() {
            let l = contents[left].parse::<i32>().unwrap();
            let r = contents[right].parse::<i32>().unwrap();
            if l != r && l + r == 2020 {
                result = l * r
            }
        }
    }

    result
}

fn solve2(filename: &str) -> i32 {
    let contents = lines_from_file(filename);
    let mut result = 0;

    for left in 0..contents.len() {
        for right in 0..contents.len() {
            for mid in 0..contents.len() {
                let l = contents[left].parse::<i32>().unwrap();
                let r = contents[right].parse::<i32>().unwrap();
                let m = contents[mid].parse::<i32>().unwrap();
                if l != r && r != m && l + r + m == 2020 {
                    result = l * r * m;
                }
            }
        }
    }

    result
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("{}", solve1("../input/2020/day1.txt"));
    println!("{}", solve2("../input/2020/day1.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 514579);
        assert_eq!(solve2("example.txt"), 241861950);
    }
}
