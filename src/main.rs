use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const MAX_I: i32 = 10_i32.pow(8);
const I_GRPS: i32 = MAX_I / 8;

fn is_prime(n:i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let s_time = Instant::now();
    let prime_count = Arc::new(Mutex::new(0));
    let prime_sum = Arc::new(Mutex::new(0_i64));
    let mut handles = vec![];

    for i in 0..8 {
        let prime_count = Arc::clone(&prime_count);
        let prime_sum = Arc::clone(&prime_sum);
        let handle = thread::spawn(move || {
            let mut cnt = prime_count.lock().unwrap();
            let mut psum = prime_sum.lock().unwrap();

            for n in ((I_GRPS*i)+1)..I_GRPS*(i+1) {
                if is_prime(n) {
                    *cnt += 1;
                    *psum += n as i64;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{} {} {}",
        s_time.elapsed().as_millis(), *prime_sum.lock().unwrap(), *prime_count.lock().unwrap());
}
