use axes2d::PlotData;

#[derive(Debug, Clone, RustcEncodable)]
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

impl Into<PlotData> for Scatter {
  fn into(self) -> PlotData {
    PlotData::Scatter(self)
  }
}
