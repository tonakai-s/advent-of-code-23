use std::fs;
use crate::common;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(&path).unwrap();

    let mut accum: u16 = 0;
    for line in content.lines() {
        let game_id = common::extract_game_id(line);
        let only_sets = common::extract_sets_of_cubes(&line);
        let formated_sets = common::format_sets(&only_sets);

        if is_valid_game(formated_sets) == true {
            accum = accum + game_id;
        }
    }

    println!("Part 1 result: {}", accum);
}

fn is_valid_game(sets: Vec<&str>) -> bool {
    let mut is_valid_game = true;
    for set in sets.iter() {
        set.split(',')
            .map(|set| set.trim())
            .for_each(|set| {
                let (cube_quantity, cube_color) = set.split_at(set.find(" ").unwrap());
                let limit: u8 = match cube_color.trim() {
                    "red" => 12,
                    "green" => 13,
                    _ => 14
                };

                let quantity: u8 = cube_quantity.parse().expect(&format!("Error parsing the cube quantity: {}", cube_quantity));
                if quantity > limit {
                    is_valid_game = false;
                }
            });
    }

    is_valid_game
}