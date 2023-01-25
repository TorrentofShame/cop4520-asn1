#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const THREAD_COUNT: i32 = 8;
const LIMIT: i32 = 100_000_000;

// Sieve of Eratosthenes
fn simple_sieve(limit: i32) -> Vec<i32> {
    let mut areprimes = vec![true; limit as usize];
    let mut n = 2;
    while n * n <= limit {
        if areprimes[n as usize] {
            for i in ((n*n)..limit).step_by(n as usize) {
                areprimes[i as usize] = false;
            }            
        }
        n += 1;
    }
    let mut primes = Vec::new();
    for i in 2..limit {
        if areprimes[i as usize] {
            primes.push(i);
        }
    }
    primes
}

fn sieve_segment(low: i32, high: i32, init: &Vec<i32>) -> Vec<i32> {
    let mut areprimes = vec![true; (high - low + 1) as usize];
    for p in init {
        let lowest_mult = if (low / *p) * *p >= low {
            (low / *p) * *p
        } else {
            (low / *p) * *p + *p
        };

        for i in (lowest_mult..=high).step_by(*p as usize) {
            areprimes[(i - low) as usize] = false;
        }
    }
    let mut primes = Vec::new();
    for i in low..=high {
        if areprimes[(i - low) as usize] {
            primes.push(i);
        }
    }
    primes
}

fn seg_sieve(limit: i32, threads: i32) -> (i32, i64) {
    let sqrt_n = (limit as f32).sqrt() as i32 + 1;
    let btm_sieve = Arc::new(simple_sieve(sqrt_n));

    let mut prime_count = (*btm_sieve).len() as i32;
    let mut prime_sum = btm_sieve.iter().fold(0_i64, |a, i| a + *i as i64); 

    let seg_size = (limit - sqrt_n) / threads;

    let (tx, rx) = channel();

    for i in 0..threads {
        let tx = tx.clone();
        let initp = btm_sieve.clone();

        thread::spawn(move || {
            let low = sqrt_n + (seg_size * i);
            let high = if low + seg_size < limit { low + seg_size } else { limit };
            let primels = sieve_segment(low, high, &initp);
            tx.send(
                (primels.len() as i32, primels.iter().fold(0_i64, |a, i| a + *i as i64))
            ).unwrap();
        });

    }

    let mut tleft = threads;
    while tleft != 0 {
        let (count, sum) = rx.recv().unwrap();

        prime_count += count;
        prime_sum += sum;

        tleft -= 1;
    }
    
    (prime_count, prime_sum)
}

fn main() {
    let s_time = Instant::now();

    let (c, s) = seg_sieve(LIMIT, THREAD_COUNT);

    println!("{} {} {}",
        s_time.elapsed().as_millis(),
        c,
        s
    );
}
