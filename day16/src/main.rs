use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Range {
    lo: u64,
    hi: u64,
}

#[derive(Debug)]
struct Rule {
    name: String,
    range1: Range,
    range2: Range,
}

fn parse_rule(line: &str) -> Rule {
    let splits: Vec<&str> = line.split(": ").collect();
    let ranges: Vec<&str> = splits[1].split(" or ").collect();
    let range1: Vec<u64> = ranges[0]
        .split("-")
        .map(|i| i.parse::<u64>().unwrap())
        .collect();
    let range2: Vec<u64> = ranges[1]
        .split("-")
        .map(|i| i.parse::<u64>().unwrap())
        .collect();
    Rule {
        name: splits[0].to_string(),
        range1: Range {
            lo: range1[0],
            hi: range1[1],
        },
        range2: Range {
            lo: range2[0],
            hi: range2[1],
        },
    }
}

fn parse_ticket(input: &str) -> Vec<u64> {
    input
        .split(",")
        .map(|t| t.parse::<u64>().unwrap())
        .collect()
}

fn rule_is_valid(rule: &Rule, field: &u64) -> bool {
    (*field >= rule.range1.lo && *field <= rule.range1.hi)
        || (*field >= rule.range2.lo && *field <= rule.range2.hi)
}

fn invalid_sum(ticket: &Vec<u64>, rules: &Vec<Rule>) -> u64 {
    let mut invalid_sum = 0;

    for field in ticket {
        let mut valid = false;
        for rule in rules {
            if rule_is_valid(&rule, field) {
                valid |= true;
            }
        }
        if !valid {
            invalid_sum += field;
        }
    }

    invalid_sum
}

fn solve1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let rules_input = input.split("\n\nyour ticket:\n").collect::<Vec<&str>>()[0].split("\n");
    let ticket_input = input.split("\n\nnearby tickets:\n").collect::<Vec<&str>>()[1].split("\n");

    let mut rules: Vec<Rule> = vec![];
    for line in rules_input {
        rules.push(parse_rule(&line));
    }

    let tickets: Vec<Vec<u64>> = ticket_input.map(|t| parse_ticket(t)).collect();
    tickets.iter().map(|t| invalid_sum(&t, &rules)).sum()
}

fn solve2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let rules_input = input.split("\n\nyour ticket:\n").collect::<Vec<&str>>()[0].split("\n");
    let ticket_input = input.split("\n\nnearby tickets:\n").collect::<Vec<&str>>()[1].split("\n");

    let mut rules: Vec<Rule> = vec![];
    for line in rules_input {
        rules.push(parse_rule(&line));
    }

    let tickets: Vec<Vec<u64>> = ticket_input
        .map(|t| parse_ticket(t))
        .filter(|t| invalid_sum(&t, &rules) == 0)
        .collect();

    let rules_set = rules
        .iter()
        .map(|r| r.name.as_ref())
        .collect::<HashSet<&str>>();
    let mut rules_pos_to_set: Vec<HashSet<&str>> = vec![];
    for _ in 0..tickets[0].len() {
        rules_pos_to_set.push(rules_set.clone());
    }

    for ticket in tickets {
        for (i, field) in ticket.iter().enumerate() {
            for rule in &rules {
                // if a rule does not match a position, remove that from the set of possible rules
                if !rule_is_valid(&rule, field) {
                    rules_pos_to_set[i].remove(&rule.name.as_ref());
                }
            }
        }
    }

    // reduce these two rulesets by hand, sudoku-time!
    for i in 0..rules_pos_to_set.len() {
        println!(
            "{} {:?}",
            i,
            rules_pos_to_set[i]
                .iter()
                .filter(|r| !(*r).starts_with("departure") || (*r) == &"departure platform")
                .map(|r| *r)
                .collect::<Vec<&str>>()
        );
    }
    for i in 0..rules_pos_to_set.len() {
        println!(
            "{} {:?}",
            i,
            rules_pos_to_set[i]
                .iter()
                .filter(|r| (*r).starts_with("departure"))
                .map(|r| *r)
                .collect::<Vec<&str>>()
        );
    }

    /*
    0 ["arrival track", "departure platform", "type", "arrival platform", "arrival station"]
    1 ["arrival track", "train", "departure platform", "route", "type", "arrival platform", "arrival station"]
    2 ["departure platform"]
    3 ["duration"]
    4 ["arrival track", "type", "arrival station"]
    5 ["arrival track", "train", "departure platform", "row", "route", "type", "arrival platform", "wagon", "price", "arrival station"]
    6 ["arrival track", "type", "arrival station"]
    7 ["arrival track", "type", "arrival station"]
    8 ["arrival track", "type"]
    9 ["arrival track", "train", "departure platform", "row", "route", "type", "arrival platform", "wagon", "arrival station"]
    10 ["seat"]
    11 ["arrival track", "type", "arrival station"]
    12 ["class"]
    13 ["arrival track", "train", "departure platform", "row", "route", "type", "arrival platform", "arrival station"]
    14 ["arrival track", "type", "arrival station"]
    15 []
    16 ["arrival track", "type", "arrival station"]
    17 ["arrival track", "departure platform", "route", "type", "arrival platform", "arrival station"]
    18 ["arrival track", "arrival location", "train", "departure platform", "row", "route", "type", "arrival platform", "wagon", "price", "arrival station"]
    19 ["arrival track", "arrival location", "train", "departure platform", "row", "route", "type", "arrival platform", "wagon", "price", "arrival station", "zone"]
    */

    /*
    0 ["departure platform"] #
    1 ["departure platform"] #
    2 ["departure platform"] # <---------------------
    3 []
    4 ["departure track"]
    5 ["departure platform"] #
    6 []
    7 ["departure date"]
    8 []
    9 ["departure platform"] #
    10 []
    11 ["departure location"]
    12 []
    13 ["departure platform"] #
    14 ["departure station"]
    15 []
    16 ["departure time"]
    17 ["departure platform"] #
    18 ["departure platform"] #
    19 ["departure platform"] #
    */

    // 2, 4, 7, 11, 14, 16
    let my_ticket = vec![
        97, 103, 89, 191, 73, 79, 83, 101, 151, 71, 149, 53, 181, 59, 61, 67, 113, 109, 107, 127,
    ];

    my_ticket[2] * my_ticket[4] * my_ticket[7] * my_ticket[11] * my_ticket[14] * my_ticket[16]
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
        assert_eq!(solve1("example.txt"), 71);
    }
}
