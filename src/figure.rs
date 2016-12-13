use rmp_serialize::{self, Encoder};
use axes2d::Axes2D;

#[derive(Debug)]
pub struct Figure {
  axes: Option<Axes2D>,
}

impl Figure {
  pub fn new() -> Figure {
    Figure { axes: None }
  }

  pub fn axes2d(mut self, axes: Axes2D) -> Self {
    self.axes = Some(axes);
    self
  }

  pub fn encode(&self, encoder: &mut Encoder) -> Result<(), rmp_serialize::encode::Error> {
    use rustc_serialize::Encoder;
    encoder.emit_seq(1, |s| {
      if let Some(ref axes) = self.axes {
        axes.encode(s)?;
      }
      Ok(())
    })
  }
}
