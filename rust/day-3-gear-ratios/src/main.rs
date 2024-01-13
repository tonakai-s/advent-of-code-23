use std::time::Instant;
use day_3_gear_ratios::{part1, part2};

fn main() {
    part1::resolve();
    // TODO: The part2 solution is digusting, fix this later if have time...
    let start = Instant::now();
    part2::resolve();
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}