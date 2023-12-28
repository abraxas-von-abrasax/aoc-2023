use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};

use crate::almanac_key::AlmanacKey;
use crate::almanac_mapper::AlmanacMapper;

type AlmanacContent = HashMap<AlmanacKey, Vec<AlmanacMapper>>;

pub struct Almanac {
    seeds: Vec<u64>,
    content: AlmanacContent,
}

impl Almanac {
    pub fn new(content: Vec<String>) -> Self {
        let mut seeds: Vec<u64> = Vec::new();

        let mut almanac_content: AlmanacContent = HashMap::new();
        almanac_content.insert(AlmanacKey::SeedToSoil, Vec::new());
        almanac_content.insert(AlmanacKey::SoilToFertilizer, Vec::new());
        almanac_content.insert(AlmanacKey::FertilizerToWater, Vec::new());
        almanac_content.insert(AlmanacKey::WaterToLight, Vec::new());
        almanac_content.insert(AlmanacKey::LightToTemperature, Vec::new());
        almanac_content.insert(AlmanacKey::TemperatureToHumidity, Vec::new());
        almanac_content.insert(AlmanacKey::HumidityToLocation, Vec::new());

        let mut currently_processing: Option<AlmanacKey> = None;

        fn extract_ranges(line: &str) -> (u64, u64, u64) {
            let mut parts = line.trim().split(" ");
            let destination_range_start: u64 = parts.next()
                .expect(format!("Line does not contain the destination range start: {}", line).as_str())
                .parse()
                .expect(format!("Destination range start is NaN: {}", line).as_str());
            let source_range_start: u64 = parts.next()
                .expect(format!("Line does not contain the source range start: {}", line).as_str())
                .parse()
                .expect(format!("Destination range start is NaN: {}", line).as_str());
            let range_length: u64 = parts.next()
                .expect(format!("Line does not contain the range length: {}", line).as_str())
                .parse()
                .expect(format!("range length is NaN: {}", line).as_str());
            (destination_range_start, source_range_start, range_length)
        }

        let mut assign_ranges = |key: &AlmanacKey, ranges: (u64, u64, u64)| {
            let (d_start, s_start, range_len) = ranges;
            let section = almanac_content.get_mut(&key)
                .expect(format!("Could not find almanac section: {}", key).as_str());
            section.push(AlmanacMapper::new(s_start, d_start, range_len));
        };

        for line in content {
            if line == "" {
                currently_processing = None;
                continue;
            }

            match &currently_processing {
                Some(key) => {
                    assign_ranges(key, extract_ranges(&line));
                }
                None => {
                    match AlmanacKey::parse_key(&line) {
                        AlmanacKey::Seed => {
                            seeds = line.split(": ")
                                .last()
                                .expect("Input does not have seed numbers")
                                .split(' ')
                                .filter(|el| *el != "")
                                .map(|el| el.parse::<u64>().expect(format!("Cannot convert string {} to u64.", el).as_str()))
                                .collect();
                        }
                        key => {
                            currently_processing = Some(key);
                        }
                    }
                }
            }
        }

        Self {
            seeds,
            content: almanac_content,
        }
    }

    pub fn traverse_backwards(&self) -> u64 {
        let mut seeds: Vec<u64> = Vec::new();

        for i in 0..self.seeds.len() {
            println!("Creating seed {} of {}", i + 1, self.seeds.len());
            if i % 2 == 1 {
                let seed_start = self.seeds[i - 1];
                let seed_range = self.seeds[i];
                for seed in seed_start..(seed_start + seed_range) {
                    seeds.push(seed);
                }
            }
        }

        seeds.sort();

        let location_mappers = self.content.get(&AlmanacKey::HumidityToLocation)
            .expect("Could not retrieve humidity-to-location mappers");

        let mut smallest_location: u64 = 0;

        for (i, mapper) in location_mappers.iter().enumerate() {
            println!("Processing mapper {} of {}", i + 1, location_mappers.len());
            let d_start = mapper.d_start();
            let d_end = mapper.d_end();

            for dest_i in d_start..=d_end {
                let mut key = AlmanacKey::HumidityToLocation;
                let mut new_dest: u64 = dest_i;

                loop {
                    let src = self.get_source_value(new_dest, &key);
                    println!("  >> Src: {}, dest: {}", src, new_dest);
                    match AlmanacKey::get_previous_key(&key) {
                        Some(prev_key) => {
                            println!("  >> Prev key: {}", prev_key);
                            key = prev_key;
                            new_dest = src;
                        }
                        None => {
                            println!("  >> No key matches");
                            if seeds.contains(&src) {
                                println!(">>> Seeds contain src {}", src);
                                if smallest_location == 0 {
                                    println!(">>> Setting smallest_location the first time (to {})", dest_i);
                                    smallest_location = dest_i;
                                } else if dest_i < smallest_location {
                                    println!(">>> New dest {} is smaller than old {}", dest_i, smallest_location);
                                    smallest_location = dest_i;
                                } else {
                                    println!(">>> New dest {} is NOT smaller than old {}", dest_i, smallest_location);
                                }
                            } else {
                                println!(">>> Source {} not found in seeds.", src);
                            }
                            break;
                        }
                    }
                }
            }
        }

        smallest_location
    }

    pub fn create_seed_ranges_and_get_locations(&self) -> Vec<u64> {
        let mut seeds: Vec<u64> = Vec::new();

        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta}) {msg}"
        )
            .unwrap()
            .progress_chars("##-");
        let pb = ProgressBar::new(self.seeds.len() as u64);
        pb.set_style(style.clone());
        pb.set_message("Processing seeds...");

        for i in 0..self.seeds.len() {
            pb.set_prefix(format!("[{}/?]", i + i));
            if i % 2 == 1 {
                let seed_start = self.seeds[i - 1];
                let seed_range = self.seeds[i];
                for seed in seed_start..(seed_start + seed_range) {
                    seeds.push(seed);
                }
            }
            pb.inc(1);
        }

        pb.finish_and_clear();

        self.get_locations(Some(&seeds))
    }

    pub fn get_locations(&self, seeds: Option<&Vec<u64>>) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::new();

        let _seeds = seeds.unwrap_or_else(|| &self.seeds);

        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta}) {msg}"
        )
            .unwrap()
            .progress_chars("##-");
        let pb = ProgressBar::new(_seeds.len() as u64);
        pb.set_style(style.clone());
        pb.set_message("Processing seeds...");

        for (i, seed) in _seeds.iter().enumerate() {
            pb.set_prefix(format!("[{}/?]", i + 1));
            let mut key = AlmanacKey::Seed;
            let mut source = seed.clone();

            loop {
                match AlmanacKey::get_next_key(&key) {
                    Some(next_key) => {
                        key = next_key;
                        let dest = self.get_mapped_value(source, &key);
                        source = dest;
                    }
                    None => {
                        locations.push(source);
                        break;
                    }
                }
            }

            pb.inc(1);
        }

        pb.finish_and_clear();

        locations
    }

    pub fn print(&self, key: &AlmanacKey) {
        if *key == AlmanacKey::Seed {
            println!("Seeds");
            println!("[{}]", self.seeds.iter().map(|seed| seed.to_string()).collect::<Vec<_>>().join(", "));
            return;
        }

        let section = self.content.get(&key)
            .expect(format!("Cannot find section {}", key.to_str()).as_str());

        println!("{}-map", key.to_str());

        for mapper in section {
            println!("{}", mapper);
        }
    }

    fn get_mapped_value(&self, source: u64, key: &AlmanacKey) -> u64 {
        let mappers = self.content.get(key)
            .expect(format!("Cannot find almanac section: {}", key).as_str());

        for mapper in mappers {
            match mapper.map(source) {
                Some(value) => return value,
                None => continue,
            }
        }

        source
    }

    fn get_source_value(&self, dest: u64, key: &AlmanacKey) -> u64 {
        let mappers = self.content.get(key)
            .expect(format!("Cannot find almanac section: {}", key).as_str());

        for mapper in mappers {
            match mapper.map_rev(dest) {
                Some(value) => return value,
                None => continue
            }
        }

        dest
    }
}