use std::thread;

use spin_lock::spin_lock::SpinLock;

pub mod atomics;
pub mod memory_ordering;
pub mod spin_lock;

fn main() {
    let x = SpinLock::new(vec![]);
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    println!("g {:?}", *g);
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
