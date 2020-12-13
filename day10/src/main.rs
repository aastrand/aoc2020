use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn solve1(filename: &str) -> i32 {
    let mut nums = ints_from_file(filename);
    nums.push(0);
    nums.sort();
    //println!("{:?}", nums);

    let mut one_diffs = 0;
    let mut three_diffs = 1;

    for i in 0..nums.len() - 1 {
        //println!("{} - {} = {}", &nums[i+1], &nums[i], &nums[i+1] - &nums[i]);
        match &nums[i + 1] - &nums[i] {
            1 => one_diffs += 1,
            3 => three_diffs += 1,
            n => panic!("Unexpected diff: {}", n),
        }
    }

    one_diffs * three_diffs
}

fn multiplier(n: usize) -> u64 {
    match n {
        3 => 7, // 3! + 1
        2 => 4, // all, a, b, none
        1 => 2, // all, none
        _ => panic!("Unsupported n: {}", n),
    }
}

fn solve2(filename: &str) -> u64 {
    let mut nums = ints_from_file(filename);
    nums.push(0);
    nums.sort();
    nums.push(nums.last().unwrap() + 3);

    let mut ones = 0;
    let mut product = 1;

    for i in 0..nums.len() - 1 {
        match &nums[i + 1] - &nums[i] {
            3 => {
                if ones > 1 {
                    product *= multiplier(ones - 1);
                }
                ones = 0;
            }
            1 => ones += 1,
            n => panic!("Unexpected diff: {}", n),
        }
    }

    product
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
        assert_eq!(solve1("example.txt"), 7 * 5);
        assert_eq!(solve1("example2.txt"), 22 * 10);
        assert_eq!(solve2("example.txt"), 8);
        assert_eq!(solve2("example2.txt"), 19208);
    }
}
