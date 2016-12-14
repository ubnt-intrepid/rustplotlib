mod mpl;
mod mpl_native;
mod mpl_file;

use std::io;
use rustc_serialize::base64::{self, ToBase64};
use figure::Figure;

pub use self::mpl::Matplotlib;
pub use self::mpl_file::MatplotlibFile;
pub use self::mpl_native::MatplotlibNative;


pub trait Backend {
  fn evaluate(&mut self, fig: &Figure) -> io::Result<&mut Self>;
}

fn msgpack(fig: &Figure) -> String {
  use rmp_serialize::Encoder;
  use rustc_serialize::Encodable;
  let mut buf = Vec::new();
  fig.encode(&mut Encoder::new(&mut buf)).unwrap();
  buf.to_base64(base64::STANDARD)
}

