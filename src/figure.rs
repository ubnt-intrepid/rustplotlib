use std::io;
use axes2d::Axes2D;
use backend::Backend;

/// Represents an instance of `matplotlib.figure.Figure`.
#[derive(Debug, Default)]
pub struct Figure<'a> {
  subplots: Option<Subplots<'a>>,
}

impl<'a> Figure<'a> {
  pub fn subplots(mut self, subplots: Subplots<'a>) -> Self {
    self.subplots = Some(subplots);
    self
  }

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    mpl.figure()?;
    if let Some(ref subplots) = self.subplots {
      subplots.apply(mpl)?;
    }
    Ok(())
  }
}

#[derive(Debug)]
pub struct Subplots<'a> {
  rows: u32,
  cols: u32,
  share_x: bool,
  share_y: bool,
  axes: Vec<Option<Axes2D<'a>>>,
}

impl<'a> Subplots<'a> {
  pub fn new(rows: u32, cols: u32) -> Self {
    Subplots {
      rows: rows,
      cols: cols,
      share_x: false,
      share_y: false,
      axes: (0..((rows * cols) as usize)).map(|_| None).collect(),
    }
  }

  pub fn share_x(mut self, share_x: bool) -> Self {
    self.share_x = share_x;
    self
  }

  pub fn share_y(mut self, share_y: bool) -> Self {
    self.share_y = share_y;
    self
  }

  pub fn at(mut self, n: usize, axes: Axes2D<'a>) -> Self {
    *self.axes.get_mut(n).unwrap() = Some(axes);
    self
  }

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    for (i, axes) in self.axes.iter().enumerate() {
      if let &Some(ref axes) = axes {
        mpl.subplot(self.rows, self.cols, (i + 1) as u32)?;
        axes.apply(mpl)?;
      }
    }
    Ok(())
  }
}
