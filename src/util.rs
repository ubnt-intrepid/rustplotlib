use rustc_serialize::base64::{self, ToBase64};
use figure::Figure;

pub fn msgpack(fig: &Figure) -> String {
  use rmp_serialize::Encoder;
  use rustc_serialize::Encodable;
  let mut buf = Vec::new();
  fig.encode(&mut Encoder::new(&mut buf)).unwrap();
  buf.to_base64(base64::STANDARD)
}
