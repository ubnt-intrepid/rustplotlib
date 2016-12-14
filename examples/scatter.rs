extern crate rustplotlib;
extern crate rand;

use rustplotlib::{backend, Backend};
use rustplotlib::{Figure, Axes2D, Scatter};

use rand::distributions::{Range, IndependentSample};

fn main() {
  let mut rng = rand::thread_rng();
  let uniform = Range::new(0.0, 1.0);

  let x1: Vec<f64> = (0..100).into_iter().map(Into::into).collect();
  let x2: Vec<f64> = (0..100).into_iter().map(|_| uniform.ind_sample(&mut rng)).collect();

  let fig = Figure::new().axes2d(Axes2D::new()
    .add(Scatter::new()
      .data(x1.iter().take(50).cloned().collect(),
            x2.iter().take(50).cloned().collect())
      .label("Red")
      .marker("o")
      .color("red"))
    .add(Scatter::new()
      .data(x1.iter().skip(50).cloned().collect(),
            x2.iter().skip(50).cloned().collect())
      .label("Blue")
      .marker("x")
      .color("blue"))
    .xlabel("Time [sec]")
    .ylabel("Distance [mm]")
    .grid(true));

  apply_mpl(&fig, "scatter.png").unwrap();

  backend::MatplotlibFile::new("scatter.py")
    .unwrap()
    .evaluate(&fig)
    .unwrap()
    .flush()
    .unwrap();
}

fn apply_mpl(fig: &Figure, filename: &str) -> std::io::Result<()> {
  let mut mp = backend::Matplotlib::new()?;
  mp.exec("plt.style.use('ggplot')")?
    .evaluate(fig)?
    .exec(format!("fig.savefig('{}')", filename))?
    .wait()
}
