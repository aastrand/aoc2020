use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn solve1(filename: &str) -> i64 {
    let lines = lines_from_file(filename);
    let ts = lines[0].parse::<i64>().unwrap();
    let buses: Vec<i64> = lines[1]
        .split(",")
        .filter(|b| *b != "x")
        .map(|b| b.parse::<i64>().unwrap())
        .collect();

    //println!("{} {:?}", ts, buses);

    // version with iterators. unreadable, or? 
    // am i so out of touch? no. it's the children who are wrong.
    /*buses
        .iter()
        .map(|bus| [((timestamp % bus) - bus).abs(), *bus])
        .min_by_key(|t| t[0])
        .unwrap()
        .iter()
        .product::<i64>()*/

    let mut min_waiting: i64 = std::i64::MAX;
    let mut chosen_bus = -1;
    for bus in buses {
        let waiting_time = ((ts % bus) - bus).abs();
        if waiting_time < min_waiting {
            min_waiting = waiting_time;
            chosen_bus = bus;
        }
    }

    min_waiting * chosen_bus
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn solve2(filename: &str) -> i64 {
    let lines = lines_from_file(filename);
    let buses: Vec<(i64, i64)> = lines[1]
        .split(",")
        .enumerate()
        .filter(|t| t.1 != "x")
        .map(|t| {
            (
                t.1.parse::<i64>().unwrap() - t.0 as i64,
                t.1.parse::<i64>().unwrap(),
            )
        })
        .collect();

    let residues: Vec<i64> = buses.iter().map(|t| t.0).collect();
    let modulii: Vec<i64> = buses.iter().map(|t| t.1).collect();

    //println!("{:?} {:?}", modulii, residues);

    chinese_remainder(&residues, &modulii).unwrap()
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
        assert_eq!(solve1("example.txt"), 295);
        assert_eq!(solve2("example.txt"), 1068781);
    }

    #[test]
    fn test_chinese_remainder() {
        let modulii = vec![17, 13, 19];
        let residues = vec![0, 11, 16];
        assert_eq!(chinese_remainder(&residues, &modulii).unwrap(), 3417);

        let modulii = vec![67, 7, 59, 61];
        let residues = vec![0, 6, 57, 58];
        assert_eq!(chinese_remainder(&residues, &modulii).unwrap(), 754018);
    }
}
