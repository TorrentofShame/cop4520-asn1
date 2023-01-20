use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

const THREAD_COUNT: u64 = 8;
const MAX_I: u64 = 10_u64.pow(8);

fn gen_primes(n: u64) -> (u64, u64) {
//    let stime = Instant::now();
//    let st = n;
    let inc = THREAD_COUNT * 2;
    let mut n = n;
    let mut c = 0;
    let mut s = 0;
    while n < MAX_I {
        let mut is_prime = true;
        if (n < 3) | (n % 2 == 0) {
            n += inc;
            continue;
        }
        let max = (n as f64).sqrt().ceil() as u64;
        for i in (3u64..=max).step_by(2) {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            c += 1;
            s += n;
        }
        n += inc;
    }
//    println!("[gen_primes]: time: {}, start: {}, end: {}", stime.elapsed().as_millis(), st, n);
    (c, s)
}

fn main() {
    let s_time = Instant::now();
    let mut prime_count = 1;
    let mut prime_sum = 2;

    let (tx, rx) = channel();

    for i in 0..THREAD_COUNT {
        let tx = tx.clone();

        thread::spawn(move || {
            tx.send(gen_primes(1 + (2*i))).unwrap();
        });

    }

    let mut tleft = THREAD_COUNT;
    while tleft != 0 {
        let (c, s) = rx.recv().unwrap();
        prime_count += c;
        prime_sum += s;

        tleft -= 1;
    }

    println!("{} {} {}",
        s_time.elapsed().as_millis(),
        prime_count,
        prime_sum
    );
}
