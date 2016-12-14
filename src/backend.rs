use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
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
    let child = Command::new("python").arg("-")
      .stdin(Stdio::piped())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()?;

    let mut mpl = Matplotlib { child: child };
    mpl.exec(PRELUDE)?;
    Ok(mpl)
  }

  pub fn wait(&mut self) -> Result<(), ()> {
    self.child.wait().and(Ok(())).or(Err(()))
  }

  fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<()> {
    let ref mut stdin = self.child.stdin.as_mut().unwrap();
    stdin.write_all(script.as_ref().as_bytes())?;
    stdin.write_all(b"\n")?;
    Ok(())
  }
}

impl Backend for Matplotlib {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    self.exec(format!(r#"fig = evaluate(r"{}")"#, msgpack(fig)))?;
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
  path: PathBuf,
  fig: Option<Figure>,
}

impl MatplotlibFile {
  pub fn new<P: AsRef<Path>>(path: P) -> io::Result<MatplotlibFile> {
    Ok(MatplotlibFile {
      path: path.as_ref().to_path_buf(),
      fig: None,
    })
  }

  pub fn flush(&self) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true)
      .create(true)
      .truncate(true)
      .open(&self.path)?;

    file.write_all(PRELUDE.as_bytes())?;
    file.write_all(b"\n")?;
    if let Some(ref fig) = self.fig {
      file.write_all(format!(r#"fig = evaluate(r"{}")"#, msgpack(fig)).as_bytes())?;
      file.write_all(b"\n")?;
    }

    Ok(())
  }
}

impl Backend for MatplotlibFile {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self> {
    self.fig = Some(fig.clone());
    Ok(self)
  }
}
