extern crate rustc_serialize;
extern crate rmp_serialize;
#[cfg(feature = "native")]
extern crate cpython;

mod axes2d;
mod plotdata;
mod figure;
mod util;

// re-exports
pub mod backend;
pub use axes2d::Axes2D;
pub use plotdata::{PlotData, Scatter};
pub use backend::Backend;
pub use figure::Figure;
