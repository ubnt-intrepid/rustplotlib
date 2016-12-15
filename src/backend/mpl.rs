use std::io::{self, Write};
use std::process::{Command, Child, Stdio};
use backend::Backend;

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
    mpl.exec("import matplotlib.pyplot as plt")?;
    Ok(mpl)
  }

  /// wait until all operations are finished.
  pub fn wait(&mut self) -> io::Result<()> {
    self.child.wait().and(Ok(()))
  }
}

impl<'a> Backend<'a> for Matplotlib {
  /// execute a string as Python script.
  fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self> {
    {
      let ref mut stdin = self.child.stdin.as_mut().unwrap();
      stdin.write_all(script.as_ref().as_bytes())?;
      stdin.write_all(b"\n")?;
    }
    Ok(self)
  }
}

impl Drop for Matplotlib {
  fn drop(&mut self) {
    let _ = self.child.kill();
  }
}
