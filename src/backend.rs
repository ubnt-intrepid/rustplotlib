use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::{Command, Child, Stdio};

use rustc_serialize::base64::{self, ToBase64};
use rmp_serialize::Encoder;
// use cpython;

use figure::Figure;

const PRELUDE: &'static str = r#"#!/usr/bin/env python

import base64
import msgpack
import matplotlib.pyplot as plt

def plot_scatter(ax, data):
    x1, x2 = data[0:2]
    l, c, m = map(lambda s: s.decode('utf-8'), data[2:5])
    ax.scatter(x1, x2, label=l, color=c, marker=m)

def make_plot(ax, data):
    plot_type, data = data
    plot_type = plot_type.decode('utf-8')
    if plot_type == "scatter":
        plot_scatter(ax, data)

def make_axes(ax, data):
    plot   = data[0]
    xlabel = data[1].decode('utf-8')
    ylabel = data[2].decode('utf-8')
    grid   = data[3]
    for p in plot:
        make_plot(ax, p)
    ax.set_xlabel(xlabel)
    ax.set_ylabel(ylabel)
    ax.grid(grid)
    ax.legend(loc='upper left')

def make_figure(fig, data):
    # TODO: support for multiple subplots
    data = data[0]
    ax = fig.add_subplot(1, 1, 1)
    make_axes(ax, data)
"#;

pub trait Backend {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self>;
}

pub struct Matplotlib {
  child: Child,
}

impl Matplotlib {
  pub fn new() -> io::Result<Matplotlib> {
    let mut child = Command::new("python").arg("-")
      .stdin(Stdio::piped())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()?;

    child.stdin.as_mut().unwrap().write_all(PRELUDE.as_bytes())?;

    Ok(Matplotlib { child: child })
  }

  pub fn wait(&mut self) -> Result<(), ()> {
    self.child.wait().and(Ok(())).or(Err(()))
  }
}

impl Backend for Matplotlib {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    let script = to_script(fig);
    self.child.stdin.as_mut().unwrap().write_all(script.as_bytes())?;
    Ok(self)
  }
}

impl Drop for Matplotlib {
  fn drop(&mut self) {
    let _ = self.child.kill();
  }
}

fn to_script(fig: &Figure) -> String {
  let mut buf = Vec::new();
  fig.encode(&mut Encoder::new(&mut buf)).unwrap();
  let data = buf.to_base64(base64::STANDARD);

  let mut s = format!("data = msgpack.unpackb(base64.b64decode(r\"{}\"))\n", data);
  s += "fig = plt.figure()\n";
  s += "make_figure(fig, data)";
  s
}

#[allow(dead_code)]
pub struct MatplotlibNative {
    // TODO: implement
}

impl MatplotlibNative {
  #[allow(dead_code)]
  pub fn new() -> MatplotlibNative {
    // TODO: implement
    MatplotlibNative {}
  }
}

impl Backend for MatplotlibNative {
  #[allow(unused_variables)]
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    // TODO: implement
    Ok(self)
  }
}

pub struct MatplotlibFile {
  file: File,
}

impl MatplotlibFile {
  pub fn new<P: AsRef<Path>>(path: P) -> io::Result<MatplotlibFile> {
    let file = OpenOptions::new().write(true)
      .create(true)
      .truncate(true)
      .open(path)?;
    Ok(MatplotlibFile { file: file })
  }
}

impl Backend for MatplotlibFile {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    let script = format!("{}\n{}\n", PRELUDE, to_script(&fig));
    self.file.write_all(script.as_bytes())?;

    Ok(self)
  }
}
