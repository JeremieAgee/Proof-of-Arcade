use serde::{Deserialize, Serialize};

/// Versioned PRNG selector. Must be incremented when algorithm changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MazePrngVersion {
    SplitMix64V1 = 1,
}

impl MazePrngVersion {
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            1 => Some(MazePrngVersion::SplitMix64V1),
            _ => None,
        }
    }

    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

/// SplitMix64 PRNG — fast, simple, deterministic.
/// Reference: https://xorshift.di.unimi.it/splitmix64.c
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64 { state: seed }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let seed = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        SplitMix64::new(seed)
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z ^ (z >> 27)
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }

    pub fn next_range(&mut self, max: u32) -> u32 {
        if max == 0 {
            return 0;
        }
        ((self.next_u32() as u64 * max as u64) >> 32) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_deterministic() {
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(42);

        for _ in 0..100 {
            assert_eq!(a.next(), b.next());
        }
    }

    #[test]
    fn splitmix64_different_seeds_different_values() {
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(43);

        let mut same_count = 0;
        for _ in 0..100 {
            if a.next() == b.next() {
                same_count += 1;
            }
        }
        // Very unlikely to be the same even once
        assert_eq!(same_count, 0);
    }
}
