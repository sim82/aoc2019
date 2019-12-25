pub mod graph;
pub mod intcode;
pub mod monitoring;
pub mod spaceimg;
mod test;

pub mod util {
    use std::collections::HashSet;
    pub fn get_prime_factors(mut v: i64) -> Vec<i64> {
        let mut primes = Vec::new();
        let mut factors = HashSet::new();
        for i in 2 as i64.. {
            if primes.iter().filter(|x| i % *x == 0).count() != 0 {
                continue;
            }
            primes.push(i);
            // println!("prime: {}", i);
            while v % i == 0 {
                println!("factor {}", i);
                v /= i;
                factors.insert(i);
            }
            if i > v {
                break;
            }
        }

        factors.iter().cloned().collect()
    }
}
