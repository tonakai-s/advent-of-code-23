use std::collections::HashMap;

pub fn get_data(input: &String) -> (Vec<char>, HashMap<&str, (String, String)>) {
    let mut instructions: Vec<char> = vec![];
    let mut lines = input.lines();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut part = line.chars().take_while(|ch| ch.is_alphabetic()).collect::<Vec<char>>();
        instructions.append(&mut part);
    }

    let mut maps: HashMap<&str, (String, String)> = HashMap::new();
    for line in &mut lines {
        let (mut id, coordinates): (&str, &str) = line.split_at( line.find('=').unwrap() );
        id = id.trim();

        let (left, right): (&str, &str) = coordinates.split_at( coordinates.find(',').unwrap() );
        let left = left.chars().filter(|ch| ch.is_alphabetic()).collect::<String>();
        let right = right.chars().filter(|ch| ch.is_alphabetic()).collect::<String>();

        maps.insert(id, (left, right));
    }

    (instructions, maps)
}