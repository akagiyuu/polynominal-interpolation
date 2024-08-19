use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn get_newton_basis(x: f64, xs: &[f64]) -> f64 {
    xs.par_iter().map(|&x_i| x - x_i).product()
}

pub fn newton_interpolation(xs: Vec<f64>, ys: Vec<f64>) -> impl Fn(f64) -> f64 {
    let n = xs.len();
    assert!(n != 0);

    move |x| {
        if n == 1 {
            return ys[0];
        }

        let prev_xs = xs[..n - 1].to_vec();
        let prev_ys = ys[..n - 1].to_vec();

        let x_n = xs[n - 1];
        let y_n = ys[n - 1];

        let basis = get_newton_basis(x, &prev_xs);
        let x_n_basis = get_newton_basis(x_n, &prev_xs);

        let prev_polynominal = newton_interpolation(prev_xs, prev_ys);

        let basis_coefficient = (y_n - prev_polynominal(x_n)) / x_n_basis;

        prev_polynominal(x) + basis_coefficient * basis
    }
}
