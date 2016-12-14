extern crate rustc_serialize;
extern crate rmp_serialize;
#[cfg(feature = "native")]
extern crate cpython;

mod axes2d;
mod figure;
mod scatter;
mod util;

// re-exports
pub mod backend;
pub use axes2d::{Axes2D, PlotData};
pub use backend::Backend;
pub use figure::Figure;
pub use scatter::Scatter;
