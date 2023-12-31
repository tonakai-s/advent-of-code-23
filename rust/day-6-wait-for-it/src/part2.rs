use std::fs;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Error reading the input file.");

    let mut lines = content.lines();
    
    let times = lines.next().unwrap();
    let time = times[(times.find(':').unwrap() + 1)..times.len()]
                            .split_ascii_whitespace()
                            .map(|time| time.trim())
                            .collect::<Vec<&str>>()
                            .join("")
                            .parse::<u64>()
                            .expect("Error parsing the time");

    let records = lines.next().unwrap();
    let record = records[(records.find(':').unwrap() + 1)..records.len()]
                            .split_ascii_whitespace()
                            .map(|distance| distance.trim())
                            .collect::<Vec<&str>>()
                            .join("")
                            .parse::<u64>()
                            .expect("Error parsing the record");

    let mut available_strategies: u32 = 0;
        for hold_time in 1..time {
            let run_time = time - hold_time;
            let run_distance = hold_time * run_time;
 
            if run_distance > record {
                available_strategies += 1;
            }
        }

    println!("Part2 Result: {available_strategies}");
}