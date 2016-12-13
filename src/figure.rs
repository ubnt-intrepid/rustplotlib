use axes2d::Axes2D;

///
#[derive(Debug)]
pub struct Figure {
  pub axes: Option<Axes2D>,
}

impl Figure {
  ///
  pub fn new() -> Figure {
    Figure { axes: None }
  }

  ///
  pub fn axes2d(mut self, axes: Axes2D) -> Self {
    self.axes = Some(axes);
    self
  }
}
