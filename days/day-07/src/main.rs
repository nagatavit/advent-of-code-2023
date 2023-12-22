use std::collections::HashMap;

fn main() {
    part1("advent-of-code-inputs/2023/day-07/example");
    part1("advent-of-code-inputs/2023/day-07/input");
    part2("advent-of-code-inputs/2023/day-07/example");
    part2("advent-of-code-inputs/2023/day-07/input");
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type.lt(&other.hand_type) {
            std::cmp::Ordering::Less
        } else if self.hand_type.gt(&other.hand_type) {
            std::cmp::Ordering::Greater
        } else {
            let mut ordering = std::cmp::Ordering::Equal;

            for (i, card) in self.cards.iter().enumerate() {
                ordering = match card.cmp(&other.cards[i]) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => continue,
                };
                break;
            }

            ordering
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let mut hands = parse_input(lines);

    hands.sort_by(|a, b| b.cmp(a));

    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (i as u32 + 1) * hand.bid;
    }

    println!("total_winnings: {total_winnings}");
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);
    let mut hands = parse_input(lines);

    for hand in &mut hands {
        convert_hand_to_jokers(hand);
    }

    hands.sort_by(|a, b| b.cmp(a));

    println!("{:#?}", hands);

    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (i as u32 + 1) * hand.bid;
    }

    println!("total_winnings: {total_winnings}");
}

fn parse_input(lines: Vec<String>) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in lines {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        let hand_str = splitted_line[0];
        let bid = splitted_line[1];

        hands.push(Hand {
            bid: bid.parse().unwrap(),
            ..parse_hand(hand_str)
        });
    }

    hands
}

fn parse_hand(hand_str: &str) -> Hand {
    let mut cards = Vec::new();

    // helper to find which kind of hand this is
    let mut card_counter = HashMap::new();

    for card_str in hand_str.chars() {
        use Card::*;
        let card = match card_str {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => panic!("{card_str}"),
        };

        card_counter
            .entry(card.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        cards.push(card);
    }

    let hand_type = find_hand_type(card_counter);

    Hand {
        cards,
        hand_type,
        bid: 0,
    }
}

fn find_hand_type(card_counter: HashMap<Card, u8>) -> HandType {
    if card_counter.len() == 1 {
        HandType::FiveOfAKind
    } else if card_counter.len() == 2 {
        match card_counter.iter().find(|(_, &counter)| counter == 4) {
            Some(_) => return HandType::FourOfAKind,
            None => return HandType::FullHouse,
        };
    } else if card_counter.iter().any(|(_, &counter)| counter == 3) {
        return HandType::ThreeOfAKind;
    } else {
        match card_counter
            .iter()
            .filter(|(_, &counter)| counter == 2)
            .count()
        {
            2 => return HandType::TwoPair,
            1 => return HandType::OnePair,
            _ => return HandType::HighCard,
        };
    }
}

fn convert_hand_to_jokers(hand: &mut Hand) {
    let mut card_counter = HashMap::new();

    for card in hand.cards.iter_mut() {
        if let Card::J = card {
            *card = Card::Joker
        }

        card_counter
            .entry(card.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    if can_turn_into_five_of_a_kind(&card_counter) {
        hand.hand_type = HandType::FiveOfAKind
    } else if can_turn_into_four_of_a_kind(&card_counter) {
        hand.hand_type = HandType::FourOfAKind
    } else if can_turn_into_full_house(&card_counter) {
        hand.hand_type = HandType::FullHouse
    } else if can_turn_into_three_of_a_kind(&card_counter) {
        hand.hand_type = HandType::ThreeOfAKind
    } else if can_turn_into_two_pair(&card_counter) {
        hand.hand_type = HandType::TwoPair
    } else if can_turn_into_one_pair(&card_counter) {
        hand.hand_type = HandType::OnePair
    }
}

fn can_turn_into_five_of_a_kind(card_counter: &HashMap<Card, u8>) -> bool {
    if card_counter.len() == 1 {
        // already a five of a kind
        return true;
    }

    if card_counter.len() == 2 && card_counter.contains_key(&Card::Joker) {
        return true;
    }

    false
}

fn can_turn_into_four_of_a_kind(card_counter: &HashMap<Card, u8>) -> bool {
    if card_counter.len() == 2 && card_counter.iter().any(|(_, &counter)| counter == 4) {
        // already a four of a kind
        return true;
    }

    let mut card_counter = card_counter.clone();

    let joker_count = card_counter.remove(&Card::Joker).unwrap_or(0);

    card_counter
        .iter()
        .filter(|(_, &counter)| counter + joker_count == 4)
        .count()
        > 0
}

fn can_turn_into_full_house(card_counter: &HashMap<Card, u8>) -> bool {
    if card_counter.len() == 2 && card_counter.iter().any(|(_, &counter)| counter == 3) {
        // already a full house
        return true;
    }

    let mut card_counter = card_counter.clone();

    let joker_count = card_counter.remove(&Card::Joker).unwrap_or(0);

    if card_counter.len() == 2
        && card_counter
            .iter()
            .filter(|(_, &counter)| counter + joker_count == 3)
            .count()
            > 0
    {
        return true;
    }

    false
}

fn can_turn_into_three_of_a_kind(card_counter: &HashMap<Card, u8>) -> bool {
    if card_counter.len() == 3 && card_counter.iter().any(|(_, &counter)| counter == 3) {
        // already a three of a kind
        return true;
    }

    let mut card_counter = card_counter.clone();

    let joker_count = card_counter.remove(&Card::Joker).unwrap_or(0);

    if card_counter.len() == 3
        && card_counter
            .iter()
            .filter(|(_, &counter)| counter + joker_count == 3)
            .count()
            > 0
    {
        return true;
    }

    false
}

fn can_turn_into_two_pair(card_counter: &HashMap<Card, u8>) -> bool {
    if card_counter
        .iter()
        .filter(|(_, &counter)| counter == 2)
        .count()
        == 2
    {
        // already a two pair
        return true;
    }

    let mut card_counter = card_counter.clone();

    let joker_count = card_counter.remove(&Card::Joker).unwrap_or(0);

    for (_, count) in card_counter.iter_mut() {
        if *count + joker_count == 2 {
            *count = 2;
            break;
        }
    }

    card_counter
        .iter()
        .filter(|(_, &counter)| counter == 2)
        .count()
        == 2
}

fn can_turn_into_one_pair(card_counter: &HashMap<Card, u8>) -> bool {
    if card_counter
        .iter()
        .filter(|(_, &counter)| counter == 2)
        .count()
        == 1
    {
        // already a one pair
        return true;
    }

    let mut card_counter = card_counter.clone();

    let joker_count = card_counter.remove(&Card::Joker).unwrap_or(0);

    for (_, count) in card_counter.iter_mut() {
        if *count + joker_count == 2 {
            *count = 2;
            break;
        }
    }

    card_counter
        .iter()
        .filter(|(_, &counter)| counter == 2)
        .count()
        == 1
}
