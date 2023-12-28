use std::fs;
use crate::common;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(&path).unwrap();

    let mut accum: u32 = 0;
    for line in content.lines() {
        let only_sets = common::extract_sets_of_cubes(&line);
        let formated_sets = common::format_sets(&only_sets);

        accum = accum + get_minimum_power_of_sets(formated_sets) as u32;
    }

    println!("Part 2 result: {}", accum);
}

fn get_minimum_power_of_sets(sets: Vec<&str>) -> u16 {
    let mut red_cube_minimum: u16 = 0;
    let mut green_cube_minimum: u16 = 0;
    let mut blue_cube_minimum: u16 = 0;
    for set in sets.iter() {
        set.split(',')
            .map(|set| set.trim())
            .for_each(|set| {
                let (cube_quantity, cube_color) = set.split_at(set.find(" ").unwrap());
                let quantity: u16 = cube_quantity.parse().expect(&format!("Error parsing the cube quantity: {}", cube_quantity));

                match cube_color.trim() {
                    "red" => {
                        if quantity > red_cube_minimum {
                            red_cube_minimum = quantity;
                        }
                    },
                    "green" => {
                        if quantity > green_cube_minimum {
                            green_cube_minimum = quantity;
                        }
                    },
                    _ => {
                        if quantity > blue_cube_minimum {
                            blue_cube_minimum = quantity;
                        }
                    }
                };
            });
    }

    red_cube_minimum * green_cube_minimum * blue_cube_minimum
}