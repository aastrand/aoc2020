use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Operation {
    instr: String,
    operand: i64,
}

struct Runner<'a> {
    code: &'a Vec<Operation>,
    acc: i64,
    pc: usize,
}

impl<'a> Runner<'a> {
    fn run(&mut self) -> bool {
        let mut visited_addresses: Vec<bool> = vec![false; self.code.len()];

        loop {
            if self.pc >= self.code.len() {
                //println!("Terminating normally");
                return true;
            }
            let op = &self.code[self.pc];

            if visited_addresses[self.pc] {
                //println!("Infinite loop detected");
                return false;
            }
            visited_addresses[self.pc] = true;

            //print!("{}: {:?}", self.pc, op);

            match op.instr.as_ref() {
                "acc" => {
                    self.acc += op.operand;
                    self.pc += 1;
                }
                "jmp" => self.pc = (self.pc as i64 + op.operand) as usize,
                "nop" => self.pc += 1,
                _ => panic!("Unknown op: {:?}", op),
            }

            //println!(" -> acc: {}, pc: {}", self.acc, self.pc);
        }
    }
}

/*
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
*/
fn parse_opcodes(lines: Vec<String>) -> Vec<Operation> {
    lines
        .iter()
        .map(|line| Operation {
            instr: line[0..3].to_string(),
            operand: line[4..].parse::<i64>().unwrap(),
        })
        .collect()
}

fn solve1(filename: &str) -> i64 {
    let mut runner = Runner {
        code: &parse_opcodes(lines_from_file(filename)),
        acc: 0,
        pc: 0,
    };

    runner.run();
    runner.acc
}

fn brute_switch_instr(from: &str, to: &str, operations: &mut Vec<Operation>) -> Option<i64> {
    let mut idx = 0;

    while idx <= operations.len() {
        while idx <= operations.len() {
            let op = &operations[idx];
            if op.instr == from {
                operations[idx] = Operation {
                    instr: to.to_string(),
                    operand: op.operand,
                };
                break;
            }
            idx += 1;
        }

        let mut runner = Runner {
            code: &operations,
            acc: 0,
            pc: 0,
        };

        if runner.run() {
            return Some(runner.acc);
        } else {
            operations[idx] = Operation {
                instr: from.to_string(),
                operand: operations[idx].operand,
            };
        }
        idx += 1;
    }

    None
}

fn solve2(filename: &str) -> i64 {
    let mut operations = parse_opcodes(lines_from_file(filename));

    if let Some(acc) = brute_switch_instr("jmp", "nop", &mut operations) {
        return acc;
    }
    if let Some(acc) = brute_switch_instr("nop", "jmp", &mut operations) {
        return acc;
    }

    panic!("Could not find solution");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("{}", solve1("../input/2020/day8.txt"));
    println!("{}", solve2("../input/2020/day8.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 5);
        assert_eq!(solve1("example2.txt"), 8);
        assert_eq!(solve2("example.txt"), 8);
    }
}
