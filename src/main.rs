use atomics::statistics::statistics;

pub mod atomics;
pub mod memory_ordering;

fn main() {
    statistics();
}
