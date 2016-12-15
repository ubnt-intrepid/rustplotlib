use std::io::{self, Write};
use std::process::{Command, Child, Stdio};
use backend::Backend;
use figure::Figure;
use util::msgpack;

#[cfg_attr(rustfmt, rustfmt_skip)]
const PRELUDE: &'static str =
  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/prelude.py"));

/// Represents an instance of Python process which executes operations.
pub struct Matplotlib {
  child: Child,
}

impl Matplotlib {
  /// create an instance of Matplotlib backend.
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

  /// execute a string as Python script.
  pub fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self> {
    {
      let ref mut stdin = self.child.stdin.as_mut().unwrap();
      stdin.write_all(script.as_ref().as_bytes())?;
      stdin.write_all(b"\n")?;
    }
    Ok(self)
  }

  /// wait until all operations are finished.
  pub fn wait(&mut self) -> io::Result<()> {
    self.child.wait().and(Ok(()))
  }
}

impl<'a> Backend<'a> for Matplotlib {
  /// replace the instance which named 'fig' to a new Figure.
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
