use std::fs;
use crate::common;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Unable to open the input file.");

    let mut hands: Vec<common::Hand> = vec![];
    for line in content.lines() {
        let (cards, mut bid): (&str, &str) = line.split_at(line.find(" ").unwrap());
        bid = bid.trim();
        hands.push(common::Hand::new(cards, bid, true));
    }

    common::bubble_sort_hands(&mut hands, get_card_power);

    let mut accum: usize = 0;
    for (idx, hand) in hands.iter().enumerate() {
        accum += hand.bid as usize * (idx + 1);
    }

    println!("Part 2 result: {accum}");
}

fn get_card_power(card: &char) -> u8 {
    if card.is_digit(10) {
        return card.to_string().parse::<u8>().expect("Error trying to parse the card num");
    }

    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        _ => 10
    }
}