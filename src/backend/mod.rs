mod mpl;
#[cfg(feature = "native")]
mod mpl_native;

use std::io;

pub use self::mpl::Matplotlib;
#[cfg(feature = "native")]
pub use self::mpl_native::MatplotlibNative;


pub trait Backend {
  fn figure(&mut self) -> io::Result<&mut Self>;
  fn subplot(&mut self, i: i32, j: i32, k: i32) -> io::Result<&mut Self>;
  fn grid(&mut self, grid: bool) -> io::Result<&mut Self>;
  fn legend(&mut self, loc: &str) -> io::Result<&mut Self>;
  fn xlim(&mut self, xlim: &(f64, f64)) -> io::Result<&mut Self>;
  fn ylim(&mut self, ylim: &(f64, f64)) -> io::Result<&mut Self>;
  fn scatter(&mut self,
             xdata: &[f64],
             ydata: &[f64],
             label: &Option<String>,
             color: &Option<String>,
             marker: &Option<String>)
             -> io::Result<&mut Self>;
  fn plot(&mut self,
          xdata: &[f64],
          ydata: &[f64],
          label: &Option<String>,
          color: &Option<String>,
          marker: &Option<String>,
          linestyle: &Option<String>,
          linewidth: &Option<f64>)
          -> io::Result<&mut Self>;
  fn set_style(&mut self, stylename: &str) -> io::Result<&mut Self>;
  fn savefig(&mut self, filename: &str) -> io::Result<&mut Self>;
  fn show(&mut self) -> io::Result<&mut Self>;
}
