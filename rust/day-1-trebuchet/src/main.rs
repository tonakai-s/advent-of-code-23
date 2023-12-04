use std::fs;

fn main() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Unable to open input file");

    let accumulator = 0;

    for (index, line) in content.lines().enumerate() {
        println!("{} - {}", index, line);

        
    }
}
