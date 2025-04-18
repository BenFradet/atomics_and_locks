use atomics::statistics::statistics;

pub mod atomics;
pub mod memory_ordering;
pub mod spin_lock;

fn main() {
    statistics();
}
