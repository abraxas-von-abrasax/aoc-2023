use std::fs;

fn main() {
    println!("[Day 1] Result Part 1: {}", solve(&Part::ONE));
    println!("[Day 1] Result Part 2: {}", solve(&Part::TWO));
}

fn solve(part: &Part) -> u32 {
    let content = get_content();
    let content = content
        .split("\n")
        .collect::<Vec<_>>();

    let mut total = 0;

    for line in content {
        total += get_line_sum(line, &part);
    }

    total
}

enum Part {
    ONE,
    TWO,
}

fn get_line_sum(line: &str, part: &Part) -> u32 {
    let first_digit = get_first_digit(line, &part, &Direction::START);
    let last_digit = get_first_digit(line, &part, &Direction::END);
    let concatenated = first_digit.to_string() + last_digit.to_string().as_str();

    concatenated.parse().unwrap()
}

enum Direction {
    START,
    END,
}

fn get_first_digit(line: &str, part: &Part, direction: &Direction) -> u32 {
    let chars: Vec<char> = match direction {
        Direction::START => line.chars().collect(),
        Direction::END => line.chars().rev().collect(),
    };

    let mut tmp = String::new();

    for char in chars {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }

        match part {
            Part::ONE => continue,
            Part::TWO => {
                match direction {
                    Direction::START => tmp.push(char),
                    Direction::END => tmp.insert(0, char),
                };

                if tmp.len() > 2 {
                    match convert_str_to_digit(&tmp) {
                        Some(num) => return num,
                        None => continue,
                    };
                }
            }
        }
    }

    0
}

fn convert_str_to_digit(string: &str) -> Option<u32> {
    match string.to_lowercase() {
        x if x.contains("one") => Some(1),
        x if x.contains("two") => Some(2),
        x if x.contains("three") => Some(3),
        x if x.contains("four") => Some(4),
        x if x.contains("five") => Some(5),
        x if x.contains("six") => Some(6),
        x if x.contains("seven") => Some(7),
        x if x.contains("eight") => Some(8),
        x if x.contains("nine") => Some(9),
        _ => None,
    }
}

fn get_content() -> String {
    fs::read_to_string("data/input.txt")
        .expect("Could not read input file")
}