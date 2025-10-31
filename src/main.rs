mod lagrange;
mod spline;

use std::array;

use anyhow::Result;

use lagrange::lagrange;
use spline::{draw, myspline, spline};

fn main() -> Result<()> {
    lagrange(
        r#"\frac{1}{1+25x^2}"#,
        |x| 1. / (1. + 25. * x.powi(2)),
        -1.,
        1.,
        1..=10,
        false,
    )?;
    lagrange(
        r#"\frac{x}{1+x^4}"#,
        |x| x / (1. + x.powi(4)),
        -5.,
        5.,
        1..=10,
        false,
    )?;
    lagrange(r#"\arctan{x}"#, |x| x.atan(), -5., 5., 1..=10, false)?;

    lagrange(
        r#"\frac{1}{1+25x^2}(chebyshev_nodes)"#,
        |x| 1. / (1. + 25. * x.powi(2)),
        -1.,
        1.,
        1..=10,
        true,
    )?;
    lagrange(
        r#"\frac{x}{1+x^4}(chebyshev_nodes)"#,
        |x| x / (1. + x.powi(4)),
        -5.,
        5.,
        1..=10,
        true,
    )?;
    lagrange(
        r#"\arctan{x}(chebyshev_nodes)"#,
        |x| x.atan(),
        -5.,
        5.,
        1..=10,
        true,
    )?;

    spline(
        r#"\frac{1}{1+25x^2}(spline)"#,
        |x| 1. / (1. + 25. * x.powi(2)),
        -1.,
        1.,
        [10, 20],
    )?;
    spline(
        r#"\frac{x}{1+x^4}(spline)"#,
        |x| x / (1. + x.powi(4)),
        -5.,
        5.,
        [10, 20],
    )?;
    spline(r#"\arctan{x}(spline)"#, |x| x.atan(), -5., 5., [10, 20])?;

    // myspline::<4>([27.7, 28., 29., 30.], [4.1, 4.3, 4.1, 3.], (3., -4.))?;
    let spline = myspline::<11>(
        array::from_fn(|i| i as f64),
        [
            0., 0.79, 1.53, 2.19, 2.71, 3.03, 3.27, 2.89, 3.06, 3.19, 3.29,
        ],
        (0.8, 0.2),
    )?;
    draw("myspline", 0., 11., spline)?;

    Ok(())
}
