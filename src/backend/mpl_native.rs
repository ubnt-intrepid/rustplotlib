#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use backend::Backend;
use figure::Figure;
// use cpython;

pub struct MatplotlibNative {
    // TODO: implement
}


impl MatplotlibNative {
  pub fn new() -> MatplotlibNative {
    // TODO: implement
    MatplotlibNative {}
  }
}

impl Backend for MatplotlibNative {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    // TODO: implement
    Ok(self)
  }
}
