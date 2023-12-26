use std::fmt::{Display, Formatter};

use crate::position::Position;

pub struct Field {
    value: char,
    position: Position,
}

impl Field {
    pub fn new(value: char, x: usize, y: usize) -> Self {
        Self {
            value,
            position: Position::new(x, y),
        }
    }

    pub fn value(&self) -> char {
        self.value
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn is_digit(&self) -> bool {
        self.value.is_digit(10)
    }

    pub fn is_part(&self) -> bool {
        !self.value.is_digit(10) && self.value != '.'
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}