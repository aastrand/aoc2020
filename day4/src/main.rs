use std::collections::HashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

extern crate regex;
use regex::Regex;

use std::collections::HashSet;

fn get_maps(filename: &str) -> Vec<HashMap<String, String>> {
    let mut maps = vec![];
    let mut map: HashMap<String, String> = HashMap::new();
    for line in lines_from_file(filename) {
        if line.len() == 0 {
            maps.push(map);
            map = HashMap::new();
        } else {
            for pair in line.trim().split(" ") {
                let mut kv = pair.split(":");
                map.insert(
                    kv.next().unwrap().to_string(),
                    kv.next().unwrap().to_string(),
                );
            }
        }
    }
    // don't forget the last one
    maps.push(map);

    maps
}

fn solve1(filename: &str) -> u64 {
    let maps = get_maps(filename);
    let mut result = 0;

    /*let req_fields: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    .iter()
    .cloned()
    .collect();*/

    for map in maps {
        let cid = if map.contains_key("cid") { 1 } else { 0 };
        //let keys: HashSet<&str> = map.keys().map(|s| &s[..]).collect();

        if map.keys().len() == 7 + cid {
            result += 1;
        }
    }

    result
}

fn solve2(filename: &str) -> u64 {
    let maps = get_maps(filename);
    let mut result = 0;

    for map in maps {
        if valid(map) {
            result += 1;
        }
    }

    result
}

fn valid(map: HashMap<String, String>) -> bool {
    let cid = if map.contains_key("cid") { 1 } else { 0 };

    map.keys().len() == 7 + cid
        && validate_year(&map["byr"], 1920, 2002)
        && validate_year(&map["iyr"], 2010, 2020)
        && validate_year(&map["eyr"], 2020, 2030)
        && validate_hgt(&map["hgt"])
        && validate_ecl(&map["ecl"])
        && validate_hcl(&map["hcl"])
        && validate_pid(&map["pid"])
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn validate_year(byr: &str, low: i32, high: i32) -> bool {
    let digit = byr.parse::<i32>().unwrap();
    digit >= low && digit <= high
}

/*
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
*/
fn validate_hgt(hgt: &str) -> bool {
    let re = Regex::new(r"([0-9]+)(cm|in)").unwrap();

    if let Some(cap) = re.captures(&hgt) {
        let hgt = cap[1].parse::<i32>().unwrap();

        match &cap[2] {
            "cm" => hgt >= 150 && hgt <= 193,
            "in" => hgt >= 59 && hgt <= 76,
            _ => false,
        }
    } else {
        false
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn validate_hcl(hcl: &str) -> bool {
    let re = Regex::new(r"^\#([0-9a-f]{6})$").unwrap();

    re.is_match(hcl)
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn validate_ecl(ecl: &str) -> bool {
    let colors: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();

    colors.contains(ecl.trim())
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn validate_pid(pid: &str) -> bool {
    let re = Regex::new(r"^([0-9]{9})$").unwrap();

    re.is_match(pid)
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
    println!("{}", solve1("mannen.txt"));
    println!("{}", solve2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 2);
        assert_eq!(solve2("example2.txt"), 0);
        assert_eq!(solve2("example3.txt"), 4);
    }

    #[test]
    fn test_validate_byr() {
        assert_eq!(validate_year("2002", 1920, 2002), true);
        assert_eq!(validate_year("2003", 1920, 2002), false);
        assert_eq!(validate_year("200", 1920, 2002), false);
        assert_eq!(validate_year("20009", 1920, 2002), false);
    }

    #[test]
    fn test_validate_hgt() {
        assert_eq!(validate_hgt("60in"), true);
        assert_eq!(validate_hgt("190cm"), true);
        assert_eq!(validate_hgt("190in"), false);
        assert_eq!(validate_hgt("190"), false);
    }

    #[test]
    fn test_validate_ecl() {
        assert_eq!(validate_ecl("brn"), true);
        assert_eq!(validate_ecl("wat"), false);
    }

    #[test]
    fn test_validate_pid() {
        assert_eq!(validate_pid("000000001"), true);
        assert_eq!(validate_pid("0123456789"), false);
    }
}
