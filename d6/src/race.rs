use std::fmt::{Display, Formatter};

pub struct Race {
    time: u64,
    d_record: u64,
}

impl Race {
    pub fn new(time: u64, d_record: u64) -> Self {
        Self {
            time,
            d_record,
        }
    }

    pub fn calc_winning_ways(&self) -> u64 {
        let is_odd = self.time % 2 != 0;
        let middle = self.time / 2;
        let mut winning_count: u64 = 0;
        let mut winning_ways: Vec<u64> = Vec::new();

        let mut i = middle.clone();

        loop {
            let d = self.calculate_distance(i);

            if d == 0 || d <= self.d_record {
                break;
            }

            winning_ways.push(i);

            if is_odd && i <= middle + 1 {
                winning_count += 1;
            } else if !is_odd && i == middle {
                winning_count += 1;
            } else {
                winning_count += 2;
            }
            i += 1;
        }

        winning_count
    }

    fn calculate_distance(&self, ms_held: u64) -> u64 {
        (self.time - ms_held) * ms_held
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "t: {}, d: {}", self.time, self.d_record)
    }
}