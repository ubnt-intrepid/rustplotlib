use std::io;
use backend::Backend;

/// Represents an instance of `matplotlib.axes.Axes`.
#[derive(Debug, Default)]
pub struct Axes2D<'a> {
  plot_data: Vec<PlotData<'a>>,
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
  legend: Option<String>,
  xlim: Option<(f64, f64)>,
  ylim: Option<(f64, f64)>,
}

impl<'a> Axes2D<'a> {
  /// add a plot data.
  pub fn add<P: Into<PlotData<'a>>>(mut self, p: P) -> Self {
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

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    for ref plot in &self.plot_data {
      plot.apply(mpl)?;
    }
    if let Some(ref xlabel) = self.xlabel {
      mpl.xlabel(xlabel)?;
    }
    if let Some(ref ylabel) = self.ylabel {
      mpl.ylabel(ylabel)?;
    }
    mpl.grid(self.grid)?;
    if let Some(ref loc) = self.legend {
      mpl.legend(loc)?;
    }
    if let Some(ref xlim) = self.xlim {
      mpl.xlim(xlim)?;
    }
    if let Some(ref ylim) = self.ylim {
      mpl.ylim(ylim)?;
    }
    Ok(())
  }
}


/// Plot type.
#[derive(Debug)]
pub enum PlotData<'a> {
  Scatter(Scatter<'a>),
  Line2D(Line2D<'a>),
  FillBetween(FillBetween<'a>),
}

impl<'a> PlotData<'a> {
  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    match *self {
      PlotData::Scatter(ref s) => s.apply(mpl),
      PlotData::Line2D(ref l) => l.apply(mpl),
      PlotData::FillBetween(ref f) => f.apply(mpl),
    }
  }
}

#[derive(Debug, Default)]
pub struct Scatter<'a> {
  xdata: &'a [f64],
  ydata: &'a [f64],
  label: Option<String>,
  color: Option<String>,
  marker: Option<String>,
}

impl<'a> Scatter<'a> {
  pub fn new(name: &str) -> Scatter<'a> {
    Scatter::default().label(name)
  }

  pub fn data(mut self, xdata: &'a [f64], ydata: &'a [f64]) -> Self {
    self.xdata = xdata;
    self.ydata = ydata;
    self
  }

  pub fn label(mut self, text: &str) -> Self {
    self.label = Some(text.to_owned());
    self
  }

  pub fn color(mut self, color: &str) -> Self {
    self.color = Some(color.to_owned());
    self
  }

  pub fn marker(mut self, marker: &str) -> Self {
    self.marker = Some(marker.to_owned());
    self
  }

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    mpl.scatter(self.xdata,
               self.ydata,
               &self.label,
               &self.color,
               &self.marker)?;
    Ok(())
  }
}

impl<'a> From<Scatter<'a>> for PlotData<'a> {
  fn from(data: Scatter) -> PlotData {
    PlotData::Scatter(data)
  }
}


#[derive(Debug, Default)]
pub struct Line2D<'a> {
  xdata: &'a [f64],
  ydata: &'a [f64],
  label: Option<String>,
  color: Option<String>,
  marker: Option<String>,
  linestyle: Option<String>,
  linewidth: Option<f64>,
}

impl<'a> Line2D<'a> {
  pub fn new(name: &str) -> Line2D<'a> {
    Line2D::default().label(name)
  }

  pub fn data(mut self, xdata: &'a [f64], ydata: &'a [f64]) -> Self {
    self.xdata = xdata;
    self.ydata = ydata;
    self
  }

  pub fn label(mut self, text: &str) -> Self {
    self.label = Some(text.to_owned());
    self
  }

  pub fn color(mut self, color: &str) -> Self {
    self.color = Some(color.to_owned());
    self
  }

  pub fn marker(mut self, marker: &str) -> Self {
    self.marker = Some(marker.to_owned());
    self
  }

  pub fn linestyle(mut self, style: &str) -> Self {
    self.linestyle = Some(style.to_owned());
    self
  }

  pub fn linewidth(mut self, width: f64) -> Self {
    self.linewidth = Some(width);
    self
  }

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    mpl.plot(self.xdata,
            self.ydata,
            &self.label,
            &self.color,
            &self.marker,
            &self.linestyle,
            &self.linewidth)?;
    Ok(())
  }
}

impl<'a> From<Line2D<'a>> for PlotData<'a> {
  fn from(data: Line2D<'a>) -> PlotData<'a> {
    PlotData::Line2D(data)
  }
}


#[derive(Debug, Default)]
pub struct FillBetween<'a> {
  x: &'a [f64],
  y1: &'a [f64],
  y2: &'a [f64],
  where_: Option<&'a [bool]>,
  interpolate: bool,
  step: Option<String>,
}

impl<'a> FillBetween<'a> {
  pub fn data(mut self, x: &'a [f64], y1: &'a [f64], y2: &'a [f64]) -> Self {
    self.x = x;
    self.y1 = y1;
    self.y2 = y2;
    self
  }

  pub fn where_(mut self, where_: &'a [bool]) -> Self {
    self.where_ = Some(where_);
    self
  }

  pub fn interpolate(mut self, interpolate: bool) -> Self {
    self.interpolate = interpolate;
    self
  }

  pub fn step(mut self, step: &str) -> Self {
    self.step = Some(step.to_owned());
    self
  }

  pub fn apply<B: Backend>(&self, mpl: &mut B) -> io::Result<()> {
    mpl.fill_between(self.x,
                    self.y1,
                    self.y2,
                    &self.where_,
                    self.interpolate,
                    &self.step)?;
    Ok(())
  }
}

impl<'a> From<FillBetween<'a>> for PlotData<'a> {
  fn from(data: FillBetween<'a>) -> PlotData<'a> {
    PlotData::FillBetween(data)
  }
}
