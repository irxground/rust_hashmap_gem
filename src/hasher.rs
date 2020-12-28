use std::hash;

#[derive(Default)]
pub struct HashBuilder;

impl hash::BuildHasher for HashBuilder {
    type Hasher = Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Hasher(0)
    }
}

pub struct Hasher(u64);

impl hash::Hasher for Hasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = u64::rotate_left(self.0 ^ (b as u64), 8);
        }
    }

    fn write_u32(&mut self, i: u32) {
        self.0 ^= i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.0 ^= i as u64;
    }

    fn write_usize(&mut self, i: usize) {
        self.0 ^= i as u64;
    }

    fn write_i32(&mut self, i: i32) {
        self.0 ^= i as u64;
    }

    fn write_i64(&mut self, i: i64) {
        self.0 ^= i as u64;
    }

    fn write_isize(&mut self, i: isize) {
        self.0 ^= i as u64;
    }
}
