pub fn check_top(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    if line_idx == &0 {
        return None;
    }

    let top_line_of_char = vectorized_lines.get(line_idx - 1).unwrap();
    let char_at_top = top_line_of_char.get(*symbol_idx).unwrap();
    if char_at_top.is_digit(10) {
        return Some(extract_number_based_on_index(&top_line_of_char, *symbol_idx));
    }

    let mut top_accum: u16 = 0;
    let chat_at_right_diagonal = top_line_of_char.get(*symbol_idx + 1).unwrap();
    if chat_at_right_diagonal.is_digit(10) {
        top_accum = top_accum + extract_number_based_on_index(&top_line_of_char, *symbol_idx + 1);
    }

    let chat_at_left_diagonal = top_line_of_char.get(*symbol_idx - 1).unwrap();
    if chat_at_left_diagonal.is_digit(10) {
        top_accum = top_accum + extract_number_based_on_index(&top_line_of_char, *symbol_idx - 1);
    }

    if top_accum == 0 {
        return None;
    }

    Some(top_accum)
}

pub fn check_bottom(vectorized_lines: &Vec<Vec<char>>, line_idx: &usize, symbol_idx: &usize) -> Option<u16> {
    let bottom_line_of_char = vectorized_lines.get(line_idx + 1);
    if bottom_line_of_char.is_none() {
        return None;
    }

    let bottom_line_of_char = bottom_line_of_char.unwrap();
    
    let char_at_bottom = bottom_line_of_char.get(*symbol_idx).unwrap();
    if char_at_bottom.is_digit(10) {
        return Some(extract_number_based_on_index(&bottom_line_of_char, *symbol_idx));
    }

    let mut bottom_accum: u16 = 0;
    let chat_at_right_diagonal = bottom_line_of_char.get(*symbol_idx + 1).unwrap();
    if chat_at_right_diagonal.is_digit(10) {
        bottom_accum = bottom_accum + extract_number_based_on_index(&bottom_line_of_char, *symbol_idx + 1);
    }

    let chat_at_left_diagonal = bottom_line_of_char.get(*symbol_idx - 1).unwrap();
    if chat_at_left_diagonal.is_digit(10) {
        bottom_accum = bottom_accum + extract_number_based_on_index(&bottom_line_of_char, *symbol_idx - 1);
    }

    if bottom_accum == 0 {
        return None;
    }

    Some(bottom_accum)
}

pub fn check_right(current_line: &Vec<char>, symbol_idx: &usize) -> Option<u16> {
    let char_at_right = current_line.get(*symbol_idx + 1);
    if char_at_right.is_none() || char_at_right.unwrap().is_digit(10) == false {
        return None;
    }

    Some(extract_number_based_on_index(&current_line, *symbol_idx + 1))
}

pub fn check_left(current_line: &Vec<char>, symbol_idx: &usize) -> Option<u16> {
    let char_at_right = current_line.get(*symbol_idx - 1);
    if char_at_right.is_none() || char_at_right.unwrap().is_digit(10) == false {
        return None;
    }

    Some(extract_number_based_on_index(&current_line, *symbol_idx - 1))
}

pub fn extract_number_based_on_index(vectorized_line: &Vec<char>, char_num_idx: usize) -> u16 {
    let mut num_str = String::new();
    
    let mut num_initial_index = if char_num_idx == 0 {
        0
    } else {
        let mut start_index = char_num_idx;
        while start_index > 0 {
            let idx = vectorized_line.get(start_index - 1);
            if idx.is_some() && idx.unwrap().is_digit(10) == true {
                start_index = start_index - 1;
            } else {
                break;
            }
        }

        start_index
    };

    num_str.push( *vectorized_line.get(num_initial_index).unwrap() );
    loop {
        let next_ch = vectorized_line.get( num_initial_index + 1 );
        if next_ch.is_none() || next_ch.unwrap().is_digit(10) == false {
            break;
        }

        num_str.push( *next_ch.unwrap() );
        num_initial_index = num_initial_index + 1;
    }

    num_str.parse::<u16>().expect(&format!("Unable to parse the full founded number: {}", num_str))
}