use std::collections::HashMap;

fn solve1(start: Vec<u64>, max_turn: u64) -> u64 {
    let mut said: HashMap<u64, u64> = HashMap::new();
    let mut last = *start.last().unwrap();
    for n in 1..start.len() + 1 {
        said.insert(start[n - 1], n as u64);
    }

    for turn in (start.len() as u64 + 1)..max_turn + 1 {
        let mut new = 0;
        if let Some(when) = said.get(&last) {
            new = turn - 1 - when;
        }
        said.insert(last, turn - 1);
        last = new;
    }

    last
}

fn main() {
    println!("{}", solve1(vec![0, 13, 1, 16, 6, 17], 2020));
    println!("{}", solve1(vec![0, 13, 1, 16, 6, 17], 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1(vec![0, 3, 6], 2020), 436);
        assert_eq!(solve1(vec![1, 3, 2], 2020), 1);
        assert_eq!(solve1(vec![2, 1, 3], 2020), 10);
        assert_eq!(solve1(vec![1, 2, 3], 2020), 27);
        assert_eq!(solve1(vec![2, 3, 1], 2020), 78);
        assert_eq!(solve1(vec![3, 2, 1], 2020), 438);
        assert_eq!(solve1(vec![3, 1, 2], 2020), 1836);
    }
}
