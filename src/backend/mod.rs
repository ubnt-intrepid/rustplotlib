mod mpl;
mod mpl_native;
mod mpl_file;

use std::io;
use figure::Figure;

pub use self::mpl::Matplotlib;
pub use self::mpl_file::MatplotlibFile;
pub use self::mpl_native::MatplotlibNative;


pub trait Backend {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self>;
}
