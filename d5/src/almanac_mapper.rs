use std::fmt::{Display, Formatter};

pub struct AlmanacMapper {
    s_start: u64,
    d_start: u64,
    range_len: u64,
}

impl AlmanacMapper {
    pub fn new(s_start: u64, d_start: u64, range_len: u64) -> Self {
        Self {
            s_start,
            d_start,
            range_len,
        }
    }

    pub fn map(&self, source: u64) -> Option<u64> {
        if self.range_len == 0 {
            return None;
        }

        let s_end = self.s_start + self.range_len - 1;

        if source < self.s_start || s_end < source {
            return None;
        }

        let diff = source - self.s_start;

        Some(self.d_start + diff)
    }

    pub fn map_rev(&self, dest: u64) -> Option<u64> {
        if self.range_len == 0 {
            return None;
        }

        let d_end = self.d_end() - 1;

        if dest < self.d_start || d_end < dest {
            return None;
        }

        let diff = dest - self.d_start;

        Some(self.s_start + diff)
    }

    pub fn d_start(&self) -> u64 {
        self.d_start
    }

    pub fn d_end(&self) -> u64 {
        self.d_start + self.range_len
    }
}

impl Display for AlmanacMapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} > {} | {}", self.s_start, self.d_start, self.range_len)
    }
}