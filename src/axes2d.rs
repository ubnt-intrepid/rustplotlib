use rmp_serialize::Encoder;
use scatter::Scatter;

pub trait PlotData {
  fn plot_type(&self) -> &'static str;
  fn encode(&self, &mut Encoder);
}


#[derive(Debug, RustcEncodable)]
pub struct Axes2D {
  scatter: Vec<Scatter>,
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
}

impl Axes2D {
  pub fn new() -> Self {
    Axes2D {
      scatter: Vec::new(),
      xlabel: None,
      ylabel: None,
      grid: false,
    }
  }

  pub fn scatter(mut self, s: Scatter) -> Self {
    self.scatter.push(s);
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
