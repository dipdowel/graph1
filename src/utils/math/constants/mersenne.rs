// mersenne_constants.rs
// Collection of Mersenne and Mersenne-like primes for use in hashing, scrambling, RNGs, and modular math.

// === u32 Mersenne and Mersenne-like Primes ===

/// 2^31 - 1 — actual Mersenne prime, 31-bit max signed int
pub const MERSENNE_PRIME_31: u32 = 2_147_483_647; // 0x7FFFFFFF

/// Largest 32-bit unsigned prime (Mersenne-like): 2^32 - 5
pub const MERSENNE_LIKE_PRIME_32: u32 = 4_294_967_291; // 0xFFFFFFFB

/// Close to 2^32 - 15, still prime
pub const MERSENNE_LIKE_PRIME_32_B: u32 = 4_294_967_277; // 0xFFFFFFED

/// 2^24 - 3, fits nicely into smaller hash tables or GPU-friendly ranges
pub const MERSENNE_LIKE_PRIME_24: u32 = 16_777_213; // 0xFFFFFF

// === u64 Mersenne and Mersenne-like Primes ===

/// 2^61 - 1 — a Mersenne prime used in high-performance math libs
pub const MERSENNE_PRIME_61: u64 = 2_305_843_009_213_693_951; // 0x1FFFFFFFFFFFFFFF

/// 2^64 - 59 — Largest known 64-bit unsigned prime (Mersenne-like)
pub const MERSENNE_LIKE_PRIME_64: u64 = 18_446_744_073_709_551_557; // 0xFFFFFFFFFFFFFFC5

/// 2^64 - 257 — Another good 64-bit prime, useful for hash domain wrapping
pub const MERSENNE_LIKE_PRIME_64_B: u64 = 18_446_744_073_709_551_359; // 0xFFFFFFFFFFFFFF01

/// 2^32 * (2^32 - 5) — product of a full u64 Mersenne-style structure
pub const MERSENNE_LIKE_COMPOSITE: u64 = (u32::MAX as u64 + 1) * MERSENNE_LIKE_PRIME_32 as u64; // pseudo-Mersenne structure

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::math::constants::golden_ratio::GOLDEN_RATIO_U32;

    #[test]
    fn test_random_1() {
        let mut randoms: Vec<u32> = Vec::with_capacity(4096);

        for counter in 0..4096 {
            randoms.push(
                ((counter as u64 * GOLDEN_RATIO_U32 as u64) % MERSENNE_LIKE_PRIME_32 as u64) as u32,
            );
        }
        println!("{:?}", randoms);
    }
}
