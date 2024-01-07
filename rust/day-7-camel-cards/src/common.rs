use std::collections::{VecDeque, HashMap};

pub enum GetCardTypeMethod {
    WithJoker,
    Normal
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum HandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<char>,
    pub bid: u64,
    pub hand_type: HandType
}

pub fn bubble_sort_hands(hands: &mut Vec<Hand>, card_pow: fn(&char) -> u8) {
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
                    let hand_card_power = card_pow(&hand.cards[card_idx]);
                    let next_hand_card_power = card_pow(&next_hand.unwrap().cards[card_idx]);

                    if hand_card_power == next_hand_card_power {
                        continue;
                    }

                    if hand_card_power < next_hand_card_power {
                        break;
                    }

                    if hand_card_power > next_hand_card_power {
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
}

impl Hand {
    pub fn new(cards: &str, bid: &str, card_type_with_joker: bool) -> Hand {
        let cards: Vec<char> = cards.chars().collect();
        let joker_cards: Vec<char> = if card_type_with_joker == true {
            Hand::convert_jokers(&cards)
        } else {
            vec![]
        };

        let hand_type = Hand::get_hand_type(if card_type_with_joker == true { &joker_cards } else { &cards });
        Hand {
            cards,
            bid: bid.parse().expect("Error parsing the BID"),
            hand_type
        }
    }

    fn convert_jokers(cards: &Vec<char>) -> Vec<char> {
        let mut converted_cards: Vec<char> = vec![];

        let mut cards_occurrences: HashMap<char, usize> = HashMap::new();
        for card in cards {
            if cards_occurrences.contains_key(card) == true {
                continue;
            }
            
            let occurrence_counter = cards.iter().filter(|filter_card| *filter_card == card).count();
            cards_occurrences.insert(*card, occurrence_counter);
        }

        let most_card = Hand::find_most_occurred(&cards_occurrences);
        for card in cards {
            if *card != 'J' {
                converted_cards.push(*card);
                continue;
            }

            converted_cards.push(most_card);
        }

        converted_cards
    }

    fn find_most_occurred(map: &HashMap<char, usize>) -> char {
        let mut most: (char, usize) = ('Z', 0);
        for (key, value) in map {
            if *value > most.1 && *key != 'J' {
                most = (*key, *value);
            }
        }

        most.0
    }

    pub fn get_hand_type(cards: &Vec<char>) -> HandType {
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