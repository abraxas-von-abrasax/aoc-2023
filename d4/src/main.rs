use std::collections::HashMap;
use std::fs;

use crate::card::Card;
use crate::processor::Processor;

mod card;
mod processor;

fn main() {
    solve_first();
    solve_second();
}

fn solve_first() {
    let content = get_content();

    let mut cards: Vec<Card> = Vec::new();

    for line in &content {
        cards.push(Card::new(line));
    }

    let sum: u32 = cards.iter().fold(0, |acc, card| acc + card.calculate_score());
    println!("Part 1: {}", sum);
}

fn solve_second() {
    let content = get_content();

    let mut cards_store: HashMap<u32, Card> = HashMap::new();

    for line in &content {
        let card = Card::new(line);
        cards_store.insert(card.id(), card);
    }

    let sum = Processor::new(cards_store).process_stack();

    println!("Part 2: {}", sum);
}

fn get_content() -> Vec<String> {
    fs::read_to_string("data/input.txt")
        .expect("Could not read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect()
}