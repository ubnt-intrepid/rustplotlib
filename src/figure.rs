use axes2d::Axes2D;

/// Represents an instance of `matplotlib.figure.Figure`.
#[derive(Debug, Default, RustcEncodable)]
pub struct Figure {
  axes: Option<Axes2D>,
}

impl Figure {
  /// create an empty instance of `Figure`.
  ///
  /// This method is the shortcut of `Default::default()`.
  pub fn new() -> Figure {
    Default::default()
  }

  /// set the axes object in the figure.
  pub fn axes2d(mut self, axes: Axes2D) -> Self {
    self.axes = Some(axes);
    self
  }
}
