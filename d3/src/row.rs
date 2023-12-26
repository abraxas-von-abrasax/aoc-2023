use std::fmt::{Display, Formatter};

use crate::field::Field;

pub struct Row {
    row: Vec<Field>,
}

impl Row {
    pub fn new(row: &str, row_index: usize) -> Self {
        let mut _row: Vec<Field> = Vec::new();

        for (j, value) in row.chars().enumerate() {
            _row.push(Field::new(value, row_index, j));
        }

        Self {
            row: _row,
        }
    }

    pub fn get(&self, index: usize) -> &Field {
        &self.row[index]
    }

    pub fn len(&self) -> usize {
        self.row.len()
    }

    pub fn iter(&self) -> RowIterator {
        RowIterator {
            row: self,
            index: 0,
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.row.iter() {
            write!(f, "{}", *c).unwrap();
        }
        Ok(())
    }
}

pub struct RowIterator<'a> {
    row: &'a Row,
    index: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a Field;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.row.row.len() {
            let result = Some(&self.row.row[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}