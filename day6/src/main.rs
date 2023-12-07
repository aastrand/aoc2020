use std::fs;

use std::collections::HashSet;

fn solve1(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .replace("\n", "")
                .chars()
                .collect::<HashSet<char>>()
                .len() as u64
        })
        .sum()
}

fn solve2(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|group| {
            let mut sets = group
                .split("\n")
                .map(|set| set.chars().collect::<HashSet<char>>());

            sets.next()
                .map(|set| {
                    sets.fold(set, |set1, set2| {
                        set1.intersection(&set2)
                            .map(|c| *c)
                            .collect::<HashSet<char>>()
                    })
                })
                .unwrap()
                .len() as u64
        })
        .sum()
}

fn main() {
    println!("{}", solve1("../input/2020/day6.txt"));
    println!("{}", solve2("../input/2020/day6.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 11);
        assert_eq!(solve2("example.txt"), 6);
    }
}
