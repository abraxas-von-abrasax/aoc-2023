use std::fmt::{Display, Formatter};

use crate::part::Part;

pub struct PartNumber<'a> {
    finalized: bool,
    value: u32,
    parts: Vec<&'a Part>,
}

impl<'a> PartNumber<'a> {
    pub fn new(value: u32) -> Self {
        Self {
            finalized: false,
            value,
            parts: Vec::new(),
        }
    }

    pub fn add_part(&mut self, part: &'a Part) {
        match self.parts.iter().find(|p| ***p == *part) {
            Some(_) => (),
            None => self.parts.push(part),
        }
    }

    pub fn parts(&self) -> &Vec<&Part> {
        &self.parts
    }

    pub fn has_part(&self, part: &Part) -> bool {
        self.parts.contains(&part)
    }

    pub fn value(&self) -> u32 {
        if !self.finalized {
            panic!("PartNumber is not finalized yet.");
        }
        self.value
    }

    pub fn finalize(self: &mut Self) {
        self.finalized = true;
    }
}

impl<'a> Display for PartNumber<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;

        if self.parts.len() > 0 {
            write!(f, " [")?;
        }

        for part in &self.parts {
            write!(f, "{}", part)?;
        }

        if self.parts.len() > 0 {
            write!(f, "]")?;
        }

        Ok(())
    }
}