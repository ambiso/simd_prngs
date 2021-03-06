use std::arch::x86_64::*;

use rng_impl::*;

/// Intel's `rand_sse` SSE2 LCG
///
/// - Cycle Length: 2^34
/// - Correlation: unknown
///
/// <https://software.intel.com/en-us/articles/fast-random-number-generator-on-the-intel-pentiumr-4-processor>
pub struct IntelLcg {
    cur_seed: u32x4,
}

impl_rngcore! { IntelLcg }

impl SimdRng for IntelLcg {
    type Result = u32x4;

    #[inline(always)]
    fn generate(&mut self) -> u32x4 {
        const MULT: u32x4 = u32x4::new(214013, 17405, 214013, 69069);
        const GADD: u32x4 = u32x4::new(2531011, 10395331, 13737667, 1);
        const MASK: u32x4 = u32x4::new(0xFFFFFFFF, 0, 0xFFFFFFFF, 0);

        let shuffle = |x: u32x4| shuffle!(x, x, [2, 3, 0, 1]);
        let mul = |x, mul| {
            u32x4::from_bits(unsafe {
                _mm_mul_epu32(__m128i::from_bits(x), __m128i::from_bits(mul))
            })
        };

        let mut cur_seed_split = shuffle(self.cur_seed);

        self.cur_seed = mul(self.cur_seed, MULT);
        let multiplier = shuffle(MULT);
        cur_seed_split = mul(cur_seed_split, multiplier);

        self.cur_seed &= MASK;
        cur_seed_split &= MASK;
        cur_seed_split = shuffle(cur_seed_split);
        self.cur_seed |= cur_seed_split;
        self.cur_seed += GADD;

        // They also recommend discarding the lower 16 bits to improve quality
        self.cur_seed
    }
}

impl SeedableRng for IntelLcg {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        // merely 32-bit seed, quality might be improved with larger seeds
        let seed = u32::from_ne_bytes(seed);
        let cur_seed = u32x4::splat(seed) + u32x4::new(0, 1, 0, 1);

        Self { cur_seed }
    }
}
