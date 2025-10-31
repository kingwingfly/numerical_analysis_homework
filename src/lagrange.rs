use std::path::PathBuf;

use anyhow::Result;
use peroxide::fuga::*;
use plotters::prelude::*;

const N: usize = 100;

pub fn lagrange<F, I>(
    caption: impl AsRef<str>,
    f: F,
    start: f64,
    end: f64,
    ns: I,
    is_chebyshev_nodes: bool,
) -> Result<()>
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
        for x in {
            match is_chebyshev_nodes {
                true => chebyshev_nodes(n + 1, start, end),
                false => (0..=n)
                    .map(|i| start + (end - start) / n as f64 * i as f64)
                    .collect::<Vec<_>>(),
            }
        } {
            xs.push(x);
            ys.push(f(x));
        }
        let lagrange_polynomial = lagrange_polynomial(xs, ys);
        let mut xys = vec![];
        for x in (0..=N).map(|i| start + (end - start) / N as f64 * i as f64) {
            let y = lagrange_polynomial.eval(x);
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
