/// Represents an instance of `matplotlib.axes.Axes`.
#[derive(Debug, Default, RustcEncodable)]
pub struct Axes2D<'a> {
  plot_data: Vec<PlotData<'a>>,
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

impl<'a> Axes2D<'a> {
  /// create an empty axes.
  ///
  /// This method is the shortcut of `Default::default()`.
  pub fn new() -> Self {
    Default::default()
  }

  /// add a plot data.
  pub fn add<P: Into<PlotData<'a>>>(mut self, p: P) -> Self {
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
pub enum PlotData<'a> {
  Scatter(Scatter<'a>),
  Plot(Plot<'a>),
}

#[derive(Debug, Default, RustcEncodable)]
pub struct Scatter<'a> {
  x: &'a [f64],
  y: &'a [f64],
  config: ScatterConfig,
}

#[derive(Debug, Default, RustcEncodable)]
pub struct ScatterConfig {
  label: Option<String>,
  color: Option<String>,
  marker: Option<String>,
}

impl<'a> Scatter<'a> {
  pub fn new(name: &str) -> Scatter<'a> {
    Scatter::default().label(name)
  }

  pub fn data(mut self, x: &'a [f64], y: &'a [f64]) -> Self {
    self.x = x;
    self.y = y;
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

impl<'a> From<Scatter<'a>> for PlotData<'a> {
  fn from(data: Scatter) -> PlotData {
    PlotData::Scatter(data)
  }
}


#[derive(Debug, Default, RustcEncodable)]
pub struct Plot<'a> {
  x: &'a [f64],
  y: &'a [f64],
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

impl<'a> Plot<'a> {
  pub fn new(name: &str) -> Plot<'a> {
    Plot::default().label(name)
  }

  pub fn data(mut self, x: &'a [f64], y: &'a [f64]) -> Self {
    self.x = x;
    self.y = y;
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

impl<'a> From<Plot<'a>> for PlotData<'a> {
  fn from(data: Plot<'a>) -> PlotData<'a> {
    PlotData::Plot(data)
  }
}