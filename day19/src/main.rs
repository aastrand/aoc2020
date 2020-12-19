use std::fs;

use std::collections::{HashMap, HashSet};

extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    name: u64,
    regex: String,
    l1: u64,
    l2: u64,
    r1: u64,
    r2: u64,
}

impl Node {
    fn new(name: u64) -> Node {
        Node {
            name: name,
            regex: String::new(),
            l1: 0,
            l2: 0,
            r1: 0,
            r2: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    graph: HashMap<u64, Node>,
    used_by: HashMap<u64, HashSet<u64>>,
    complete: HashSet<u64>,
    zero: String,
}

fn parse_side(input: &str) -> (u64, Option<u64>) {
    let split: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
    let s2 = if let Some(part2) = split.get(1) {
        Some(part2.parse::<u64>().unwrap())
    } else {
        None
    };
    return (split[0].parse::<u64>().unwrap(), s2);
}

impl Graph {
    fn new() -> Graph {
        Graph {
            graph: HashMap::new(),
            used_by: HashMap::new(),
            complete: HashSet::new(),
            zero: String::new(),
        }
    }

    fn update_used_by(&mut self, source: u64, target: u64) {
        if !self.used_by.contains_key(&target) {
            self.used_by.insert(target, HashSet::new());
        }

        self.used_by.get_mut(&target).unwrap().insert(source);
    }

    fn set_complete(&mut self, node: u64) {
        self.complete.insert(node);
    }

    fn set_regex(&mut self, node: u64, regex: &str) {
        self.graph.get_mut(&node).unwrap().regex = regex.to_string();
    }

    fn is_ready(&self, node: u64) -> bool {
        let mut ready = true;
        let node = self.graph.get(&node).unwrap();
        if node.l1 > 0 {
            ready &= self.complete.contains(&node.l1);
        }
        if node.l2 > 0 {
            ready &= self.complete.contains(&node.l2);
        }
        if node.r1 > 0 {
            ready &= self.complete.contains(&node.r1);
        }
        if node.r2 > 0 {
            ready &= self.complete.contains(&node.r2);
        }
        ready
    }

    fn parse_graph(input: &str) -> Graph {
        let mut graph = Graph::new();

        for rule in input.split("\n") {
            let split: Vec<&str> = rule.split(": ").collect();
            let name = split[0].parse::<u64>().unwrap();
            let mut node = Node::new(name);

            if name == 0 {
                graph.zero = split[1].to_string();
                continue;
            }

            match split[1] {
                "\"a\"" => {
                    node.regex = " a ".to_string();
                    graph.complete.insert(name);
                }
                "\"b\"" => {
                    node.regex = " b ".to_string();
                    graph.complete.insert(name);
                }
                _ => {
                    let sides: Vec<&str> = split[1].split(" | ").collect();
                    node.regex = format!(" {} ", split[1]).to_string();
                    let (l1, l2) = parse_side(sides[0]);
                    node.l1 = l1;
                    graph.update_used_by(name, node.l1);

                    if let Some(value) = l2 {
                        graph.update_used_by(name, value);
                        node.l2 = value;
                    }

                    if let Some(right) = sides.get(1) {
                        let (r1, r2) = parse_side(right);
                        node.r1 = r1;
                        graph.update_used_by(name, r1);

                        if let Some(value) = r2 {
                            graph.update_used_by(name, value);
                            node.r2 = value;
                        }
                    }
                }
            }
            graph.graph.insert(name, node);
        }

        graph
    }
}

fn propagate(graph: &mut Graph) {
    let mut to_propagate = HashSet::new();
    for n in &graph.complete {
        if let Some(used_by) = graph.used_by.get(&n) {
            for u in used_by {
                if graph.is_ready(*u) {
                    to_propagate.insert(*u);
                }
            }
        }
    }

    while to_propagate.len() > 0 {
        //println!("{:?}", to_propagate);
        for n in &to_propagate {
            //println!("{}", n);
            let node = graph.graph.get(&n).unwrap();
            let mut regex = node.regex.clone();
            if node.l1 > 0 && node.l2 > 0 {
                let l1 = graph.graph.get(&node.l1).unwrap();
                let l2 = graph.graph.get(&node.l2).unwrap();
                regex = str::replace(
                    &regex,
                    &format!(" {} ", &l1.name.to_string()),
                    &format!(" ({}) ", l1.regex),
                );
                regex = str::replace(
                    &regex,
                    &format!(" {} ", &l2.name.to_string()),
                    &format!(" ({}) ", l2.regex),
                );
            } else if node.l1 > 0 {
                let l1 = graph.graph.get(&node.l1).unwrap();
                regex = str::replace(
                    &regex,
                    &format!(" {} ", &l1.name.to_string()),
                    &format!(" ({}) ", l1.regex),
                );
            }
            if node.r1 > 0 && node.r2 > 0 {
                let r1 = graph.graph.get(&node.r1).unwrap();
                let r2 = graph.graph.get(&node.r2).unwrap();
                regex = str::replace(
                    &regex,
                    &format!(" {} ", &r1.name.to_string()),
                    &format!(" ({}) ", r1.regex),
                );
                regex = str::replace(
                    &regex,
                    &format!(" {} ", &r2.name.to_string()),
                    &format!(" ({}) ", r2.regex),
                );
            } else if node.r1 > 0 {
                let r1 = graph.graph.get(&node.r1).unwrap();
                regex = str::replace(
                    &regex,
                    &format!(" {} ", &r1.name.to_string()),
                    &format!(" ({}) ", r1.regex),
                );
            }
            //println!("{} {}", n, regex);
            graph.set_regex(*n, &regex);
            graph.set_complete(*n);
        }
        let mut new_to_propagate = HashSet::new();
        for n in &to_propagate {
            if let Some(used_by) = graph.used_by.get(&n) {
                for u in used_by {
                    if graph.is_ready(*u) {
                        new_to_propagate.insert(*u);
                    }
                }
            }
        }
        to_propagate = new_to_propagate.clone();
    }
}

fn solve1(filename: &str) -> u64 {
    let input: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let mut graph = Graph::parse_graph(&input[0]);
    propagate(&mut graph);

    let nodes: Vec<u64> = graph
        .zero
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut final_re = String::new();
    for node in nodes {
        final_re.push_str(&format!("({})", graph.graph.get(&node).unwrap().regex));
    }

    let re = Regex::new(&format!("^{}$", &final_re.replace(" ", ""))).unwrap();

    input[1]
        .split("\n")
        .filter(|l| re.is_match(l))
        .collect::<Vec<&str>>()
        .len() as u64
}

fn propagate2(graph: &mut Graph) {
    let mut to_propagate = HashSet::new();
    for n in &graph.complete {
        if let Some(used_by) = graph.used_by.get(&n) {
            for u in used_by {
                if graph.is_ready(*u) {
                    to_propagate.insert(*u);
                }
            }
        }
    }

    while to_propagate.len() > 0 {
        for n in &to_propagate {
            let node = graph.graph.get(&n).unwrap();
            let mut regex = node.regex.clone();
            match n {
                8 => {
                    let n42 = graph.graph.get(&42).unwrap();
                    regex = str::replace(
                        &regex,
                        " 42 ",
                        " 42 | 42 (42 | 42 (42 | 42 (42 | 42 (42 | 42 (42))))) ",
                    );
                    regex = str::replace(&regex, "42", &format!(" ({}) ", n42.regex));
                }
                11 => {
                    let n42 = graph.graph.get(&42).unwrap();
                    let n31 = graph.graph.get(&31).unwrap();
                    regex = str::replace(
                        &regex,
                        " 42 31 ",
                        " 42 31 | 42 (42 31 | 42 (42 31 | 42 (42 31 | 42 (42 31 | 42 (42 31 | 42 (42 31 | 42 (42 31 ) 31) 31) 31) 31) 31) 31) 31 "
                    );
                    regex = str::replace(&regex, "42", &format!(" ({}) ", n42.regex));
                    regex = str::replace(&regex, "31", &format!(" ({}) ", n31.regex));
                }
                _ => {
                    if node.l1 > 0 && node.l2 > 0 {
                        let l1 = graph.graph.get(&node.l1).unwrap();
                        let l2 = graph.graph.get(&node.l2).unwrap();
                        regex = str::replace(
                            &regex,
                            &format!(" {} ", &l1.name.to_string()),
                            &format!(" ({}) ", l1.regex),
                        );
                        regex = str::replace(
                            &regex,
                            &format!(" {} ", &l2.name.to_string()),
                            &format!(" ({}) ", l2.regex),
                        );
                    } else if node.l1 > 0 {
                        let l1 = graph.graph.get(&node.l1).unwrap();
                        regex = str::replace(
                            &regex,
                            &format!(" {} ", &l1.name.to_string()),
                            &format!(" ({}) ", l1.regex),
                        );
                    }
                    if node.r1 > 0 && node.r2 > 0 {
                        let r1 = graph.graph.get(&node.r1).unwrap();
                        let r2 = graph.graph.get(&node.r2).unwrap();
                        regex = str::replace(
                            &regex,
                            &format!(" {} ", &r1.name.to_string()),
                            &format!(" ({}) ", r1.regex),
                        );
                        regex = str::replace(
                            &regex,
                            &format!(" {} ", &r2.name.to_string()),
                            &format!(" ({}) ", r2.regex),
                        );
                    } else if node.r1 > 0 {
                        let r1 = graph.graph.get(&node.r1).unwrap();
                        regex = str::replace(
                            &regex,
                            &format!(" {} ", &r1.name.to_string()),
                            &format!(" ({}) ", r1.regex),
                        );
                    }
                }
            }
            graph.set_regex(*n, &regex);
            graph.set_complete(*n);
        }
        let mut new_to_propagate = HashSet::new();
        for n in &to_propagate {
            if let Some(used_by) = graph.used_by.get(&n) {
                for u in used_by {
                    if graph.is_ready(*u) {
                        new_to_propagate.insert(*u);
                    }
                }
            }
        }
        to_propagate = new_to_propagate.clone();
    }
}

fn solve2(filename: &str) -> u64 {
    let input: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let mut graph = Graph::parse_graph(&input[0]);
    propagate2(&mut graph);

    let nodes: Vec<u64> = graph
        .zero
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut final_re = String::new();
    for node in nodes {
        final_re.push_str(&format!("({})", graph.graph.get(&node).unwrap().regex));
    }

    let re = Regex::new(&format!("^{}$", &final_re.replace(" ", ""))).unwrap();

    input[1]
        .split("\n")
        .filter(|l| re.is_match(l))
        .collect::<Vec<&str>>()
        .len() as u64
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
        assert_eq!(solve2("example2.txt"), 12);
    }

    #[test]
    fn test_parse_side() {
        let (s1, s2) = parse_side("11 23");
        assert_eq!(s1, 11);
        assert_eq!(s2, Some(23));

        let (s1, s2) = parse_side("11");
        assert_eq!(s1, 11);
        assert_eq!(s2, None);
    }
}
