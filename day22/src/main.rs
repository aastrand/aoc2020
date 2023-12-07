use std::collections::HashSet;
use std::fs;

fn parse_deck(input: &str) -> Vec<u64> {
    let deck = input.split("\n").collect::<Vec<&str>>();
    deck[1..]
        .iter()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn deck_to_str(deck: &Vec<u64>) -> String {
    deck.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn play_round(p1_deck: &mut Vec<u64>, p2_deck: &mut Vec<u64>) {
    println!("Player 1's deck: {}", deck_to_str(p1_deck));
    println!("Player 2's deck: {}", deck_to_str(p2_deck));

    let p1_card = p1_deck.remove(0);
    let p2_card = p2_deck.remove(0);

    println!("Player 1 plays: {}", p1_card);
    println!("Player 2 plays: {}", p2_card);

    if p1_card > p2_card {
        println!("Player 1 wins the round!");
        p1_deck.push(p1_card);
        p1_deck.push(p2_card);
    } else if p2_card > p1_card {
        println!("Player 2 wins the round!");
        p2_deck.push(p2_card);
        p2_deck.push(p1_card);
    } else {
        panic!("This can't happen");
    }
}

fn score_deck(deck: &Vec<u64>) -> u64 {
    let mut multiplier = deck.len() as u64;
    let mut score = 0;

    for c in deck {
        score += multiplier * c;
        multiplier -= 1;
    }

    score
}

fn solve1(filename: &str) -> u64 {
    let player_input = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut p1_deck = parse_deck(&player_input[0]);
    let mut p2_deck = parse_deck(&player_input[1]);

    let mut round = 1;
    while p1_deck.len() > 0 && p2_deck.len() > 0 {
        println!("-- Round {} --", round);
        play_round(&mut p1_deck, &mut p2_deck);
        println!("");
        round += 1;
    }

    println!("\n== Post-game results ==");
    println!("Player 1's deck: {}", deck_to_str(&p1_deck));
    println!("Player 2's deck: {}", deck_to_str(&p2_deck));
    println!("");

    if p1_deck.len() > 0 {
        score_deck(&p1_deck)
    } else {
        score_deck(&p2_deck)
    }
}

fn copy_deck(deck: &Vec<u64>, amount: u64) -> Vec<u64> {
    let mut copy = vec![];
    for i in 0..amount as usize {
        copy.push(deck[i]);
    }
    copy
}

fn play_recursive_game(num: u64, p1_deck: &mut Vec<u64>, p2_deck: &mut Vec<u64>) -> u64 {
    let mut p1_prev_decks: HashSet<String> = HashSet::new();
    let mut p2_prev_decks: HashSet<String> = HashSet::new();
    let mut round = 1;

    println!("=== Game {} ===\n", num);

    while p1_deck.len() > 0 && p2_deck.len() > 0 {
        println!("-- Round {} (Game {}) --", round, num);

        let p1_deck_str = deck_to_str(p1_deck);
        let p2_deck_str = deck_to_str(p2_deck);

        println!("Player 1's deck: {}", p1_deck_str);
        println!("Player 2's deck: {}", p2_deck_str);

        if p1_prev_decks.contains(&p1_deck_str) || p2_prev_decks.contains(&p2_deck_str) {
            println!("Player 1 wins round {} of game {}!", round, num);
            return 1;
        }

        p1_prev_decks.insert(p1_deck_str);
        p2_prev_decks.insert(p2_deck_str);

        /*
        Before either player deals a card,
        if there was a previous round in this game that had exactly the same cards
        in the same order in the same players' decks, the game instantly ends in a win for player 1.
        Previous rounds from other games are not considered.
        (This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
        */

        let p1_card = p1_deck.remove(0);
        let p2_card = p2_deck.remove(0);

        println!("Player 1 plays: {}", p1_card);
        println!("Player 2 plays: {}", p2_card);

        /*
        If both players have at least as many cards remaining in their deck as the value of the card they just drew,
        the winner of the round is determined by playing a new game of Recursive Combat (see below).
        */
        let winner = if p1_card <= p1_deck.len() as u64 && p2_card <= p2_deck.len() as u64 {
            let game = unsafe {
                GAME_COUNTER += 1;
                GAME_COUNTER
            };
            let mut p1_copy = copy_deck(p1_deck, p1_card);
            let mut p2_copy = copy_deck(p2_deck, p2_card);

            println!("Playing a sub-game to determine the winner...\n");
            let winner = play_recursive_game(game, &mut p1_copy, &mut p2_copy);
            println!("... anyway, back to game {}", num);

            winner
        } else {
            if p1_card > p2_card {
                1
            } else {
                2
            }
        };

        if winner == 1 {
            println!("Player 1 wins round {} of game {}!", round, num);
            p1_deck.push(p1_card);
            p1_deck.push(p2_card);
        } else if winner == 2 {
            println!("Player 2 wins round {} of game {}!", round, num);
            p2_deck.push(p2_card);
            p2_deck.push(p1_card);
        } else {
            panic!("This can't happen");
        }

        println!("");
        round += 1;
    }

    if p1_deck.len() > 0 {
        1
    } else {
        2
    }
}

static mut GAME_COUNTER: u64 = 1;

fn solve2(filename: &str) -> u64 {
    let player_input = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut p1_deck = parse_deck(&player_input[0]);
    let mut p2_deck = parse_deck(&player_input[1]);

    play_recursive_game(unsafe { GAME_COUNTER }, &mut p1_deck, &mut p2_deck);

    println!("\n== Post-game results ==");
    println!("Player 1's deck: {}", deck_to_str(&p1_deck));
    println!("Player 2's deck: {}", deck_to_str(&p2_deck));

    if p1_deck.len() > 0 {
        score_deck(&p1_deck)
    } else {
        score_deck(&p2_deck)
    }
}

fn main() {
    println!("{}", solve1("../input/2020/day22.txt"));
    println!("{}", solve2("../input/2020/day22.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 306);
        assert_eq!(solve2("example.txt"), 291);
    }
}
