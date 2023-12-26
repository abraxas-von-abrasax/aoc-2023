use std::cmp::min;
use std::collections::HashMap;

use crate::card::Card;

pub struct Processor {
    max: u32,
    store: HashMap<u32, Card>,
    stack: Vec<Card>,
}

impl Processor {
    pub fn new(store: HashMap<u32, Card>) -> Self {
        Self {
            max: store.len() as u32,
            store,
            stack: Vec::new(),
        }
    }

    pub fn process_stack(&mut self) -> u32 {
        self.init();
        let mut sum: u32 = 0;
        let mut values_map: HashMap<u32, u32> = HashMap::new();

        for card in self.stack.iter() {
            let rec_sum = self.process_item(card, &mut values_map);
            sum += rec_sum;
        }

        sum
    }

    fn process_item(&self, card: &Card, values_map: &mut HashMap<u32, u32>) -> u32 {
        if values_map.contains_key(&card.id()) {
            return *values_map.get(&card.id()).unwrap();
        }
        // One item is always the current card itself.
        let mut sum: u32 = 1;

        let score = card.get_winning_count();
        let start = min(card.id() + 1, self.max);
        let end = min(start + score - 1, self.max);

        let indices = start..=end;

        let mut copies: Vec<&Card> = Vec::new();

        for index in indices.clone() {
            let copy = self.store.get(&index)
                .expect(
                    format!("Card #{} requested card #{}. Could not find it.", card.id(), index)
                        .as_str()
                );
            copies.push(copy);
        }

        if copies.len() == 0 {
            values_map.insert(card.id(), sum);
            return sum;
        }

        for copy in copies {
            sum += self.process_item(copy, values_map);
        }

        values_map.insert(card.id(), sum);

        sum
    }

    fn init(&mut self) {
        for card in self.store.values() {
            self.stack.push(card.clone());
        }
    }
}