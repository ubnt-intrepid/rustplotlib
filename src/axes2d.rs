use std::fmt::Debug;
use encode::{Encoder, Encodable, EncodeResult};


pub trait PlotData: Debug + Encodable {
  fn plot_type(&self) -> &'static str;
}


#[derive(Debug)]
pub struct Axes2D {
  plot_data: Vec<Box<PlotData>>,
  xlabel: Option<String>,
  ylabel: Option<String>,
  grid: bool,
}

impl Axes2D {
  pub fn new() -> Self {
    Axes2D {
      plot_data: Vec::new(),
      xlabel: None,
      ylabel: None,
      grid: false,
    }
  }

  pub fn add<P: 'static + PlotData>(mut self, p: P) -> Self {
    self.plot_data.push(Box::new(p));
    self
  }

  pub fn xlabel(mut self, text: &str) -> Self {
    self.xlabel = Some(text.to_owned());
    self
  }

  pub fn ylabel(mut self, text: &str) -> Self {
    self.ylabel = Some(text.to_owned());
    self
  }

  pub fn grid(mut self, enabled: bool) -> Self {
    self.grid = enabled;
    self
  }
}

impl Encodable for Axes2D {
  fn encode(&self, s: &mut Encoder) -> EncodeResult {
    use rustc_serialize::Encoder;
    s.emit_seq(4, |s| {
      s.emit_seq(self.plot_data.len(), |s| {
          for (i, elem) in self.plot_data.iter().enumerate() {
            s.emit_seq_elt(i, |s| {
                s.emit_tuple(2, |s| {
                  s.emit_tuple_arg(0, |s| s.emit_str(elem.plot_type()))?;
                  s.emit_tuple_arg(1, |s| elem.encode(s))?;
                  Ok(())
                })
              })?;
          }
          Ok(())
        })?;
      s.emit_option(|s| {
          match self.xlabel {
            Some(ref xlabel) => s.emit_option_some(|s| s.emit_str(xlabel)),
            None => s.emit_option_none(),
          }
        })?;
      s.emit_option(|s| {
          match self.ylabel {
            Some(ref ylabel) => s.emit_option_some(|s| s.emit_str(ylabel)),
            None => s.emit_option_none(),
          }
        })?;
      s.emit_bool(self.grid)?;
      Ok(())
    })
  }
}
