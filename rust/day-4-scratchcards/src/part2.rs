use std::fs;

#[derive(Debug)]
struct Card {
    id: u16,
    instances_quantity: u32,
    winning_nums: Vec<String>,
    your_nums: Vec<String>,
    matches: u16
}

impl Card {
    fn new(id: u16, winning_nums: Vec<String>, your_nums: Vec<String>) -> Card {
        Card {
            id,
            instances_quantity: 1,
            winning_nums,
            your_nums,
            matches: 0
        }
    }

    fn set_matches(&mut self) {
        for num in &self.your_nums {
            if self.winning_nums.contains(num) {
                self.matches = self.matches + 1;
            }
        }
    }
}

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Error on opening the input file.");

    let mut cards: Vec<Card> = vec![];

    for line in content.lines() {
        let colon_byte = line.find(':').unwrap();
        let card_id = line[0..colon_byte + 1].chars()
                            .filter(|ch| ch.is_digit(10))
                            .collect::<String>()
                            .parse::<u16>()
                            .expect(&format!("Error parsing the card ID"));

        let mut all_nums = line[(colon_byte + 1)..line.len()].split('|')
                                                            .map(|nums| nums.trim().to_string());
        let winning_nums: Vec<String> = all_nums.next()
                                            .unwrap()
                                            .split_ascii_whitespace()
                                            .map(|num| num.to_string())
                                            .collect();
        let your_nums: Vec<String> = all_nums.next()
                                        .unwrap()
                                        .split_ascii_whitespace()
                                        .map(|num| num.to_string())
                                        .collect();

        cards.push(Card::new(card_id, winning_nums, your_nums));
    }

    for card_idx in 0..cards.len() {
        {
            let card = &mut cards[card_idx];
            card.set_matches();
        }
        if cards[card_idx].matches > 0 {
            for idx in 1..cards[card_idx].matches + 1 {
                let card_id = cards[card_idx].id;
                let curr_instances_quantity = cards[card_idx].instances_quantity;

                let update_card = cards.get_mut(( (card_id + idx) - 1 ) as usize).unwrap();
                update_card.instances_quantity = update_card.instances_quantity + curr_instances_quantity;
            }
        }
    }

    let mut accum: u32 = 0;
    for card in &cards {
        accum = accum + card.instances_quantity;
    }

    println!("Part2 Result: {}", accum);
}