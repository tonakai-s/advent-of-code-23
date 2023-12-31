use std::fs;
use crate::common;

pub fn resolve() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("Error reading the input file.");

    let mut lines = content.lines();
    
    let mut times = lines.next().unwrap();
    times = &times[(times.find(':').unwrap() + 1)..times.len()];
    let times: Vec<&str> = times.split_ascii_whitespace().map(|time| time.trim()).collect();

    let mut distances = lines.next().unwrap();
    distances = &distances[(distances.find(':').unwrap() + 1)..distances.len()];
    let distances: Vec<&str> = distances.split_ascii_whitespace().map(|distance| distance.trim()).collect();

    let mut races: Vec<common::Race> = vec![];
    for idx in 0..times.len() {
        races.push(
            common::Race {
                time: times.get(idx).unwrap().parse::<u32>().unwrap(),
                record: distances.get(idx).unwrap().parse::<u32>().unwrap()
            }
        );
    }

    let mut margin_of_error = 0;
    for race in &mut races {
        let mut available_strategies = 0;
        for hold_time in 1..race.time {
            let run_time = race.time - hold_time;
            let run_distance = hold_time * run_time;
 
            if run_distance > race.record {
                available_strategies += 1;
            }
        }

        if margin_of_error == 0 {
            margin_of_error = available_strategies;
            continue;
        }

        margin_of_error = margin_of_error * available_strategies;
    }

    println!("Part1 Result: {margin_of_error}");
}