use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use backend::Backend;
use figure::Figure;
use util::msgpack;


#[cfg_attr(rustfmt, rustfmt_skip)]
const PRELUDE: &'static str =
  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/prelude.py"));


/// Matplotlib backend for saving to the file.
pub struct MatplotlibFile<'a> {
  path: PathBuf,
  fig: Option<&'a Figure>,
}

impl<'a> MatplotlibFile<'a> {
  /// create an instance of MatplotlibFile backend.
  pub fn new<P: AsRef<Path>>(path: P) -> io::Result<MatplotlibFile<'a>> {
    Ok(MatplotlibFile {
      path: path.as_ref().to_path_buf(),
      fig: None,
    })
  }

  /// apply all modification to the file.
  pub fn flush(&self) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true)
      .create(true)
      .truncate(true)
      .open(&self.path)?;

    file.write_all(PRELUDE.as_bytes())?;
    file.write_all(r#"
if __name__ == '__main__':
    main()"#
        .as_bytes())?;

    file.write_all(b"\n#==>\n#")?;
    if let Some(ref fig) = self.fig {
      file.write_all(msgpack(fig).as_bytes())?;
    }
    file.write_all(b"\n#<==\n")?;

    Ok(())
  }
}

impl<'a> Backend<'a> for MatplotlibFile<'a> {
  /// replace the internal data of figure.
  fn evaluate(&'a mut self, fig: &'a Figure) -> io::Result<&'a mut Self> {
    self.fig = Some(fig);
    Ok(self)
  }
}
