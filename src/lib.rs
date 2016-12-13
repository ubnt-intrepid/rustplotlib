extern crate rustc_serialize;
extern crate rmp_serialize;
// extern crate cpython;

mod axes2d;
mod encode;
mod figure;
mod scatter;

// re-exports
pub mod backend;
pub use axes2d::Axes2D;
pub use backend::Backend;
pub use figure::Figure;
pub use scatter::Scatter;
