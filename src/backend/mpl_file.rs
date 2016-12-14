use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use backend::Backend;
use figure::Figure;
use util::msgpack;


const PRELUDE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                                   "/scripts/prelude.py"));


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
