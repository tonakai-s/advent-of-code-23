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
            if ch.is_digit(10) == false && *ch != '.' {
                if let Some(num_at_top) = common::check_top(&vectorized_lines, &line_idx, &char_idx) {
                    accum = accum + num_at_top as usize;
                }
                if let Some(num_at_top) = common::check_bottom(&vectorized_lines, &line_idx, &char_idx) {
                    accum = accum + num_at_top as usize;
                }
                if let Some(num_at_right) = common::check_right(&line, &char_idx) {
                    accum = accum + num_at_right as usize;
                }
                if let Some(num_at_left) = common::check_left(&line, &char_idx) {
                    accum = accum + num_at_left as usize;
                }
                
                
            }
        }
    }

    println!("Part 1 Result: {}", accum);
}