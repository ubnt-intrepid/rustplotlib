#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use backend::Backend;
use figure::Figure;
use util::msgpack;
use cpython::{GILGuard, Python};


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

  pub fn exec<S: AsRef<str>>(&mut self, script: S) -> &mut Self {
    self.gil.python().run(script.as_ref(), None, None).unwrap();
    self
  }
}

impl<'a> Backend<'a> for MatplotlibNative {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    self.exec(format!("fig = evaluate('{}')", msgpack(fig)));
    Ok(self)
  }
}
