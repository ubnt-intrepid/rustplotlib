extern crate rustplotlib;

use rustplotlib::{backend, Backend};
use rustplotlib::{Figure, Axes2D, Scatter, Plot};
use std::f64::consts::PI;

fn main() {
  let x: Vec<f64> = (0..30).into_iter().map(|i| (i as f64) * 0.08 * PI).collect();
  let y1: Vec<f64> = x.iter().map(|x| x.sin()).collect();
  let y2: Vec<f64> = x.iter().map(|x| x.cos()).collect();

  let fig = Figure::new().axes2d(Axes2D::new()
    .add(Scatter::new("sin(x)")
      .data(&x, &y1)
      .marker("o"))
    .add(Plot::new("cos(x)")
      .data(&x, &y2)
      .color("red")
      .marker("x")
      .linestyle("--")
      .linewidth(1.0))
    .xlabel("Time [sec]")
    .ylabel("Distance [mm]")
    .legend("upper right")
    .xlim(0.0, 8.0)
    .ylim(-2.0, 2.0));

  apply_mpl(&fig, "scatter.png").unwrap();
  apply_mpl_file(&fig, "scatter.py").unwrap();

  #[cfg(feature = "native")]
  apply_mpl_native(&fig, "scatter_native.png").unwrap();
}

fn apply_mpl(fig: &Figure, filename: &str) -> std::io::Result<()> {
  let mut mp = backend::Matplotlib::new()?;
  mp.exec("plt.style.use('ggplot')")?
    .evaluate(fig)?
    .exec(format!("fig.savefig('{}')", filename))?
    .wait()
}

fn apply_mpl_file(fig: &Figure, filename: &str) -> std::io::Result<()> {
  let mut mp = backend::MatplotlibFile::new(filename)?;
  mp.evaluate(fig)?
    .flush()
}

#[cfg(feature = "native")]
fn apply_mpl_native(fig: &Figure, filename: &str) -> std::io::Result<()> {
  let mut mp = backend::MatplotlibNative::new();
  mp.exec("plt.style.use('dark_background')");
  mp.evaluate(fig)?
    .exec(format!("plt.savefig('{}')", filename));
  Ok(())
}
