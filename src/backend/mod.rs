mod mpl;
mod mpl_file;
#[cfg(feature = "native")]
mod mpl_native;

use std::io;
use figure::Figure;

pub use self::mpl::Matplotlib;
pub use self::mpl_file::MatplotlibFile;
#[cfg(feature = "native")]
pub use self::mpl_native::MatplotlibNative;


pub trait Backend<'a> {
  fn evaluate(&'a mut self, fig: &'a Figure) -> io::Result<&'a mut Self>;
}
