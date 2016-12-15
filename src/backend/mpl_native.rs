use std::io;
use backend::Backend;
use cpython::{GILGuard, Python, PyModule, PyDict, NoArgs};


pub struct MatplotlibNative {
  gil: GILGuard,
  plt: PyModule,
}

impl MatplotlibNative {
  pub fn new() -> MatplotlibNative {
    let gil = Python::acquire_gil();
    let plt;
    {
      let py = gil.python();
      plt = PyModule::import(py, "matplotlib.pyplot").unwrap();
    }
    MatplotlibNative {
      gil: gil,
      plt: plt,
    }
  }

  pub fn py<'a>(&'a self) -> Python<'a> {
    self.gil.python()
  }

  pub fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self> {
    self.py().run(script.as_ref(), None, None).unwrap();
    Ok(self)
  }

  // save current figure as a pickle-format file.
  pub fn dump_pickle<S: AsRef<str>>(&mut self, filename: S) -> io::Result<&mut Self> {
    let pl = PyModule::import(self.py(), "pickle").unwrap();
    let gcf = self.plt.call(self.py(), "gcf", NoArgs, None).unwrap();
    let file = self.py().eval(&format!("open('{}', 'wb')", filename.as_ref()), None, None).unwrap();
    pl.call(self.py(), "dump", (gcf, file), None).unwrap();
    Ok(self)
  }
}

impl Backend for MatplotlibNative {
  /// call `plt.figure()` to create a instance of `matplotlib.figure.Figure`.
  fn figure(&mut self) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "figure", NoArgs, None).unwrap();
    Ok(self)
  }

  fn savefig(&mut self, filename: &str) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "savefig", (filename,), None).unwrap();
    Ok(self)
  }

  fn show(&mut self) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "show", NoArgs, None).unwrap();
    Ok(self)
  }

  fn subplot(&mut self, i: u32, j: u32, k: u32) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "subplot", (i, j, k), None).unwrap();
    Ok(self)
  }

  fn grid(&mut self, grid: bool) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "grid", (grid,), None).unwrap();
    Ok(self)
  }

  fn legend(&mut self, loc: &str) -> io::Result<&mut Self> {
    let kwargs = PyDict::new(self.py());
    kwargs.set_item(self.py(), "loc", loc).unwrap();
    self.plt
      .call(self.py(), "legend", NoArgs, Some(&kwargs))
      .unwrap();
    Ok(self)
  }

  fn xlim(&mut self, xlim: &(f64, f64)) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "xlim", xlim, None).unwrap();
    Ok(self)
  }

  fn ylim(&mut self, ylim: &(f64, f64)) -> io::Result<&mut Self> {
    self.plt.call(self.py(), "ylim", ylim, None).unwrap();
    Ok(self)
  }

  fn scatter(&mut self,
             xdata: &[f64],
             ydata: &[f64],
             label: &Option<String>,
             color: &Option<String>,
             marker: &Option<String>)
             -> io::Result<&mut Self> {
    let kwargs = PyDict::new(self.py());
    kwargs.set_item(self.py(), "label", label).unwrap();
    kwargs.set_item(self.py(), "color", color).unwrap();
    kwargs.set_item(self.py(), "marker", marker).unwrap();
    self.plt.call(self.py(), "scatter", (xdata, ydata), Some(&kwargs)).unwrap();
    Ok(self)
  }

  fn plot(&mut self,
          xdata: &[f64],
          ydata: &[f64],
          label: &Option<String>,
          color: &Option<String>,
          marker: &Option<String>,
          linestyle: &Option<String>,
          linewidth: &Option<f64>)
          -> io::Result<&mut Self> {
    let kwargs = PyDict::new(self.py());
    kwargs.set_item(self.py(), "label", label).unwrap();
    kwargs.set_item(self.py(), "color", color).unwrap();
    kwargs.set_item(self.py(), "marker", marker).unwrap();
    kwargs.set_item(self.py(), "ls", linestyle).unwrap();
    kwargs.set_item(self.py(), "lw", linewidth).unwrap();
    self.plt.call(self.py(), "plot", (xdata, ydata), Some(&kwargs)).unwrap();
    Ok(self)
  }

  fn set_style(&mut self, stylename: &str) -> io::Result<&mut Self> {
    use cpython::FromPyObject;
    let style = self.plt
      .get(self.py(), "style")
      .and_then(|ref style| PyModule::extract(self.py(), style))
      .unwrap();
    style.call(self.py(), "use", (stylename,), None).unwrap();
    Ok(self)
  }
}
