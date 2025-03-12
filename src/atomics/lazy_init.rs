use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    let mut rng = rand::rng();
    rng.random::<u64>()
}
