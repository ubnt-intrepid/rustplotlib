mod scatter;
pub use self::scatter::Scatter;


/// Plot type.
#[derive(Debug, Clone, RustcEncodable)]
pub enum PlotData {
  Scatter(Scatter),
}


/// Represents an instance of `matplotlib.axes.Axes`.
#[derive(Debug, Clone, RustcEncodable)]
pub struct Axes2D {
  plot_data: Vec<PlotData>,
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
  legend: Option<String>,
  xlim: Option<(f64, f64)>,
  ylim: Option<(f64, f64)>,
}

impl Default for Axes2D {
  fn default() -> Axes2D {
    Axes2D {
      plot_data: Vec::new(),
      xlabel: None,
      ylabel: None,
      grid: false,
      legend: None,
      xlim: None,
      ylim: None,
    }
  }
}

impl Axes2D {
  /// create an empty axes.
  ///
  /// This method is the shortcut of `Default::default()`.
  pub fn new() -> Self {
    Default::default()
  }

  /// add a plot data.
  pub fn add<P: Into<PlotData>>(mut self, p: P) -> Self {
    self.plot_data.push(p.into());
    self
  }

  /// set the label text of x axis.
  pub fn xlabel(mut self, text: &str) -> Self {
    self.xlabel = Some(text.to_owned());
    self
  }

  /// set the label text of y axis.
  pub fn ylabel(mut self, text: &str) -> Self {
    self.ylabel = Some(text.to_owned());
    self
  }

  /// set whether the grid is shown or not.
  pub fn grid(mut self, enabled: bool) -> Self {
    self.grid = enabled;
    self
  }

  /// set the location of legend in the axes.
  ///
  /// if the value of `loc` is empty, the legend is hidden.
  pub fn legend(mut self, loc: &str) -> Self {
    self.legend = if loc.trim() != "" {
      Some(loc.to_owned())
    } else {
      None
    };
    self
  }

  /// set the range of x axis.
  pub fn xlim(mut self, lb: f64, ub: f64) -> Self {
    self.xlim = Some((lb, ub));
    self
  }

  /// set the range of y axis.
  pub fn ylim(mut self, lb: f64, ub: f64) -> Self {
    self.ylim = Some((lb, ub));
    self
  }
}
