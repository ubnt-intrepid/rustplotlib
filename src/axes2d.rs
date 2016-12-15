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

  pub fn apply<'b, B: Backend<'b> + ?Sized>(&self, mpl: &mut B) -> io::Result<()> {
    for ref plot in &self.plot_data {
      plot.apply(mpl)?;
    }
    mpl.exec(format!("ax.grid({})", if self.grid { "True" } else { "False" }))?;
    if let Some(ref loc) = self.legend {
      mpl.exec(format!("ax.legend(loc='{}')", loc))?;
    }
    if let Some((ref lb, ref ub)) = self.xlim {
      mpl.exec(format!("ax.set_xlim(({}, {}))", lb, ub))?;
    }
    if let Some((ref lb, ref ub)) = self.ylim {
      mpl.exec(format!("ax.set_ylim(({}, {}))", lb, ub))?;
    }
    Ok(())
  }
}


/// Plot type.
#[derive(Debug)]
pub enum PlotData<'a> {
  Scatter(Scatter<'a>),
  Line2D(Line2D<'a>),
}

impl<'a> PlotData<'a> {
  pub fn apply<'b, B: Backend<'b> + ?Sized>(&self, mpl: &mut B) -> io::Result<()> {
    match *self {
      PlotData::Scatter(ref s) => s.apply(mpl),
      PlotData::Line2D(ref l) => l.apply(mpl),
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

  pub fn apply<'b, B: Backend<'b> + ?Sized>(&self, mpl: &mut B) -> io::Result<()> {
    let xdata = to_pyvec(self.xdata);
    let ydata = to_pyvec(self.ydata);
    let mut code = format!("ax.scatter({}, {}, ", xdata, ydata);
    if let Some(ref label) = self.label {
      code += &format!("label='{}', ", label);
    }
    if let Some(ref color) = self.color {
      code += &format!("color='{}', ", color);
    }
    if let Some(ref marker) = self.marker {
      code += &format!("marker='{}', ", marker);
    }
    code += ")";
    mpl.exec(code)?;
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

  pub fn apply<'b, B: Backend<'b> + ?Sized>(&self, mpl: &mut B) -> io::Result<()> {
    let xdata = to_pyvec(self.xdata);
    let ydata = to_pyvec(self.ydata);
    let mut code = format!("ax.plot({}, {}, ", xdata, ydata);
    if let Some(ref label) = self.label {
      code += &format!("label='{}', ", label);
    }
    if let Some(ref color) = self.color {
      code += &format!("color='{}', ", color);
    }
    if let Some(ref marker) = self.marker {
      code += &format!("marker='{}', ", marker);
    }
    if let Some(ref ls) = self.linestyle {
      code += &format!("linestyle='{}', ", ls);
    }
    if let Some(ref lw) = self.linewidth {
      code += &format!("linewidth='{}', ", lw);
    }
    code += ")";
    mpl.exec(code)?;
    Ok(())
  }
}

impl<'a> From<Line2D<'a>> for PlotData<'a> {
  fn from(data: Line2D<'a>) -> PlotData<'a> {
    PlotData::Line2D(data)
  }
}

fn to_pyvec(data: &[f64]) -> String {
  let data: Vec<String> = data.iter().map(|x| format!("{}", x)).collect();
  format!("[{}]", data.join(","))
}