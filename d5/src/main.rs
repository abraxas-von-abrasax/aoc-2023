use std::fs;

use crate::almanac::Almanac;

mod almanac;
mod almanac_key;
mod almanac_mapper;

fn main() {
    solve_first();
    solve_second();
}

fn solve_first() {
    let almanac = create_almanac();
    let locations = almanac.get_locations(None);
    println!("Part 1: {}", locations.iter().min().unwrap());
}

/// This is a very naive implementation. In the end, it brute-forces the result.
/// When I have more time, I will solve it "properly" by reducing the number of seeds with interval algebra.
fn solve_second() {
    let almanac = create_almanac();
    let locations = almanac.create_seed_ranges_and_get_locations();
    println!("Part 2: {}", locations.iter().min().unwrap());
}

fn create_almanac() -> Almanac {
    let content = fs::read_to_string("data/input.txt")
        .expect("Could not read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    Almanac::new(content)
}