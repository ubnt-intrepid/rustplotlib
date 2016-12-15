use std::io;
use axes2d::Axes2D;
use backend::Backend;

/// Represents an instance of `matplotlib.figure.Figure`.
#[derive(Debug, Default)]
pub struct Figure<'a> {
  rows: u32,
  cols: u32,
  axes: Vec<Option<Axes2D<'a>>>,
}

impl<'a> Figure<'a> {
  /// create an empty instance of `Figure`.
  ///
  /// This method is the shortcut of `Default::default()`.
  pub fn new() -> Figure<'a> {
    Default::default()
  }

  /// set the axes object in the figure.
  pub fn subplots(mut self, rows: u32, cols: u32, axes: Vec<Option<Axes2D<'a>>>) -> Self {
    self.axes.clear();
    self.rows = rows;
    self.cols = cols;
    self.axes = axes;

    let remains = (self.rows * self.cols) as usize - self.axes.len();
    if remains > 0 {
      self.axes.extend((0..remains).into_iter().map(|_| None));
    }
    self
  }

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    mpl.figure()?;
    for (i, axes) in self.axes.iter().enumerate() {
      if let &Some(ref axes) = axes {
        mpl.subplot(self.rows, self.cols, (i + 1) as u32)?;
        axes.apply(mpl)?;
      }
    }
    Ok(())
  }
}
