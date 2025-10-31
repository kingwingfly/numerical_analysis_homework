mod lagrange;
mod spline;

use anyhow::Result;

use lagrange::lagrange;
use spline::spline;

fn main() -> Result<()> {
    lagrange(
        r#"\frac{1}{1+25x^2}"#,
        |x| 1. / (1. + 25. * x.powi(2)),
        -1.,
        1.,
        1..=8,
        false,
    )?;
    lagrange(
        r#"\frac{x}{1+x^4}"#,
        |x| x / (1. + x.powi(4)),
        -5.,
        5.,
        1..=8,
        false,
    )?;
    lagrange(r#"\arctan{x}"#, |x| x.atan(), -5., 5., 1..=8, false)?;

    lagrange(
        r#"\frac{1}{1+25x^2}(chebyshev_nodes)"#,
        |x| 1. / (1. + 25. * x.powi(2)),
        -1.,
        1.,
        1..=8,
        true,
    )?;
    lagrange(
        r#"\frac{x}{1+x^4}(chebyshev_nodes)"#,
        |x| x / (1. + x.powi(4)),
        -5.,
        5.,
        1..=8,
        true,
    )?;
    lagrange(
        r#"\arctan{x}(chebyshev_nodes)"#,
        |x| x.atan(),
        -5.,
        5.,
        1..=8,
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
    Ok(())
}
