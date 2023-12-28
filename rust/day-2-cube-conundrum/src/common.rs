pub fn extract_sets_of_cubes(line: &str) -> String {
    let game_colon_byte = line.find(":").unwrap();
    line[(game_colon_byte + 1)..line.len()].trim().to_string()
}

pub fn format_sets(game_sets: &str) -> Vec<&str> {
    game_sets.split(";").map(|set| set.trim()).collect::<Vec<&str>>()
}

pub fn extract_game_id(game_line: &str) -> u16 {
    let comma_byte = game_line.find(":").unwrap();
    let str_id: String = game_line[0..comma_byte].chars().filter(|ch| ch.is_digit(10)).collect();
    str_id.parse::<u16>().unwrap()
}