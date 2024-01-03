use std::{fs, collections::VecDeque};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum HandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
    hand_type: HandType
}

impl Hand {
    fn new(cards: &str, bid: &str) -> Hand {
        let cards: Vec<char> = cards.chars().collect();
        let hand_type = Hand::get_hand_type(&cards);
        Hand {
            cards,
            bid: bid.parse().expect("Error parsing the BID"),
            hand_type
        }
    }

    fn get_hand_type(cards: &Vec<char>) -> HandType {
        if cards.windows(2).all(|w| w[0] == w[1]) {
            return HandType::FiveKind;
        }

        let mut ordered_cards: VecDeque<char> = VecDeque::new();
        let mut greater_card_counter = 0;
        for card in cards {
            let card_counter = cards.iter().filter(|filter_card| *filter_card == card).count();
            if ordered_cards.contains(card) == true {
                continue;
            }

            for _ in 0..card_counter {
                if card_counter >= greater_card_counter {
                    ordered_cards.push_front(card.clone());
                    continue;
                }

                ordered_cards.push_back(card.clone());
            }

            if card_counter > greater_card_counter {
                greater_card_counter = card_counter;
            }
        }

        let jokers = vec!['A', 'B', 'C', 'D', 'E'];
        let mut joker_idx_use = 0;
        let mut previous_card: Option<char> = None;
        for ordered_idx in 0..ordered_cards.len() {
            let card = ordered_cards[ordered_idx];
            if previous_card.is_none() {
                ordered_cards[ordered_idx] = jokers[joker_idx_use];
                previous_card = Some(card);
                continue;
            }

            if card != previous_card.unwrap() {
                joker_idx_use += 1;
            }

            ordered_cards[ordered_idx] = jokers[joker_idx_use];
            previous_card = Some(card);
        }

        match ordered_cards.iter().collect::<String>().as_str() {
            "AAAAB" => return HandType::FourKind,
            "AAABB" => return HandType::FullHouse,
            "AAABC" => return HandType::ThreeKind,
            "AABBC" => return HandType::TwoPair,
            "AABCD" => return HandType::OnePair,
            _ => return HandType::HighCard,
        }
    }
}

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Unable to open the input file.");

    let mut hands: Vec<Hand> = vec![];
    for line in content.lines() {
        let (cards, mut bid): (&str, &str) = line.split_at(line.find(" ").unwrap());
        bid = bid.trim();
        hands.push(Hand::new(cards, bid));
    }

    let iteration_max = hands.len();
    for max in (0..iteration_max).rev() {
        for hand_idx in 0..max {
            let hand = &hands[hand_idx];
            let next_hand = hands.get(hand_idx + 1);

            if next_hand.is_none() {
                continue;
            }

            if hand.hand_type == next_hand.unwrap().hand_type {
                for card_idx in 0..hand.cards.len() {
                    let hand_card_power = get_card_power(&hand.cards[card_idx]);
                    let next_hand_card_power = get_card_power(&next_hand.unwrap().cards[card_idx]);

                    if hand_card_power == next_hand_card_power {
                        continue;
                    }

                    if hand_card_power < next_hand_card_power {
                        break;
                    }

                    if hand_card_power > next_hand_card_power {
                        // println!("ON {increment} SWAP {} - {}", &hand.cards[card_idx], &next_hand.unwrap().cards[card_idx]);
                        // println!("ON {increment} SWAP {:?} - {:?} | CARD IDX {card_idx}", &hand, &next_hand.unwrap());
                        hands.swap(hand_idx, hand_idx + 1);
                        break;
                    }
                }

                continue;
            }

            if hand.hand_type > next_hand.unwrap().hand_type {
                hands.swap(hand_idx, hand_idx + 1);
                continue;
            }
        }
    }

    let mut accum: usize = 0;
    for (idx, hand) in hands.iter().enumerate() {
        // println!("Calc: {} * {}", hand.bid, idx + 1);
        accum += hand.bid as usize * (idx + 1);
    }

    println!("Part 1 result: {accum}");
}

fn get_card_power(card: &char) -> u8 {
    if card.is_digit(10) {
        return card.to_string().parse::<u8>().expect("Error trying to parse the card num");
    }

    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        _ => 10
    }
}