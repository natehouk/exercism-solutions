use std::cmp;

fn check_prime(n: u32) -> bool {
    for i in 2..(n as f64).sqrt().floor() as u32 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut i = 2;
    let mut primes = 0;
    loop {
        if check_prime(i) {
            primes += 1;
            if primes == n + 1 {
                return i;
            } 
        }
        i += 1;
    }
}
