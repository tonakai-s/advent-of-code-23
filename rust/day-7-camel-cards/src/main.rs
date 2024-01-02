use std::{fs, collections::VecDeque};

#[derive(Debug)]
enum HandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug)]
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

        println!("Ordered sizes: {:?}", ordered_cards);

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

fn main() {
    let path = "input-example.txt";
    let content = fs::read_to_string(path).expect("Unable to open the input file.");

    let mut hands: Vec<Hand> = vec![];
    for line in content.lines() {
        let (cards, mut bid): (&str, &str) = line.split_at(line.find(" ").unwrap());
        bid = bid.trim();

        hands.push(Hand::new(cards, bid));
    }

    println!("Hands: {:#?}", hands);
}
