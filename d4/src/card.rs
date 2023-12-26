use std::fmt::{Display, Formatter};

pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    pub fn new(line: &str) -> Self {
        let mut parts = line.split(": ");

        let card_part = parts.next()
            .expect("Line does not have content.");
        let id: u32 = card_part.split(' ')
            .last()
            .expect("Card part does not have a card number")
            .parse()
            .expect("Card number is NaN");

        let numbers = parts.next()
            .expect("Line does not contain winning or drawn numbers");

        let mut parts = numbers.split(" | ");

        let winning_numbers: Vec<u32> = parts.next()
            .expect("Line does not contain winning numbers")
            .split(' ')
            .filter(|el| *el != "")
            .map(|el| el.parse::<u32>().expect(format!("Cannot convert winning number: {}", el).as_str()))
            .collect();

        let drawn_numbers: Vec<u32> = parts.next()
            .expect("Line does not contain drawn numbers")
            .split(' ')
            .filter(|el| *el != "")
            .map(|el| el.parse::<u32>().expect(format!("Cannot convert drawn number: {}", el).as_str()))
            .collect();

        Self {
            id,
            winning_numbers,
            drawn_numbers,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_winning_count(&self) -> u32 {
        let mut winning_count: u32 = 0;

        for winning_num in &self.winning_numbers {
            for drawn_num in &self.drawn_numbers {
                if winning_num == drawn_num {
                    winning_count += 1;
                }
            }
        }

        winning_count
    }

    pub fn calculate_score(&self) -> u32 {
        let winning_count = self.get_winning_count();

        if winning_count == 0 {
            return 0;
        }

        (0..winning_count - 1).fold(1, |acc, _| acc * 2)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let winning_numbers_str = self.winning_numbers.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let drawn_numbers_str = self.drawn_numbers.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "#{} winning: [", self.id).unwrap();
        write!(f, "{}], drawn: [", winning_numbers_str).unwrap();
        write!(f, "{}]", drawn_numbers_str)
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            winning_numbers: self.winning_numbers.clone(),
            drawn_numbers: self.drawn_numbers.clone(),
        }
    }
}