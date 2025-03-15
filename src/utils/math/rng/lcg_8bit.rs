/// A Linear Congruential Generator (LCG) that uses `u32` internally
/// but efficiently extracts `u8` values.
pub struct LCG {
    state: u32,
    buffer: u32, // Stores 4 bytes from a single LCG call
    index: u8,   // Tracks which byte is being used
}

impl LCG {
    /// Creates a new LCG instance with an initial seed
    pub fn new(seed: u32) -> Self {
        Self {
            state: seed,
            buffer: 0,
            index: 4, // Start by forcing a new `u32` generation
        }
    }

    /// Generates the next pseudo-random `u8`
    pub fn next(&mut self) -> u8 {
        // If all 4 bytes are consumed, generate a new `u32` random number
        if self.index >= 4 {
            self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
            self.buffer = self.state; // Store the generated `u32`
            self.index = 0;
        }

        // Extract the next `u8` from the stored `u32`
        let result = (self.buffer >> (self.index * 8)) as u8;
        self.index += 1;
        result
    }

    /// Generates a number within a given range `[min, max]`
    pub fn next_in_range(&mut self, min: u8, max: u8) -> u8 {
        assert!(min < max, "Invalid range: min must be less than max");
        min + (self.next() % (max - min + 1))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut rng = LCG::new(42); // Seed the generator

        let mut buf: Vec<u8> = Vec::new();
        let mut buf_ranged: Vec<u8> = Vec::new();

        for _ in 0..10000 {
            buf.push(rng.next());
            buf_ranged.push(rng.next_in_range(0, 15));

        }

        println!("buf: {:?}", buf);
        println!("\n\n\nbuf_ranged: {:?}", buf_ranged);

        assert_eq!(1,1);
    }}

