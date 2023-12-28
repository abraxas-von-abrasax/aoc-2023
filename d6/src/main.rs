use std::fs;

use crate::race::Race;

mod race;

fn main() {
    solve_first();
    solve_second();
}

fn solve_first() {
    let races = get_races();

    let sum: u64 = races
        .iter()
        .map(|race| race.calc_winning_ways())
        .collect::<Vec<u64>>()
        .iter()
        .fold(1, |acc, cur| acc * cur);

    println!("Part 1: {}", sum);
}

fn solve_second() {
    let race = get_single_race();
    println!("Part 2: {}", race.calc_winning_ways());
}

fn get_races() -> Vec<Race> {
    let nums = get_numeric_lines();
    let len = nums[0].len();

    let times = &nums[0];
    let distances = &nums[1];

    let mut races: Vec<Race> = Vec::new();

    for i in 0..len {
        races.push(Race::new(times[i], distances[i]));
    }

    races
}

fn get_single_race() -> Race {
    let nums = get_numeric_lines();

    let join = |nums: &Vec<u64>| -> String {
        nums.iter()
            .map(|num| num.to_string())
            .fold(String::new(), |acc, cur| acc + &cur)
    };
    let t: u64 = join(&nums[0]).parse().unwrap();
    let d: u64 = join(&nums[1]).parse().unwrap();
    Race::new(t, d)
}

fn get_numeric_lines() -> Vec<Vec<u64>> {
    fs::read_to_string("data/input.txt")
        .expect("Could not read input file")
        .split("\n")
        .map(
            |line| String::from(line)
                .split(':')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<String>>()
                .iter()
                .map(|num_str| num_str.parse().unwrap())
                .collect::<Vec<u64>>()
        )
        .collect::<Vec<Vec<u64>>>()
}