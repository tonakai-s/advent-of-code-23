use std::fs;
use crate::common;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Unable to open the input file.");

    let mut vectorized_lines: Vec<Vec<char>> = vec![];
    for line in content.lines() {
        let vectorized_line: Vec<char> = line.chars().collect();
        vectorized_lines.push(vectorized_line);
    }

    let mut accum: usize = 0;
    for (line_idx, line) in vectorized_lines.iter().enumerate() {
        for (char_idx, ch) in line.iter().enumerate() {
            let mut adjacent_numbers: Vec<u16> = vec![];
            if *ch ==  '*' {
                let num_at_top = check_top(&vectorized_lines, &line_idx, &char_idx);
                if num_at_top.is_some() {
                    adjacent_numbers.push(num_at_top.unwrap());
                } else {
                    if let Some(num_at_top_right) = check_top_right_diagonal(&vectorized_lines, &line_idx, &char_idx) {
                        adjacent_numbers.push(num_at_top_right);
                    }
                    if let Some(num_at_top_left) = check_top_left_diagonal(&vectorized_lines, &line_idx, &char_idx) {
                        adjacent_numbers.push(num_at_top_left);
                    }
                }

                let num_at_bottom = check_bottom(&vectorized_lines, &line_idx, &char_idx);
                if num_at_bottom.is_some() {
                    adjacent_numbers.push(num_at_bottom.unwrap());
                } else {
                    if let Some(num_at_bottom_right) = check_bottom_right_diagonal(&vectorized_lines, &line_idx, &char_idx) {
                        adjacent_numbers.push(num_at_bottom_right);
                    }
                    if let Some(num_at_bottom_left) = check_bottom_left_diagonal(&vectorized_lines, &line_idx, &char_idx) {
                        adjacent_numbers.push(num_at_bottom_left);
                    }
                }

                if let Some(num_at_right) = common::check_right(&line, &char_idx) {
                    adjacent_numbers.push(num_at_right);
                }
                if let Some(num_at_left) = common::check_left(&line, &char_idx) {
                    adjacent_numbers.push(num_at_left);
                }

                if adjacent_numbers.len() == 2 {
                    let ratio: usize = *adjacent_numbers.first().unwrap() as usize * *adjacent_numbers.last().unwrap() as usize;
                    accum = accum + ratio as usize;
                }
            }
        }
    }

    println!("Part 2 Result: {}", accum);
}

pub fn check_top(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    if line_idx == &0 {
        return None;
    }

    let top_line_of_char = vectorized_lines.get(line_idx - 1).unwrap();
    let char_at_top = top_line_of_char.get(*symbol_idx).unwrap();
    if char_at_top.is_digit(10) {
        return Some(common::extract_number_based_on_index(&top_line_of_char, *symbol_idx));
    }

    None
}

pub fn check_top_right_diagonal(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    if line_idx == &0 {
        return None;
    }

    let top_line_of_char = vectorized_lines.get(line_idx - 1).unwrap();
    let char_at_right_diagonal = top_line_of_char.get(*symbol_idx + 1).unwrap();
    if char_at_right_diagonal.is_digit(10) {
        return Some(common::extract_number_based_on_index(&top_line_of_char, *symbol_idx + 1));
    }

    None
}

pub fn check_top_left_diagonal(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    if *line_idx == 0 {
        return None;
    }

    let top_line_of_char = vectorized_lines.get(line_idx - 1).unwrap();
    let char_at_left_diagonal = top_line_of_char.get(*symbol_idx - 1).unwrap();
    if char_at_left_diagonal.is_digit(10) {
        return Some(common::extract_number_based_on_index(&top_line_of_char, *symbol_idx - 1));
    }

    None
}

pub fn check_bottom(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    let bottom_line_of_char = vectorized_lines.get(line_idx + 1);
    if bottom_line_of_char.is_none() {
        return None;
    }

    let bottom_line_of_char = bottom_line_of_char.unwrap();
    
    let char_at_bottom = bottom_line_of_char.get(*symbol_idx).unwrap();
    if char_at_bottom.is_digit(10) {
        return Some(common::extract_number_based_on_index(&bottom_line_of_char, *symbol_idx));
    }

    None
}

pub fn check_bottom_right_diagonal(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    let bottom_line_of_char = vectorized_lines.get(line_idx + 1);
    if bottom_line_of_char.is_none() {
        return None;
    }

    let bottom_line_of_char = bottom_line_of_char.unwrap();
    let char_at_right_diagonal = bottom_line_of_char.get(*symbol_idx + 1).unwrap();
    if char_at_right_diagonal.is_digit(10) {
        return Some(common::extract_number_based_on_index(&bottom_line_of_char, *symbol_idx + 1));
    }

    None
}

pub fn check_bottom_left_diagonal(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    let bottom_line_of_char = vectorized_lines.get(line_idx + 1);
    if bottom_line_of_char.is_none() {
        return None;
    }

    let bottom_line_of_char = bottom_line_of_char.unwrap();
    let char_at_left_diagonal = bottom_line_of_char.get(*symbol_idx - 1).unwrap();
    if char_at_left_diagonal.is_digit(10) {
        return Some(common::extract_number_based_on_index(&bottom_line_of_char, *symbol_idx - 1));
    }

    None
}