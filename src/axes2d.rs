/// Represents an instance of `matplotlib.axes.Axes`.
#[derive(Debug, Default, RustcEncodable)]
pub struct Axes2D {
  plot_data: Vec<PlotData>,
  config: Axes2DConfig,
}

#[derive(Debug, Default, RustcEncodable)]
pub struct Axes2DConfig {
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
  legend: Option<String>,
  xlim: Option<(f64, f64)>,
  ylim: Option<(f64, f64)>,
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
    self.config.xlabel = Some(text.to_owned());
    self
  }

  /// set the label text of y axis.
  pub fn ylabel(mut self, text: &str) -> Self {
    self.config.ylabel = Some(text.to_owned());
    self
  }

  /// set whether the grid is shown or not.
  pub fn grid(mut self, enabled: bool) -> Self {
    self.config.grid = enabled;
    self
  }

  /// set the location of legend in the axes.
  ///
  /// if the value of `loc` is empty, the legend is hidden.
  pub fn legend(mut self, loc: &str) -> Self {
    self.config.legend = if loc.trim() != "" {
      Some(loc.to_owned())
    } else {
      None
    };
    self
  }

  /// set the range of x axis.
  pub fn xlim(mut self, lb: f64, ub: f64) -> Self {
    self.config.xlim = Some((lb, ub));
    self
  }

  /// set the range of y axis.
  pub fn ylim(mut self, lb: f64, ub: f64) -> Self {
    self.config.ylim = Some((lb, ub));
    self
  }
}


/// Plot type.
#[derive(Debug, RustcEncodable)]
pub enum PlotData {
  Scatter(Scatter),
  Plot(Plot),
}

#[derive(Debug, Default, RustcEncodable)]
pub struct Scatter {
  x: Vec<f64>,
  y: Vec<f64>,
  config: ScatterConfig,
}

#[derive(Debug, Default, RustcEncodable)]
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

impl From<Scatter> for PlotData {
  fn from(data: Scatter) -> PlotData {
    PlotData::Scatter(data)
  }
}


#[derive(Debug, Default, RustcEncodable)]
pub struct Plot {
  x: Vec<f64>,
  y: Vec<f64>,
  config: PlotConfig,
}

#[derive(Debug, Default, RustcEncodable)]
pub struct PlotConfig {
  label: Option<String>,
  color: Option<String>,
  marker: Option<String>,
  linestyle: Option<String>,
  linewidth: Option<f64>,
}

impl Plot {
  pub fn new(name: &str) -> Plot {
    Plot::default().label(name)
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

  pub fn linestyle(mut self, style: &str) -> Self {
    self.config.linestyle = Some(style.to_owned());
    self
  }

  pub fn linewidth(mut self, width: f64) -> Self {
    self.config.linewidth = Some(width);
    self
  }
}

impl From<Plot> for PlotData {
  fn from(data: Plot) -> PlotData {
    PlotData::Plot(data)
  }
}