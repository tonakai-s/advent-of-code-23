use std::fs;
use crate::common;

pub fn resolve() {
    const ENTRYPOINT: &str = "AAA";
    const ENDPOINT: &str = "ZZZ";

    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Unable to open the input file.");
    let (instructions, maps) = common::get_data(&content);

    let mut steps_counter = 0;
    let mut current_position = maps.get(ENTRYPOINT).unwrap();

    let mut iter_index = 0;

    loop {
        let instruction = instructions[iter_index];
        let next_position_id =  match instruction {
            'L' => current_position.0.as_str(),
            _ => current_position.1.as_str()
        };

        steps_counter += 1;
        if next_position_id == ENDPOINT {
            break;
        }

        current_position = maps.get(next_position_id).unwrap();
        iter_index = if instructions.get(iter_index + 1).is_none() { 0 } else { iter_index + 1 };
    }

    println!("Steps count: {steps_counter}");
}