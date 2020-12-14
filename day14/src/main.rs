use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::HashMap;

fn solve1(filename: &str) -> u64 {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in lines_from_file(filename) {
        let splits = line.split(" = ").collect::<Vec<&str>>();
        //println!("{:?}", splits);
        match splits[0] {
            "mask" => {
                and_mask = u64::from_str_radix(&splits[1].replace("X", "1"), 2).unwrap();
                or_mask = u64::from_str_radix(&splits[1].replace("X", "0"), 2).unwrap();
            }
            _ => {
                // mem[
                let address = splits[0][4..splits[0].len() - 1].parse::<u64>().unwrap();
                let mut value = splits[1].parse::<u64>().unwrap();
                value &= and_mask;
                value |= or_mask;
                memory.insert(address, value);
            }
        }
    }

    memory.values().sum()
}

fn generate_addresses(addr: &str, mask: &str) -> Vec<u64> {
    let mut addresses = vec![];
    let mut chars: Vec<char> = addr.chars().collect();
    let maskchars: Vec<char> = mask.chars().collect();

    let mut x_positions = vec![];
    for i in 0..chars.len() {
        match maskchars[i] {
            'X' => {
                chars[i] = 'X';
                x_positions.push(i);
            }
            '1' => chars[i] = '1',
            _ => {}
        }
    }
    for i in 0..2 << x_positions.len() {
        let this_mask = &format!("{:0>36b}", i).chars().collect::<Vec<char>>();
        for j in 0..x_positions.len() {
            chars[x_positions[j]] = this_mask[this_mask.len() - 1 - j];
        }
        addresses.push(u64::from_str_radix(&chars.iter().collect::<String>(), 2).unwrap());
    }

    addresses
}

fn solve2(filename: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "".to_string();

    for line in lines_from_file(filename) {
        let splits = line.split(" = ").collect::<Vec<&str>>();
        //println!("{:?}", splits);
        match splits[0] {
            "mask" => {
                mask = splits[1].to_string();
            }
            _ => {
                let address = &format!(
                    "{:0>36b}",
                    &splits[0][4..splits[0].len() - 1].parse::<u64>().unwrap()
                );
                let value = splits[1].parse::<u64>().unwrap();

                for addr in generate_addresses(&address, &mask) {
                    memory.insert(addr, value);
                }
            }
        }
    }

    memory.values().sum()
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
        assert_eq!(solve1("example.txt"), 165);
        assert_eq!(solve2("example2.txt"), 208);
    }
}
