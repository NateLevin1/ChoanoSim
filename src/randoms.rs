use once_cell::unsync::Lazy;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

// chosen at random https://xkcd.com/221/
const RANDOM_SEED: u64 = 230575;

// To do this quickly, we have to use unsafe. Be careful!
static mut RANDOM: Lazy<Xoshiro256Plus> = Lazy::new(|| Xoshiro256Plus::seed_from_u64(RANDOM_SEED));

pub fn random_float() -> f64 {
    unsafe { RANDOM.gen::<f64>() }
}

pub fn random(max: u32) -> u32 {
    (random_float() * max as f64) as u32
}
