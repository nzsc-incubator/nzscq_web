// https://en.wikipedia.org/wiki/Xorshift#xorshift+
use crate::app::JsPrng;
use crate::opponent::Random;

use std::{f64, u32};

#[derive(Debug, Clone)]
pub struct Xorshift128Plus(u64, u64);

impl Xorshift128Plus {
    pub fn new(seed: (u64, u64)) -> Xorshift128Plus {
        let mut generator = Xorshift128Plus(seed.0, seed.1);
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

impl From<[u32; 4]> for Xorshift128Plus {
    fn from(seed: [u32; 4]) -> Xorshift128Plus {
        let bytes: Vec<[u8; 4]> = seed
            .iter()
            .map(|component| component.to_be_bytes())
            .collect();

        Xorshift128Plus::new((
            u64::from_be_bytes([
                bytes[0][0],
                bytes[0][1],
                bytes[0][2],
                bytes[0][3],
                bytes[1][0],
                bytes[1][1],
                bytes[1][2],
                bytes[1][3],
            ]),
            u64::from_be_bytes([
                bytes[2][0],
                bytes[2][1],
                bytes[2][2],
                bytes[2][3],
                bytes[3][0],
                bytes[3][1],
                bytes[3][2],
                bytes[3][3],
            ]),
        ))
    }
}

impl From<JsPrng> for Xorshift128Plus {
    fn from(mut prng: JsPrng) -> Xorshift128Plus {
        let seed = [
            (prng.random() * f64::from(u32::MAX)) as u32,
            (prng.random() * f64::from(u32::MAX)) as u32,
            (prng.random() * f64::from(u32::MAX)) as u32,
            (prng.random() * f64::from(u32::MAX)) as u32,
        ];

        seed.into()
    }
}