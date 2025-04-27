use std::{
    sync::atomic::{AtomicBool, Ordering, fence},
    thread,
    time::Duration,
};

// non-atomic shared variable
static mut DATA: [u64; 10] = [0; 10];
#[allow(clippy::declare_interior_mutable_const)]
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

pub fn fences() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Ordering::Release);
        });
    }
    thread::sleep(Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    if ready.contains(&true) {
        // only acquire if there is data to be read
        fence(Ordering::Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data {i} = {}", unsafe { DATA[i] });
            }
        }
    }
}

fn some_calculation(i: usize) -> u64 {
    i as u64
}
