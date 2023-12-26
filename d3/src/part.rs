use std::fmt::{Display, Formatter};

use crate::position::Position;

pub struct Part {
    position: Position,
    value: char,
    is_gear: bool,
}

impl Part {
    pub fn new(x: usize, y: usize, value: char) -> Self {
        Self {
            position: Position::new(x, y),
            value,
            is_gear: value == '*',
        }
    }

    pub fn is_gear(&self) -> bool {
        self.is_gear
    }
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.value == other.value && self.is_gear == other.is_gear
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{}) {}{}", self.position.x(), self.position.y(), self.value, if self.is_gear { " (gear)" } else { "" })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part::Part;

    #[test]
    fn parts_should_be_equal() {
        let part1 = Part { is_gear: true, value: '*', position: crate::position::Position::new(1, 1) };
        let part2 = Part { is_gear: true, value: '*', position: crate::position::Position::new(1, 1) };

        if part1 != part2 {
            panic!("part1 should be equal to part2");
        }
    }

    #[test]
    fn parts_should_not_be_equal() {
        let part1 = Part { is_gear: true, value: '*', position: crate::position::Position::new(1, 1) };
        let part2 = Part { is_gear: true, value: '*', position: crate::position::Position::new(2, 1) };

        if part1 == part2 {
            panic!("part1 should not be equal to part2")
        }
    }
}