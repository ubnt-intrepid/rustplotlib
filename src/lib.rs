extern crate rustc_serialize;
extern crate rmp_serialize;
extern crate cpython;

// re-exports
pub mod axes2d;
pub mod backend;
pub mod figure;

pub use axes2d::{Axes2D, Scatter};
pub use backend::Backend;
pub use figure::Figure;
