#[derive(Debug)]
struct Cups {
    cur: usize,
    max: u64,
    lookup: Vec<Cup>,
}

#[derive(Debug, Clone, Copy)]
struct Cup {
    label: u64,
    next: u64,
}

impl Cup {
    fn new(label: u64) -> Cup {
        Cup {
            label: label,
            next: 0,
        }
    }
}

impl Cups {
    fn new(input: &str, total: usize) -> Cups {
        let mut lookup: Vec<Cup> = vec![Cup::new(0); total + 1];
        let mut nums = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        for i in 10..total + 1 {
            nums.push(i as u64);
        }
        for n in &nums {
            lookup[*n as usize] = Cup::new(*n);
        }
        for i in 0..nums.len() - 1 {
            lookup[nums[i] as usize].next = lookup[nums[i + 1] as usize].label;
        }
        lookup[nums[total - 1] as usize].next = lookup[nums[0] as usize].label;
        Cups {
            cur: lookup[nums[0] as usize].label as usize,
            max: total as u64,
            lookup: lookup,
        }
    }

    fn get_next(&self, label: u64) -> u64 {
        self.lookup[label as usize].next
    }

    fn update_next(&mut self, label: u64, next: u64) {
        self.lookup[label as usize].next = next;
    }

    fn make_move(&mut self) {
        let cur_value = self.lookup[self.cur];

        // The crab picks up the three cups that are immediately clockwise of the current cup.
        // They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let cup1 = self.lookup[cur_value.next as usize];
        let cup2 = self.lookup[cup1.next as usize];
        let cup3 = self.lookup[cup2.next as usize];

        // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
        // If this would select one of the cups that was just picked up,
        // the crab will keep subtracting one until it finds a cup that wasn't just picked up.
        // If at any point in this process the value goes below the lowest value on any cup's label,
        // it wraps around to the highest value on any cup's label instead.
        let mut destination_value = cur_value.label;
        for _ in 0..4 {
            destination_value = destination_value - 1;
            if destination_value < 1 {
                destination_value = self.max;
            }
            if cup1.label != destination_value
                && cup2.label != destination_value
                && cup3.label != destination_value
            {
                break;
            }
        }

        // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
        // They keep the same order as when they were picked up.
        // n1 cup1 cup2 cup3 n2 => n1.next = cup3.next
        // d1 cup1 cup2 cup3 d2 => d1.next = cup1, cup3.next = d2
        self.update_next(cur_value.label, cup3.next);
        let d1 = self.lookup[destination_value as usize].label;
        let d2 = self.get_next(destination_value);
        self.update_next(d1, cup1.label);
        self.update_next(cup3.label, d2);

        // The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
        self.cur = self.lookup[cur_value.label as usize].next as usize;
    }

    #[allow(dead_code)]
    fn cups(&self, start: usize, len: usize) -> Vec<u64> {
        let mut nums = vec![];
        let mut cur = self.lookup[start];

        for _ in 0..len {
            nums.push(cur.label);
            cur = self.lookup[cur.next as usize];
        }

        nums
    }

    fn order(&self) -> String {
        let mut cur = self.lookup[self.lookup[1].next as usize];
        let mut order = String::new();
        while cur.label != 1 {
            order.push_str(&format!("{}", cur.label));
            cur = self.lookup[cur.next as usize];
        }

        order
    }
}

fn solve1(input: &str, moves: u64) -> String {
    let mut cups = Cups::new(input, 9);
    for _m in 0..moves {
        //println!("-- move {} --", _m + 1);
        cups.make_move();
        //println!("");
    }
    cups.order()
}

fn solve2(input: &str, moves: u64) -> u64 {
    let mut cups = Cups::new(input, 1_000_000);
    for _ in 0..moves {
        cups.make_move();
    }

    let one = cups.lookup[1];
    let n1 = cups.lookup[one.next as usize];
    let n2 = cups.lookup[n1.next as usize];

    n1.label * n2.label
}

fn main() {
    println!("{}", solve1("215694783", 100));
    println!("{}", solve2("215694783", 10_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("389125467", 10), "92658374");
        assert_eq!(solve1("389125467", 100), "67384529");
        assert_eq!(solve2("389125467", 10_000_000), 149245887792);
    }

    #[test]
    fn test_make_move() {
        /*
        -- move 1 --
        cups: (3) 8  9  1  2  5  4  6  7
        pick up: 8, 9, 1
        destination: 2

        -- move 2 --
        cups:  3 (2) 8  9  1  5  4  6  7
        pick up: 8, 9, 1
        destination: 7
        */
        let mut cups = Cups::new("389125467", 9);
        assert_eq!(cups.cur, 3);
        assert_eq!(cups.cups(3, 9), vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
        cups.make_move();
        assert_eq!(cups.cur, 2);
        assert_eq!(cups.cups(3, 9), vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }

    #[test]
    fn test_order() {
        let cups = Cups::new("583741926", 9);
        assert_eq!(cups.order(), "92658374");
    }
}
