// https://en.wikipedia.org/wiki/Xorshift#xorshift+
use crate::opponent::Random;

use std::{f64, u32};

#[derive(Debug, Clone)]
pub struct Xorshift128Plus(u64, u64);

impl Xorshift128Plus {
    pub fn new(seed: (u64, u64)) -> Xorshift128Plus {
        Xorshift128Plus(seed.0, seed.1)
    }

    fn random_u32(&mut self) -> u32 {
        let bytes = self.random_u64().to_be_bytes();

        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn random_u64(&mut self) -> u64 {
        let mut t = self.0;
        let s = self.1;
        self.0 = s;
        t ^= t << 23;
        t ^= t >> 17;
        t ^= s ^ (s >> 26);
        self.1 = t;

        t + s
    }
}

impl Random for Xorshift128Plus {
    fn random(&mut self) -> f64 {
        f64::from(self.random_u32()) / f64::from(u32::MAX)
    }
}