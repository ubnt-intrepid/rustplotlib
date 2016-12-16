extern crate rustplotlib;
#[cfg(feature = "native")]
extern crate cpython;

use rustplotlib::{backend, Backend};
use rustplotlib::{Figure, Axes2D, Scatter, Line2D, FillBetween};
use std::f64::consts::PI;

fn main() {
  let x: Vec<f64> = (0..40).into_iter().map(|i| (i as f64) * 0.08 * PI).collect();
  let y1: Vec<f64> = x.iter().map(|x| x.sin()).collect();
  let y2: Vec<f64> = x.iter().map(|x| x.cos()).collect();

  let ax1 = Axes2D::new()
    .add(Scatter::new("sin(x)")
      .data(&x, &y1)
      .marker("o"))
    .add(Line2D::new("cos(x)")
      .data(&x, &y2)
      .color("red")
      .marker("x")
      .linestyle("--")
      .linewidth(1.0))
    .xlabel("Time [sec]")
    .ylabel("Distance [mm]")
    .legend("upper right")
    .xlim(0.0, 8.0)
    .ylim(-2.0, 2.0);
  let ax2 = Axes2D::new().add(FillBetween::new()
    .data(&x, &y1, &y2)
    .interpolate(true));

  let fig = Figure::new().subplots(2, 1, vec![Some(ax1), Some(ax2)]);

  apply_mpl(&fig, "simple.png").unwrap();
  #[cfg(feature = "native")]
  apply_mpl_native(&fig, "simple_native.png").unwrap();
}

fn apply_mpl(fig: &Figure, filename: &str) -> std::io::Result<()> {
  let mut mp = backend::Matplotlib::new()?;
  mp.set_style("ggplot")?;
  fig.apply(&mut mp)?;
  mp.savefig(filename)?
    .dump_pickle(format!("{}.pkl", filename))?
    .wait()
}

#[cfg(feature = "native")]
fn apply_mpl_native(fig: &Figure, filename: &str) -> std::io::Result<()> {
  let mut mp = backend::MatplotlibNative::new();
  mp.set_style("dark_background")?;
  fig.apply(&mut mp)?;
  mp.savefig(filename)?;
  mp.dump_pickle(format!("{}.pkl", filename))?;
  Ok(())
}