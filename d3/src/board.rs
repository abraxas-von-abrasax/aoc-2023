use std::fmt::{Display, Formatter};

use crate::field::Field;
use crate::position::Position;
use crate::row::Row;

pub struct Board {
    rows: Vec<Row>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: Vec::new()
        }
    }

    pub fn add_row(&mut self, row: &str, index: usize) {
        self.rows.push(Row::new(row, index));
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            board: self,
            index: 0,
        }
    }

    pub fn get_connectors(&self, fields: &Vec<&Field>) -> Vec<Position> {
        let mut part_positions: Vec<Position> = Vec::new();

        for field in fields.iter() {
            let field_position = field.position();

            let min_x = if field_position.x() == 0 { 0 } else { field_position.x() - 1 };
            let max_x = if field_position.x() == self.max_x() { self.max_x() } else { field_position.x() + 1 };
            let min_y = if field_position.y() == 0 { 0 } else { field_position.y() - 1 };
            let max_y = if field_position.y() == self.max_y() { self.max_y() } else { field_position.y() + 1 };

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if x == field_position.x() && y == field_position.y() {
                        continue;
                    }

                    let field = self.rows[x].get(y);

                    if field.is_part() {
                        part_positions.push(Position::new(x, y));
                    }
                }
            }
        }

        part_positions
    }

    fn max_x(&self) -> usize {
        self.rows.len() - 1
    }

    fn max_y(&self) -> usize {
        self.rows[0].len() - 1
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", row).unwrap();
        }

        Ok(())
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    index: usize,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = &'a Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.board.rows().len() {
            let result = Some(&self.board.rows[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}