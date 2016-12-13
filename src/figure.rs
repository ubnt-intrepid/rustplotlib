use axes2d::Axes2D;
use encode::{Encodable, Encoder, EncodeResult};

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
}

impl Encodable for Figure {
  fn encode(&self, s: &mut Encoder) -> EncodeResult {
    use rustc_serialize::Encoder;
    s.emit_option(|s| {
      match self.axes {
        Some(ref axes) => s.emit_option_some(|s| axes.encode(s)),
        None => s.emit_option_none(),
      }
    })
  }
}
