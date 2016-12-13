use axes2d::PlotData;
use encode::{Encoder, EncodeResult, Encodable};

#[derive(Debug, RustcEncodable)]
pub struct Scatter {
  x: Vec<f64>,
  y: Vec<f64>,
  label: Option<String>,
  color: String,
  marker: String,
}

impl Scatter {
  pub fn new() -> Scatter {
    Scatter {
      x: Vec::new(),
      y: Vec::new(),
      label: None,
      color: "blue".to_owned(),
      marker: "o".to_owned(),
    }
  }

  pub fn data(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
    self.x = x;
    self.y = y;
    self
  }

  pub fn label(mut self, text: &str) -> Self {
    self.label = Some(text.to_owned());
    self
  }

  pub fn color(mut self, color: &str) -> Self {
    self.color = color.to_owned();
    self
  }

  pub fn marker(mut self, marker: &str) -> Self {
    self.marker = marker.to_owned();
    self
  }
}

impl PlotData for Scatter {
  fn plot_type(&self) -> &'static str {
    "scatter"
  }
}

impl Encodable for Scatter {
  fn encode(&self, s: &mut Encoder) -> EncodeResult {
    use rustc_serialize::Encodable;
    Encodable::encode(self, s)
  }
}
