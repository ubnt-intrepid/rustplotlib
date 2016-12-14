use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::{Command, Child, Stdio};

use rustc_serialize::base64::{self, ToBase64};
// use cpython;

use figure::Figure;

const PRELUDE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                                   "/scripts/prelude.py"));

fn msgpack(fig: &Figure) -> String {
  use rmp_serialize::Encoder;
  use rustc_serialize::Encodable;
  let mut buf = Vec::new();
  fig.encode(&mut Encoder::new(&mut buf)).unwrap();
  buf.to_base64(base64::STANDARD)
}


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
    {
      let ref mut stdin = self.child.stdin.as_mut().unwrap();
      stdin.write_all(format!("data = r\"{}\"\n", msgpack(fig)).as_bytes())?;
      stdin.write_all(b"data = msgpack.unpackb(base64.b64decode(data)\n")?;
      stdin.write_all(b"fig = plt.figure()\n")?;
      stdin.write_all(b"make_figure(fig, data)\n")?;
    }
    Ok(self)
  }
}

impl Drop for Matplotlib {
  fn drop(&mut self) {
    let _ = self.child.kill();
  }
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
    let mut s = format!("data = msgpack.unpackb(base64.b64decode(r\"{}\"))\n",
                        msgpack(fig));
    s += "fig = plt.figure()\n";
    s += "make_figure(fig, data)";
    let script = format!("{}\n{}\n", PRELUDE, s);
    self.file.write_all(script.as_bytes())?;

    Ok(self)
  }
}
