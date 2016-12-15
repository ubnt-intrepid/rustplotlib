#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use backend::Backend;
use figure::Figure;
use util::msgpack;
use cpython::{GILGuard, Python, PyModule};


#[cfg_attr(rustfmt, rustfmt_skip)]
const PRELUDE: &'static str =
  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/prelude.py"));


pub struct MatplotlibNative {
  gil: GILGuard,
}

impl MatplotlibNative {
  pub fn new() -> MatplotlibNative {
    let gil = Python::acquire_gil();
    gil.python()
      .run(PRELUDE, None, None)
      .unwrap();

    MatplotlibNative { gil: gil }
  }

  pub fn python<'a>(&'a self) -> Python<'a> {
    self.gil.python()
  }

  pub fn exec<S: AsRef<str>>(&mut self, script: S) -> &mut Self {
    self.gil.python().run(script.as_ref(), None, None).unwrap();
    self
  }

  pub fn set_stylesheet(&mut self, stylename: &str) -> io::Result<&mut Self> {
    {
      use cpython::FromPyObject;
      let py = self.python();
      let plt = PyModule::import(py, "matplotlib.pyplot")
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to load matplotlib.pyplot"))?;
      let style = PyModule::extract(py, &plt.get(py, "style").unwrap())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to load the module 'style'"))?;
      style.call(py, "use", (stylename,), None).unwrap();
    }
    Ok(self)
  }

  pub fn savefig(&mut self, filename: &str) -> io::Result<&mut Self> {
    {
      let py = self.python();
      let plt = PyModule::import(py, "matplotlib.pyplot")
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to load matplotlib.pyplot"))?;
      plt.call(py, "savefig", (filename,), None)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to call savefig()"))?;
    }
    Ok(self)
  }
}

impl<'a> Backend<'a> for MatplotlibNative {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    self.exec(format!("fig = evaluate('{}')", msgpack(fig)));
    Ok(self)
  }
}
