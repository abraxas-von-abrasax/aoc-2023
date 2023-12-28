use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, PartialEq)]
pub enum AlmanacKey {
    Seed,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl AlmanacKey {
    pub fn to_str(&self) -> &str {
        match self {
            AlmanacKey::Seed => "seed",
            AlmanacKey::SeedToSoil => "seed-to-soil",
            AlmanacKey::SoilToFertilizer => "soil-to-fertilizer",
            AlmanacKey::FertilizerToWater => "fertilizer-to-water",
            AlmanacKey::WaterToLight => "water-to-light",
            AlmanacKey::LightToTemperature => "light-to-temperature",
            AlmanacKey::TemperatureToHumidity => "temperature-to-humidity",
            AlmanacKey::HumidityToLocation => "humidity-to-location",
        }
    }

    pub fn parse_key(line: &str) -> Self {
        if line.starts_with("seeds:") {
            AlmanacKey::Seed
        } else if line.starts_with("seed-to-soil") {
            AlmanacKey::SeedToSoil
        } else if line.starts_with("soil-to-fertilizer") {
            AlmanacKey::SoilToFertilizer
        } else if line.starts_with("fertilizer-to-water") {
            AlmanacKey::FertilizerToWater
        } else if line.starts_with("water-to-light") {
            AlmanacKey::WaterToLight
        } else if line.starts_with("light-to-temperature") {
            AlmanacKey::LightToTemperature
        } else if line.starts_with("temperature-to-humidity") {
            AlmanacKey::TemperatureToHumidity
        } else if line.starts_with("humidity-to-location") {
            AlmanacKey::HumidityToLocation
        } else {
            panic!("Unknown almanac section: {}", line);
        }
    }

    pub fn get_next_key(key: &AlmanacKey) -> Option<Self> {
        match key {
            AlmanacKey::Seed => Some(AlmanacKey::SeedToSoil),
            AlmanacKey::SeedToSoil => Some(AlmanacKey::SoilToFertilizer),
            AlmanacKey::SoilToFertilizer => Some(AlmanacKey::FertilizerToWater),
            AlmanacKey::FertilizerToWater => Some(AlmanacKey::WaterToLight),
            AlmanacKey::WaterToLight => Some(AlmanacKey::LightToTemperature),
            AlmanacKey::LightToTemperature => Some(AlmanacKey::TemperatureToHumidity),
            AlmanacKey::TemperatureToHumidity => Some(AlmanacKey::HumidityToLocation),
            AlmanacKey::HumidityToLocation => None,
        }
    }

    pub fn get_previous_key(key: &AlmanacKey) -> Option<Self> {
        match key {
            AlmanacKey::HumidityToLocation => Some(AlmanacKey::TemperatureToHumidity),
            AlmanacKey::TemperatureToHumidity => Some(AlmanacKey::LightToTemperature),
            AlmanacKey::LightToTemperature => Some(AlmanacKey::WaterToLight),
            AlmanacKey::WaterToLight => Some(AlmanacKey::FertilizerToWater),
            AlmanacKey::FertilizerToWater => Some(AlmanacKey::SoilToFertilizer),
            AlmanacKey::SoilToFertilizer => Some(AlmanacKey::SeedToSoil),
            _ => None,
        }
    }
}

impl Display for AlmanacKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
