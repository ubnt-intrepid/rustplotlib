use std::io;
use backend::Backend;
use cpython::{GILGuard, Python, PyModule, PyTuple};


pub struct MatplotlibNative {
  gil: GILGuard,
}

impl MatplotlibNative {
  pub fn new() -> MatplotlibNative {
    let mut mpl = MatplotlibNative { gil: Python::acquire_gil() };
    mpl.exec("import matplotlib.pyplot as plt").unwrap();
    mpl
  }

  pub fn python<'a>(&'a self) -> Python<'a> {
    self.gil.python()
  }
}

impl<'a> Backend<'a> for MatplotlibNative {
  fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self> {
    self.gil.python().run(script.as_ref(), None, None).unwrap();
    Ok(self)
  }

  fn set_style(&mut self, stylename: &str) -> io::Result<&mut Self> {
    {
      use cpython::FromPyObject;
      let py = self.python();
      let plt = PyModule::import(py, "matplotlib.pyplot").unwrap();
      let style = plt.get(py, "style").and_then(|ref style| PyModule::extract(py, style)).unwrap();
      style.call(py, "use", (stylename,), None).unwrap();
    }
    Ok(self)
  }

  fn savefig(&mut self, filename: &str) -> io::Result<&mut Self> {
    {
      let py = self.python();
      let plt = PyModule::import(py, "matplotlib.pyplot").unwrap();
      plt.call(py, "savefig", (filename,), None).unwrap();
    }
    Ok(self)
  }

  fn show(&mut self) -> io::Result<&mut Self> {
    {
      let py = self.python();
      let plt = PyModule::import(py, "matplotlib.pyplot").unwrap();
      plt.call(py, "show", PyTuple::empty(py), None).unwrap();
    }
    Ok(self)
  }
}
