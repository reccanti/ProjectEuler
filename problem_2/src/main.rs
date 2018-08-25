#![feature(test)]
extern crate num;
extern crate test;

use num::bigint::BigUint;
use num::Integer;
use std::time::{Duration, SystemTime};
use test::Bencher;

fn naive_sum_even_fibonacci (amount: u32) -> BigUint {
    let init = (
        BigUint::from(1 as u32), // n1
        BigUint::from(2 as u32), // n2
        BigUint::from(2 as u32)  // sum
    );
    let (_, _, sum) = (2..amount).fold(init, |acc, _i| {
        let (n1, n2, sum) = acc;
        let next = n1 + n2.clone();
        if next.is_even() {
            (n2, next.clone(), sum + next)
        } else {
            (n2, next, sum)
        }
    });
    sum
}

fn main() {
    let duration = SystemTime::now();
    let result = naive_sum_even_fibonacci(4000000);
    let time = match duration.elapsed() {
        Ok(elapsed) => {
            elapsed.as_secs()
        }
        Err(_e) => {
            0
        }
    };
    println!("Duration: {}s", time);
    println!("");
    println!("{}", result);
}

#[test]
fn test_naive_sum_even_fibonacci () {
    assert_eq!(naive_sum_even_fibonacci(10), BigUint::from(44 as u32));
}
