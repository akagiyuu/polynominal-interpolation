use itertools::multizip;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelBridge,
    ParallelIterator,
};

fn get_lagrange_barycentric_weights(xs: &[f64]) -> Vec<f64> {
    xs.par_iter()
        .enumerate()
        .map(|(index, &x_index)| {
            let inversed_weight: f64 = xs
                .par_iter()
                .enumerate()
                .filter(|&(i, _)| i != index)
                .map(|(_, &x_i)| x_i)
                .map(|x_i| (x_index - x_i))
                .product();

            1. / inversed_weight
        })
        .collect()
}

pub fn lagrange_interpolation(xs: Vec<f64>, ys: Vec<f64>) -> impl Fn(f64) -> f64 {
    let weights = get_lagrange_barycentric_weights(&xs);

    move |x| {
        if let Some(i) = xs.par_iter().position_first(|&x_i| x == x_i) {
            return ys[i];
        }

        let numerator: f64 = multizip((&weights, &xs, &ys))
            .par_bridge()
            .map(|(&w_i, &x_i, &y_i)| (w_i * y_i) / (x - x_i))
            .sum();
        let denominator: f64 = (&weights, &xs)
            .into_par_iter()
            .map(|(&w_i, &x_i)| w_i / (x - x_i))
            .sum();

        numerator / denominator
    }
}
