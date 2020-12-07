use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

extern crate regex;
use regex::Regex;

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Edge {
    amount: u64,
    color: String,
}

fn parse_line(line: &str) -> (String, Vec<Edge>) {
    let splits: Vec<&str> = line.split("contain").collect();
    let bag_re = Regex::new(r"([a-z ]+)+ bag[s]?\.?").unwrap();
    let bag = if let Some(cap) = bag_re.captures(splits[0]) {
        cap[1].to_string()
    } else {
        panic!("Could not parse '{}'", splits[0]);
    };

    if splits[1] == " no other bags." {
        (bag, vec![])
    } else {
        let target_re = Regex::new(r"(\d) ([a-z ]+)+ bag[s]?\.?").unwrap();
        let mut targets = vec![];
        for target in splits[1].split(",") {
            if let Some(cap) = target_re.captures(target.trim()) {
                targets.push(Edge {
                    amount: cap[1].parse::<u64>().unwrap(),
                    color: cap[2].to_string(),
                });
            } else {
                panic!("Could not parse '{}'", target);
            }
        }
        (bag, targets)
    }
}

fn build_graph(lines: &Vec<String>) -> HashMap<String, Vec<Edge>> {
    let mut graph: HashMap<String, Vec<Edge>> = HashMap::new();

    for line in lines {
        let (bag, targets) = parse_line(&line);
        graph.insert(bag, targets);
    }

    graph
}

fn dfs(start: &str, graph: &HashMap<String, Vec<Edge>>) -> HashSet<String> {
    let mut stack: Vec<&Edge> = vec![];
    let mut visited: HashSet<String> = HashSet::new();

    for edge in graph.get(start).unwrap() {
        stack.push(&edge);
    }
    while stack.len() > 0 {
        let node = stack.last().unwrap();
        let color = &node.color;
        if !visited.contains(&node.color) {
            for edge in graph.get(&node.color).unwrap() {
                stack.push(&edge);
            }
            visited.insert(color.to_string());
        } else {
            stack.pop();
        }
    }

    visited
}

fn unique_paths_in_dag(
    graph: &HashMap<String, Vec<Edge>>,
    path: Vec<String>,
    mut paths: Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    let node = path.last().unwrap();
    if graph.get(node).unwrap().len() > 0 {
        for edge in graph.get(node).unwrap() {
            let mut new_path = path.clone();
            new_path.push(edge.color.to_string());

            paths = unique_paths_in_dag(graph, new_path, paths);
        }
    } else {
        paths.push(path);
    }

    paths
}

fn solve1(filename: &str) -> u64 {
    let graph = build_graph(&lines_from_file(filename));
    let mut result = 0;

    for bag in graph.keys() {
        let visited = dfs(bag, &graph);
        if visited.contains("shiny gold") {
            result += 1;
        }
    }

    result
}

fn solve2(filename: &str) -> u64 {
    let graph = build_graph(&lines_from_file(filename));
    let mut result = 0;
    let mut amounts: HashMap<(&str, &str), u64> = HashMap::new();
    for from in graph.keys() {
        for to in graph.get(from).unwrap() {
            amounts.insert((&from, &to.color), to.amount);
        }
    }

    let paths = unique_paths_in_dag(&graph, vec!["shiny gold".to_string()], vec![]);
    let mut visited: HashSet<Vec<String>> = HashSet::new();

    for path in paths {
        let mut sub_amount = 0;
        let mut multiplier = 1;
        let mut sub_path: Vec<String> = vec![path[0].clone()];
        for i in 0..path.len() - 1 {
            sub_path.push(path[i + 1].to_string());
            let amount = amounts.get(&(&path[i], &path[i + 1])).unwrap();
            if !visited.contains(&(sub_path)) {
                visited.insert(sub_path.clone());
                sub_amount += amount * multiplier;
            }
            multiplier *= amount;
        }
        result += sub_amount;
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
        assert_eq!(solve1("example.txt"), 4);
        assert_eq!(solve2("example.txt"), 32);
        assert_eq!(solve2("example2.txt"), 126);
    }

    #[test]
    fn test_parse_line() {
        let (bag, targets) = parse_line("mirrored coral bags contain 4 shiny salmon bags, 1 light orange bag, 4 faded yellow bags, 5 shiny maroon bags.");
        assert_eq!(bag, "mirrored coral");
        assert_eq!(targets.len(), 4);

        assert_eq!(targets[0].amount, 4);
        assert_eq!(targets[0].color, "shiny salmon");
        assert_eq!(targets[1].amount, 1);
        assert_eq!(targets[1].color, "light orange");
        assert_eq!(targets[2].amount, 4);
        assert_eq!(targets[2].color, "faded yellow");
        assert_eq!(targets[3].amount, 5);
        assert_eq!(targets[3].color, "shiny maroon");
    }

    #[test]
    fn test_parse_line_leaf() {
        let (bag, targets) = parse_line("dotted black bags contain no other bags.");
        assert_eq!(bag, "dotted black");
        assert_eq!(targets.len(), 0);
    }
}
