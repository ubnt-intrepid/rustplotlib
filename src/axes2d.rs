#[derive(Debug, RustcEncodable)]
pub struct Scatter {
  x: Vec<f64>,
  y: Vec<f64>,
  label: Option<String>,
  color: String,
  marker: String,
}

impl Scatter {
  ///
  pub fn new() -> Scatter {
    Scatter {
      x: Vec::new(),
      y: Vec::new(),
      label: None,
      color: "blue".to_owned(),
      marker: "o".to_owned(),
    }
  }

  ///
  pub fn data(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
    self.x = x;
    self.y = y;
    self
  }

  ///
  pub fn label(mut self, text: &str) -> Self {
    self.label = Some(text.to_owned());
    self
  }

  ///
  pub fn color(mut self, color: &str) -> Self {
    self.color = color.to_owned();
    self
  }

  ///
  pub fn marker(mut self, marker: &str) -> Self {
    self.marker = marker.to_owned();
    self
  }
}

///
#[derive(Debug, RustcEncodable)]
pub struct Axes2D {
  scatter: Vec<Scatter>,
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
}

impl Axes2D {
  ///
  pub fn new() -> Self {
    Axes2D {
      scatter: Vec::new(),
      xlabel: None,
      ylabel: None,
      grid: false,
    }
  }

  ///
  pub fn scatter(mut self, s: Scatter) -> Self {
    self.scatter.push(s);
    self
  }

  ///
  pub fn xlabel(mut self, text: &str) -> Self {
    self.xlabel = Some(text.to_owned());
    self
  }

  ///
  pub fn ylabel(mut self, text: &str) -> Self {
    self.ylabel = Some(text.to_owned());
    self
  }

  ///
  pub fn grid(mut self, enabled: bool) -> Self {
    self.grid = enabled;
    self
  }
}
