mod mpl;
#[cfg(feature = "native")]
mod mpl_native;

use std::io;
use figure::Figure;

pub use self::mpl::Matplotlib;
#[cfg(feature = "native")]
pub use self::mpl_native::MatplotlibNative;


pub trait Backend<'a> {
  /// execute a string as Python script.
  fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self>;

  fn evaluate(&'a mut self, fig: &'a Figure) -> io::Result<&'a mut Self> {
    fig.apply(self)?;
    Ok(self)
  }
}
