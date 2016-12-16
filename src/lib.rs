#[cfg(feature = "native")]
extern crate cpython;

mod axes2d;
mod figure;

// re-exports
pub mod backend;
pub use axes2d::{Axes2D, PlotData, Scatter, Line2D, FillBetween};
pub use backend::Backend;
pub use figure::Figure;
