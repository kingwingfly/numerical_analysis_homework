use std::path::PathBuf;

use anyhow::Result;
use peroxide::{fuga::*, traits::matrix::SolveKind};
use plotters::prelude::*;

const N: usize = 100;

pub fn spline<F, I>(caption: impl AsRef<str>, f: F, start: f64, end: f64, ns: I) -> Result<()>
where
    F: Fn(f64) -> f64,
    I: IntoIterator<Item = usize>,
{
    let path = PathBuf::from("assets").join(sanitize_filename::sanitize(format!(
        "{}.svg",
        caption.as_ref()
    )));
    let root = SVGBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut min_y = f64::INFINITY;
    let mut max_y = -f64::INFINITY;

    let mut xys = vec![];
    for x in (0..=N).map(|i| start + (end - start) / N as f64 * i as f64) {
        let y = f(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        xys.push((x, y));
    }

    let ns = ns.into_iter().collect::<Vec<_>>();
    let max = ns.iter().copied().max().unwrap_or_default();
    let mut lagrange_xys = vec![];
    for &n in ns.iter() {
        let mut xs = vec![];
        let mut ys = vec![];
        for x in (0..=n)
            .map(|i| start + (end - start) / n as f64 * i as f64)
            .collect::<Vec<_>>()
        {
            xs.push(x);
            ys.push(f(x));
        }
        let cubic_spline = cubic_spline(&xs, &ys)?;
        let mut xys = vec![];
        for x in (0..=N).map(|i| start + (end - start) / N as f64 * i as f64) {
            let y = cubic_spline.eval(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            xys.push((x, y));
        }
        lagrange_xys.push(xys);
    }

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(start..end, min_y..max_y)?;
    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(xys, RED.stroke_width(3)))?
        .label("target")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    for (n, xys) in ns.into_iter().zip(lagrange_xys) {
        let color = RGBColor(0, 255 - (n as f64 / max as f64 * 255.) as u8, 0);
        chart
            .draw_series(LineSeries::new(xys, &color))?
            .label(format!("{n}"))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.6))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

pub fn draw(caption: impl AsRef<str>, start: f64, end: f64, spline: CubicSpline) -> Result<()> {
    let path = PathBuf::from("assets").join(sanitize_filename::sanitize(format!(
        "{}.svg",
        caption.as_ref()
    )));
    let root = SVGBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut min_y = f64::INFINITY;
    let mut max_y = -f64::INFINITY;

    let mut xys = vec![];
    for x in (0..=N).map(|i| start + (end - start) / N as f64 * i as f64) {
        let y = spline.eval(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        xys.push((x, y));
    }

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(start..end, min_y..max_y)?;
    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(xys, RED.stroke_width(3)))?
        .label("spline")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.6))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

pub fn myspline<const N: usize>(
    x: [f64; N],
    fx: [f64; N],
    (df0, dfn): (f64, f64),
) -> Result<CubicSpline> {
    assert!(x.is_sorted(), "x should be sorted and correspond to fx");

    let h: Vec<f64> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let divided_difference = move |a: usize, b: usize| {
        if a == b {
            if a == 0 {
                df0
            } else if a == N - 1 {
                dfn
            } else {
                unreachable!()
            }
        } else {
            (fx[a] - fx[b]) / (x[a] - x[b])
        }
    };
    let lambda = [1.]
        .into_iter()
        .chain(h.windows(2).map(|w| w[1] / (w[0] + w[1])))
        .collect::<Vec<_>>();

    let mut coef = Matrix {
        data: vec![0.; N * N],
        row: N,
        col: N,
        shape: Shape::Row,
    };
    for i in 0..N {
        coef[(i, i)] = 2.;
        if i == 0 {
            coef[(i, i + 1)] = lambda[i];
        } else if i == N - 1 {
            coef[(i, i - 1)] = 1.;
        } else {
            coef[(i, i - 1)] = 1. - lambda[i];
            coef[(i, i + 1)] = lambda[i];
        }
    }

    let d = (0..N)
        .map(|i| {
            6. * (divided_difference(i, (i + 1).min(N - 1)) - divided_difference(i.saturating_sub(1), i)) / {
                if i == 0 {
                    h[0]
                } else if i == N - 1 {
                    h[i - 1]
                } else {
                    h[i - 1] + h[i]
                }
            }
        })
        .collect::<Vec<_>>();

    let m = coef.solve(&d, SolveKind::LU);

    Ok(CubicSpline::from(
        (0..N - 1)
            .map(|i| {
                let xi = x[i];
                let yi = fx[i];
                let yi1 = fx[i + 1];
                let mi = m[i];
                let mi1 = m[i + 1];
                let hi = h[i];

                let a = yi;
                let b = (yi1 - yi) / hi - hi * (2.0 * mi + mi1) / 6.0;
                let c = mi / 2.0;
                let d = (mi1 - mi) / (6.0 * hi);

                let a3 = d;
                let a2 = -3.0 * d * xi + c;
                let a1 = 3.0 * d * xi * xi - 2.0 * c * xi + b;
                let a0 = -d * xi.powi(3) + c * xi * xi - b * xi + a;

                (x[i]..x[i + 1], Polynomial::new(vec![a3, a2, a1, a0]))
            })
            .collect::<Vec<_>>(),
    ))
}
