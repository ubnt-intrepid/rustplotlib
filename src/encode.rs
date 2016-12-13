use rmp_serialize;

pub use rmp_serialize::Encoder;
pub type EncodeResult = Result<(), rmp_serialize::encode::Error>;

// to avoid generic parameter, use this crate to encode to msgpack format,
// instead of `rustc_serialize::Encodable`
pub trait Encodable {
  fn encode(&self, s: &mut Encoder) -> EncodeResult;
}
