use divan::Bencher;
use polynominal_interpolation::{lagrange_interpolation, newton_interpolation};
use std::ops::RangeInclusive;

use itertools::Itertools;
use rand::prelude::*;

fn generate_random_polynomial(d: usize) -> impl Fn(f64) -> f64 {
    let mut random = rand::prelude::StdRng::seed_from_u64(d as u64);

    let mut coefficiences: Vec<f64> = vec![0.; d + 1];
    random.fill(&mut coefficiences[..]);

    move |x| {
        coefficiences
            .iter()
            .skip(1)
            .fold(coefficiences[0], |acc, c_i| acc * x + c_i)
    }
}

const TEST_RANGE: RangeInclusive<i128> = -10..=10;
const DEGREES: [usize; 3] = [1, 5, 10];

#[divan::bench(args = DEGREES)]
fn lagrange(bencher: Bencher, d: usize) {
    bencher
        .with_inputs(|| {
            let original_polynomial = generate_random_polynomial(d);
            let xs = (0..=d).map(|x_i| x_i as f64).collect_vec();
            let ys = xs.iter().map(|&xs| original_polynomial(xs)).collect_vec();

            (xs, ys)
        })
        .bench_values(|(xs, ys)| {
            let polynomial = lagrange_interpolation(xs, ys);

            TEST_RANGE.for_each(|x| {
                polynomial(x as f64);
            })
        })
}

#[divan::bench(args = DEGREES)]
fn newton(bencher: Bencher, d: usize) {
    bencher
        .with_inputs(|| {
            let original_polynomial = generate_random_polynomial(d);
            let xs = (0..=d).map(|x_i| x_i as f64).collect_vec();
            let ys = xs.iter().map(|&xs| original_polynomial(xs)).collect_vec();

            (xs, ys)
        })
        .bench_values(|(xs, ys)| {
            let polynomial = newton_interpolation(xs, ys);

            TEST_RANGE.for_each(|x| {
                polynomial(x as f64);
            })
        })
}

fn main() {
    divan::main()
}
