use std::fs;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Error on opening the input file.");

    let mut accum: usize = 0;
    for line in content.lines() {
        let colon_byte = line.find(':').unwrap();
        let all_nums = line[(colon_byte + 1)..line.len()].trim().to_string();

        let mut nums = all_nums.split('|').map(|nums| nums.trim());
        let winning_nums: Vec<&str> = nums.next().unwrap().split_ascii_whitespace().collect();
        let your_nums: Vec<&str> = nums.next().unwrap().split_ascii_whitespace().collect();

        let mut your_winning_nums: usize = 0;
        for your_num in &your_nums {
            if winning_nums.contains(your_num) {
                if your_winning_nums == 0 {
                    your_winning_nums = 1;
                } else {
                    your_winning_nums = your_winning_nums * 2;
                }
            }
        }

        accum = accum + your_winning_nums;
    }

    println!("Part1 result: {accum}");
}