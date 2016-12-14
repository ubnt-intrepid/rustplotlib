use std::io::{self, Write};
use std::process::{Command, Child, Stdio};
use backend::Backend;
use figure::Figure;
use super::msgpack;

const PRELUDE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                                   "/scripts/prelude.py"));

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
