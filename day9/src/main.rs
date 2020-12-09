use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[allow(dead_code)]
struct Result {
    i1: i64,
    i2: i64,
}

fn find_first_not_sum_previous(numbers: &Vec<i64>, preamble: usize) -> i64 {
    let mut idx = 0;

    while idx + preamble + 1 < numbers.len() {
        let mut sums: Vec<Result> = vec![];
        for i in &numbers[idx..idx + preamble] {
            for j in &numbers[idx..idx + preamble] {
                if i != j && i + j == numbers[idx + preamble] {
                    sums.push(Result { i1: *i, i2: *j });
                }
            }
        }

        if sums.len() == 0 {
            return numbers[idx + preamble];
        }
        idx += 1;
    }

    panic!("Could not find number in list that met requirements!")
}

fn solve1(filename: &str) -> i64 {
    find_first_not_sum_previous(&ints_from_file(filename), 25)
}

fn find_min_max_in_cont_sum(numbers: &Vec<i64>, wanted_sum: i64) -> i64 {
    for start in 0..numbers.len() {
        let mut sum = 0;
        let mut min = std::i64::MAX;
        let mut max = std::i64::MIN;
        for num in &numbers[start..] {
            if num < &min {
                min = *num;
            } else if num > &max {
                max = *num;
            }
            sum += num;
            if sum == wanted_sum {
                return min + max;
            }
        }
    }
    panic!("Could not find continous range that met requirements!")
}

fn solve2(filename: &str) -> i64 {
    find_min_max_in_cont_sum(&ints_from_file(filename), 675280050)
}

fn ints_from_file(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i64>().unwrap())
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
        assert_eq!(
            find_first_not_sum_previous(&ints_from_file("example.txt"), 5),
            127
        );
        assert_eq!(
            find_min_max_in_cont_sum(&ints_from_file("example.txt"), 127),
            62
        );
    }
}
