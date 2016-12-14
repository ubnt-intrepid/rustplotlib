use axes2d::PlotData;

#[derive(Debug, Clone, Default, RustcEncodable)]
pub struct Scatter {
  x: Vec<f64>,
  y: Vec<f64>,
  config: ScatterConfig,
}

#[derive(Debug,Clone,Default,RustcEncodable)]
pub struct ScatterConfig {
  label: Option<String>,
  color: Option<String>,
  marker: Option<String>,
}

impl Scatter {
  pub fn new(name: &str) -> Scatter {
    Scatter::default().label(name)
  }

  pub fn data<X, Y>(mut self, x: X, y: Y) -> Self
    where X: Into<Vec<f64>>,
          Y: Into<Vec<f64>>
  {
    self.x = x.into();
    self.y = y.into();
    self
  }

  pub fn label(mut self, text: &str) -> Self {
    self.config.label = Some(text.to_owned());
    self
  }

  pub fn color(mut self, color: &str) -> Self {
    self.config.color = Some(color.to_owned());
    self
  }

  pub fn marker(mut self, marker: &str) -> Self {
    self.config.marker = Some(marker.to_owned());
    self
  }
}

impl Into<PlotData> for Scatter {
  fn into(self) -> PlotData {
    PlotData::Scatter(self)
  }
}
