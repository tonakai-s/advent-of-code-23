use std::fs;

pub fn resolve() -> String {
    let path = "input2.txt";
    let content = fs::read_to_string(path).expect("Unable to open input file");

    let mut accum: u32 = 0;
    let lines: Vec<&str> = content.lines().collect();

    for line in lines.iter() {
        let line_only_digits = extract_true_digits(&line);
        let first_digit: char = match extract_str_prefix(&line) {
            Some(prefix) => {
                if let Some(digit_on_prefix) = scan_prefix_to_get_digit(&prefix){
                    digit_on_prefix
                } else {
                    line_only_digits.chars().nth(0).unwrap()
                }
            },
            None => {
                line_only_digits.chars().nth(0).unwrap()
            }
        };

        let last_digit = match extract_str_postfix(&line) {
            Some(postfix) => {
                if let Some(digit_on_postfix) = scan_postfix_to_get_digit(&postfix){
                    digit_on_postfix
                } else {
                    line_only_digits.chars().nth(line_only_digits.len() - 1).unwrap()
                }
            },
            None => {
                line_only_digits.chars().nth(line_only_digits.len() - 1).unwrap()
            }
        };

        let digit = format!("{}{}", first_digit, last_digit);
        let int_digit = digit.parse::<u32>().expect(&format!("Unable to parse the final digit {}", digit));

        accum = accum + int_digit;
    }

    accum.to_string()
}

fn scan_prefix_to_get_digit(str_prefix: &str) -> Option<char> {
    let mut digit: Option<char> = None;

    for (start_index, _) in str_prefix.chars().enumerate() {
        for inner_index in start_index..str_prefix.len() + 1 {
            if let Some(digit_eval) = get_str_digit_eval(&str_prefix[start_index..inner_index]){
                digit = Some(digit_eval);
                break;
            }
        }

        if digit.is_some() {
            break;
        }
    }

    digit
}

fn scan_postfix_to_get_digit(str_postfix: &str) -> Option<char> {
    let mut digit: Option<char> = None;

    let reversed_postfix: String = str_postfix.chars().rev().collect();
    for (start_index, _) in reversed_postfix.chars().enumerate() {
        for inner_index in start_index..reversed_postfix.len() + 1 {
            let str_to_verify = &reversed_postfix[start_index..inner_index];
            if let Some(digit_eval) = get_str_digit_eval(&str_to_verify.chars().rev().collect::<String>()){
                digit = Some(digit_eval);
                break;
            }
        }

        if digit.is_some() {
            break;
        }
    }

    digit
}

fn get_str_digit_eval(str_prefix: &str) -> Option<char> {
    match str_prefix {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None
    }
}

fn extract_str_prefix(line: &str) -> Option<String> {
    let str_prefix: String = line.chars().take_while(|ch| { ch.is_digit(10) == false }).collect();
    if str_prefix.is_empty() {
        return None;
    }

    Some(str_prefix)
}

fn extract_str_postfix(line: &str) -> Option<String> {
    let str_prefix: String = line.chars().rev().take_while(|ch| { ch.is_digit(10) == false }).collect();
    if str_prefix.is_empty() {
        return None;
    }

    Some(str_prefix.chars().rev().collect::<String>())
}

fn extract_true_digits(line: &str) -> String {
    line.chars().filter(|ch| { ch.is_digit(10) }).collect()
}