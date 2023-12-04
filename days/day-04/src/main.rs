use std::collections::{HashSet, VecDeque};

fn main() {
    part1("advent-of-code-inputs/2023/day-04/example");
    part1("advent-of-code-inputs/2023/day-04/input");
    part2("advent-of-code-inputs/2023/day-04/example");
    part2("advent-of-code-inputs/2023/day-04/input");
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let scrachcards = parse_scratchcards(lines);

    let sum: u32 = scrachcards
        .iter()
        .fold(0, |acc, card| acc + check_card_value(card));

    println!("Score: {}", sum)
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);
    let scrachcards = parse_scratchcards(lines);

    let mut card_count = 0;
    let mut copies_increment = VecDeque::new();

    let mut current_card_count = 1;
    for card in scrachcards {
        println!("card: {} has {} copies", card.id, current_card_count);

        card_count += current_card_count;

        let number_of_matches = count_card_matching(&card);
        for i in 0..number_of_matches {
            if i as usize >= copies_increment.len() {
                copies_increment.push_back(current_card_count);
            } else {
                *copies_increment.get_mut(i as usize).unwrap() += current_card_count;
            }
        }

        println!("{:?}", copies_increment);

        let next_copies = copies_increment.pop_front().unwrap_or(0);

        current_card_count = next_copies + 1;
    }

    println!("Card count: {}", card_count)
}

#[derive(Debug)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    scratched: HashSet<i32>,
}

fn parse_scratchcards(lines: Vec<String>) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in lines {
        let card_line: Vec<&str> = line.split(": ").collect();

        // get what's after the space
        let id = card_line[0]
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let values: Vec<&str> = card_line[1].split(" | ").collect();

        let mut winning = HashSet::new();
        for val in values[0].split(' ') {
            let val = match val.parse::<i32>() {
                Ok(val) => val,
                Err(_) => continue,
            };
            winning.insert(val);
        }

        let mut scratched = HashSet::new();
        for val in values[1].split(' ') {
            let val = match val.parse::<i32>() {
                Ok(val) => val,
                Err(_) => continue,
            };
            scratched.insert(val);
        }

        cards.push(Card {
            id,
            winning,
            scratched,
        })
    }

    cards
}

fn check_card_value(card: &Card) -> u32 {
    let mut win_count = 0;

    for win in card.winning.iter() {
        if card.scratched.contains(win) {
            win_count += 1;
        }
    }

    if win_count == 0 {
        0
    } else {
        1 << (win_count - 1)
    }
}

fn count_card_matching(card: &Card) -> u32 {
    let mut matching = 0;

    for win in card.winning.iter() {
        if card.scratched.contains(win) {
            matching += 1;
        }
    }

    matching
}
