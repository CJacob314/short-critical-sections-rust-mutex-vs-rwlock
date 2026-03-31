#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

#[cfg(feature = "rwlock")]
use std::sync::RwLock;
use std::{arch::x86_64::{__rdtscp, _mm_lfence}, sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU64, Ordering}}, thread, time::Duration};
use std::hint::black_box;

fn main() {
    #[cfg(not(feature = "rwlock"))]
    let counter = Arc::new(Mutex::new(0_usize));

    #[cfg(feature = "rwlock")]
    let counter = Arc::new(RwLock::new(0_usize));

    for nthreads in 1..=15 {
        let start = Arc::new(AtomicBool::new(false));
        let stop = Arc::new(AtomicBool::new(false));
        let total_ops = Arc::new(AtomicU64::new(0));

        let mut handles = Vec::with_capacity(nthreads);

        for _ in 0..nthreads {
            let counter = Arc::clone(&counter);
            let start = Arc::clone(&start);
            let stop = Arc::clone(&stop);
            let total_ops = Arc::clone(&total_ops);

            handles.push(thread::spawn(move || {
                while !start.load(Ordering::Relaxed) {
                    std::hint::spin_loop();
                }

                let mut local_ops = 0_u64;

                while !stop.load(Ordering::Relaxed) {
                    #[cfg(feature = "rwlock")]
                    {
                        let guard = counter.read().unwrap();
                        black_box(*guard);
                    }

                    #[cfg(not(feature = "rwlock"))]
                    {
                        let guard = counter.lock().unwrap();
                        black_box(*guard);
                    }

                    local_ops += 1;
                }

                total_ops.fetch_add(local_ops, Ordering::Relaxed);
            }));
        }

        thread::sleep(Duration::from_millis(500));

        let start_cycles = timestamp();

        start.store(true, Ordering::Relaxed);

        thread::sleep(Duration::from_millis(400));

        stop.store(true, Ordering::Relaxed);

        let end_cycles = timestamp();

        for handle in handles {
            handle.join().unwrap();
        }

        let cycles = end_cycles - start_cycles;
        let ops = total_ops.load(Ordering::Relaxed);
        let ops_per_cycle = ops as f64 / cycles as f64;

        println!("{nthreads} {ops_per_cycle}");
    }
}

#[inline(always)]
fn timestamp() -> u64 {
    let mut dummy = 0_u32;

    unsafe { _mm_lfence() };

    let ts = unsafe { __rdtscp(&raw mut dummy) };

    unsafe { _mm_lfence() }; // Felix Clouter: "LFENCE does not execute until all prior
                             // instructions have completed locally, and no later instruction
                             // begins execution until LFENCE completes."

    ts
}

