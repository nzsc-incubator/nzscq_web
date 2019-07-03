// https://en.wikipedia.org/wiki/Xorshift#xorshift+
use crate::opponent::Random;

use murmur3::murmur3_32::MurmurHasher;


use std::hash::Hasher;
use std::{f64, u32};
#[derive(Debug, Clone)]
pub struct Xorshift128Plus(u64, u64);

impl Xorshift128Plus {
    pub fn new(a: u64, b: u64) -> Xorshift128Plus {
        let mut generator = Xorshift128Plus(a, b);
        generator.warmup();

        generator
    }

    fn warmup(&mut self) {
        for _ in 0..WARMUP_CYCLES {
            self.random();
        }
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

        t.wrapping_add(s)
    }
}

const WARMUP_CYCLES: usize = 256;

impl Random for Xorshift128Plus {
    fn random(&mut self) -> f64 {
        f64::from(self.random_u32()) / f64::from(u32::MAX)
    }
}

impl From<&str> for Xorshift128Plus {
    fn from(seed: &str) -> Xorshift128Plus {
        let mut hasher: MurmurHasher = Default::default();
        hasher.write(seed.as_ref());
        let a = hasher.finish();
        hasher.write(seed.as_ref());
        let b = hasher.finish();

        Xorshift128Plus::new(a, b)
    }
}

impl From<String> for Xorshift128Plus {
    fn from(seed: String) -> Xorshift128Plus {
        Xorshift128Plus::from(&seed[..])
    }
}
