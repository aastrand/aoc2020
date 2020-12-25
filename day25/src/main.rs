fn do_loop(subject: u64, modu: u64) -> u64 {
    (subject * subject) % modu
}

fn solve1(card_pubkey: u64, door_pubkey: u64) -> u64 {
    let mut subject = 7;
    let mut card_loop = 0;
    while subject != card_pubkey {
        subject = do_loop(subject, 20201227);
        card_loop += 1;
    }

    let mut key = door_pubkey;
    for _ in 0..card_loop {
        key = do_loop(key, 20201227);
    }

    key
}

fn main() {
    println!("{}", solve1(6930903, 19716708));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1(5764801, 17807724), 14897079);
    }
}
