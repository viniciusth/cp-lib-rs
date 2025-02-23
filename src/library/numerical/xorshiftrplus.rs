use std::time::Instant;

pub struct XRng {
    x: u64,
    y: u64,
}

impl XRng {
    pub fn new() -> Self {
        let now = Instant::now();
        let elapsed = now.elapsed();
        Self {
            x: elapsed.as_secs(),
            y: elapsed.subsec_nanos() as u64,
        }
    }

    // xorshiftr128+ generator
    pub fn next(&mut self) -> u64 {
        let mut x = self.x;
        let y = self.y;
        self.x = y;
        x ^= x << 23;
        x ^= x >> 17;
        x ^= y;
        self.y = x.wrapping_add(y);
        x
    }

    // Fisher-Yates shuffle
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len() as u64).rev() {
            let j = self.next() % (i+1);
            slice.swap(i as usize, j as usize);
        }
    }
}
