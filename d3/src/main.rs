use std::collections::HashMap;
use std::fs;

use crate::board::Board;
use crate::field::Field;
use crate::part::Part;
use crate::part_number::PartNumber;
use crate::position::Position;
use crate::utils::create_number;

mod part_number;
mod part;
mod position;
mod board;
mod row;
mod field;
mod utils;

fn main() {
    solve_first();
    solve_second();
}

fn solve_first() {
    let board = get_board();
    let parts_map = get_parts_map(&board);
    let part_numbers = get_part_numbers(&board, &parts_map);

    let sum: u32 = part_numbers.iter().fold(0, |acc, cur| {
        if cur.parts().len() == 0 {
            acc
        } else {
            acc + cur.value()
        }
    });

    println!("Part 1: {}", sum);
}

fn solve_second() {
    let board = get_board();
    let parts_map = get_parts_map(&board);
    let part_numbers = get_part_numbers(&board, &parts_map);

    let mut gear_parts: Vec<&Part> = Vec::new();

    for part in parts_map.values() {
        if part.is_gear() {
            gear_parts.push(part);
        }
    }

    let mut sum: u32 = 0;

    for gear_part in gear_parts {
        let mut parts_of_gear: Vec<&PartNumber> = Vec::new();

        for part_number in part_numbers.iter() {
            if part_number.has_part(gear_part) {
                parts_of_gear.push(part_number);
            }
        }

        if parts_of_gear.len() != 2 {
            continue;
        }

        sum += parts_of_gear[0].value() * parts_of_gear[1].value();
    }

    println!("Part 2: {}", sum);
}

fn get_parts_map(board: &Board) -> HashMap<Position, Part> {
    let mut parts_map = HashMap::new();

    for row in board.iter() {
        for field in row.iter() {
            if field.is_part() {
                let x = field.position().x();
                let y = field.position().y();
                parts_map.insert(Position::new(x, y), Part::new(x, y, field.value()));
            }
        }
    }

    parts_map
}

fn get_part_numbers<'a>(board: &'a Board, parts_map: &'a HashMap<Position, Part>) -> Vec<PartNumber<'a>> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    let mut last_digits_fields: Vec<&Field> = Vec::new();

    for row in board.iter() {
        for field in row.iter() {
            if field.is_digit() {
                last_digits_fields.push(&field);
            } else {
                if last_digits_fields.len() > 0 {
                    let mut chars: Vec<char> = Vec::new();

                    for field in last_digits_fields.iter() {
                        chars.push(field.value());
                    }

                    let mut part_number = PartNumber::new(create_number(&chars));
                    let part_positions = board.get_connectors(&last_digits_fields);

                    for pos in part_positions {
                        if parts_map.contains_key(&pos) {
                            let part = parts_map.get(&pos).unwrap();
                            part_number.add_part(part);
                        }
                    }

                    part_number.finalize();

                    part_numbers.push(part_number);
                    last_digits_fields.clear();
                }
            }
        }
    }

    part_numbers
}


fn get_board() -> Board {
    let content = fs::read_to_string("data/input.txt")
        .expect("Could not read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let mut board = Board::new();

    content.iter().enumerate().for_each(|(i, row)| {
        board.add_row(row.as_str(), i);
    });

    board
}