pub fn create_number(vec: &Vec<char>) -> u32 {
    vec.iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .expect("Could not parse string into number")
}