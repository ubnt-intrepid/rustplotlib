use scatter::Scatter;

#[derive(Debug, Clone, RustcEncodable)]
pub enum PlotData {
  Scatter(Scatter),
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Axes2D {
  plot_data: Vec<PlotData>,
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
}

impl Axes2D {
  pub fn new() -> Self {
    Axes2D {
      plot_data: Vec::new(),
      xlabel: None,
      ylabel: None,
      grid: false,
    }
  }

  pub fn add<P: Into<PlotData>>(mut self, p: P) -> Self {
    self.plot_data.push(p.into());
    self
  }

  pub fn xlabel(mut self, text: &str) -> Self {
    self.xlabel = Some(text.to_owned());
    self
  }

  pub fn ylabel(mut self, text: &str) -> Self {
    self.ylabel = Some(text.to_owned());
    self
  }

  pub fn grid(mut self, enabled: bool) -> Self {
    self.grid = enabled;
    self
  }
}
