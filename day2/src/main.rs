use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

extern crate regex;
use regex::Regex;

fn solve1(filename: &str) -> i32 {
    let contents = lines_from_file(filename);
    let mut result = 0;
    let re = Regex::new(r"([0-9]+)\-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    for line in contents {
        for cap in re.captures_iter(&line) {
            //println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            let num = num_chars_in_str(cap[3].chars().next().unwrap(), &cap[4]);
            if num >= cap[1].parse::<i32>().unwrap() && num <= cap[2].parse::<i32>().unwrap() {
                result += 1;
            }
        }
    }

    result
}

fn num_chars_in_str(chr: char, str: &str) -> i32 {
    let mut result = 0;
    for c in str.chars() {
        if c == chr {
            result += 1;
        }
    }
    result
}

fn solve2(filename: &str) -> i32 {
    let contents = lines_from_file(filename);
    let mut result = 0;
    let re = Regex::new(r"([0-9]+)\-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    for line in contents {
        for cap in re.captures_iter(&line) {
            //println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            let chars: Vec<char> = cap[4].chars().collect();
            if (chars[cap[1].parse::<usize>().unwrap() - 1] == cap[3].chars().next().unwrap()) 
                ^ (chars[cap[2].parse::<usize>().unwrap() - 1] == cap[3].chars().next().unwrap())
            {
                result += 1;
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
    println!("{}", solve1("input.txt"));
    println!("{}", solve2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 2);
        //assert_eq!(solve2("example.txt"), 241861950);
    }

    #[test]
    fn test_num_chars_in_str() {
        assert_eq!(num_chars_in_str('a', "abcde"), 1);
        assert_eq!(num_chars_in_str('b', "cdefg"), 0);
        assert_eq!(num_chars_in_str('c', "ccccccccc"), 9);
    }
}
