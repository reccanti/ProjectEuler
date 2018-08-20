#![feature(test)]
extern crate test;
use test::Bencher;

// naive implementation
fn get_sum_three_five_multiples(start: u32) -> u32 {
    let mut amount = start;
    let mut sum = 0;
    while amount > 0 {
        amount -= 1;
        if amount % 3 == 0 || amount % 5 == 0 {
            sum += amount;
        }
    }
    sum
}

fn get_sum_three_five_multiples_reduced(start: u32) -> u32 {
    let amount = start - 1;
    let threes = amount / 3;
    let fives = amount / 5;
    let fifteens = amount / (3 * 5);

    ((3 * threes * (threes + 1)) + (5 * fives * (fives + 1)) - (15 * fifteens * (fifteens + 1))) / 2
}

#[test]
fn calculates_sum_three_five() {
    assert_eq!(get_sum_three_five_multiples(10), 23);
}

#[test]
fn calculates_sum_three_five_reduced() {
    assert_eq!(get_sum_three_five_multiples_reduced(10), 23);
}

#[bench]
fn bench_sum_three_five(b: &mut Bencher) {
    b.iter(|| get_sum_three_five_multiples(1000000));
}

#[bench]
fn bench_sum_three_five_reduced(b: &mut Bencher) {
    b.iter(|| get_sum_three_five_multiples_reduced(1000000));
}

fn main() {
    print!("Sum of all three and five multiples below 1000: {}\n", get_sum_three_five_multiples_reduced(1000));
}
