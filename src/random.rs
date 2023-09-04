use std::ops::Range;

/// Utiltiy functions related to random number generation
use rand::{thread_rng, Rng};

const RAND_MAX: i64 = 32767;

/// Returns a random float in between 0 and 1
pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(0..RAND_MAX) as f64 / (RAND_MAX + 1) as f64;
}

/// Returns a random float within a range
///
/// ## Arguments
/// - `range` Range of numbers to generate within 
pub fn random_f64_in_range(range: Range<f64>) -> f64 {
    return range.start + (range.end-range.start) * random_f64();
}
