# Polynomial Interpolation

A library for interpolating a function using polynomial

## Implemented method

- [Lagrange interpolation](https://mathworld.wolfram.com/LagrangeInterpolatingPolynomial.html)
- [Newton interpolation](https://en.wikipedia.org/wiki/Newton_polynomial)

## Benchmark

polynomial_interpolation  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ lagrange                             │               │               │               │         │
│  ├─ 1                   211.4 µs      │ 947.7 µs      │ 244.5 µs      │ 288.1 µs      │ 100     │ 100
│  ├─ 5                   262.5 µs      │ 1.336 ms      │ 347.3 µs      │ 354.2 µs      │ 100     │ 100
│  ╰─ 10                  282.7 µs      │ 431.3 µs      │ 306.9 µs      │ 311.5 µs      │ 100     │ 100
╰─ newton                               │               │               │               │         │
   ├─ 1                   1.404 µs      │ 2.607 µs      │ 1.455 µs      │ 1.472 µs      │ 100     │ 100
   ├─ 5                   2.073 ms      │ 7.295 ms      │ 3.511 ms      │ 3.547 ms      │ 100     │ 100
   ╰─ 10                  76.28 ms      │ 127.3 ms      │ 115.1 ms      │ 104.6 ms      │ 100     │ 100
