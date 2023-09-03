use rand::{thread_rng, Rng};

const RAND_MAX: i64 = 32767;

pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(0..RAND_MAX) as f64 / (RAND_MAX + 1) as f64;
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    return min + (max-min) * random_f64();
}
