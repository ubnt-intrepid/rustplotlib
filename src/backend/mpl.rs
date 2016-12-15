use std::io::{self, Write};
use std::process::{Command, Child, Stdio};
use backend::Backend;

/// Represents an instance of Python process which executes operations.
pub struct Matplotlib {
  child: Child,
}

impl Matplotlib {
  /// create an instance of Matplotlib backend.
  pub fn new() -> io::Result<Matplotlib> {
    let child = Command::new("python").arg("-")
      .stdin(Stdio::piped())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()?;

    let mut mpl = Matplotlib { child: child };
    mpl.exec("import matplotlib.pyplot as plt")?;
    Ok(mpl)
  }

  /// wait until all operations are finished.
  pub fn wait(&mut self) -> io::Result<()> {
    self.child.wait().and(Ok(()))
  }

  /// execute a string as Python script.
  pub fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self> {
    {
      let ref mut stdin = self.child.stdin.as_mut().unwrap();
      stdin.write_all(script.as_ref().as_bytes())?;
      stdin.write_all(b"\n")?;
    }
    Ok(self)
  }
}

impl Backend for Matplotlib {
  fn figure(&mut self) -> io::Result<&mut Self> {
    self.exec("plt.figure()")
  }

  fn subplot(&mut self, i: i32, j: i32, k: i32) -> io::Result<&mut Self> {
    self.exec(format!("plt.subplot({}, {}, {})", i, j, k))
  }

  fn grid(&mut self, grid: bool) -> io::Result<&mut Self> {
    self.exec(format!("plt.grid({})", if grid { "True" } else { "False" }))
  }

  fn legend(&mut self, loc: &str) -> io::Result<&mut Self> {
    self.exec(format!("plt.legend(loc='{}')", loc))
  }

  fn xlim(&mut self, xlim: &(f64, f64)) -> io::Result<&mut Self> {
    self.exec(format!("plt.xlim(({}, {}))", xlim.0, xlim.1))
  }

  fn ylim(&mut self, ylim: &(f64, f64)) -> io::Result<&mut Self> {
    self.exec(format!("plt.ylim(({}, {}))", ylim.0, ylim.1))
  }

  fn scatter(&mut self,
             xdata: &[f64],
             ydata: &[f64],
             label: &Option<String>,
             color: &Option<String>,
             marker: &Option<String>)
             -> io::Result<&mut Self> {
    let mut code = format!("plt.scatter({}, {}, ", to_pyvec(xdata), to_pyvec(ydata));
    if let &Some(ref label) = label {
      code += &format!("label='{}', ", label);
    }
    if let &Some(ref color) = color {
      code += &format!("color='{}', ", color);
    }
    if let &Some(ref marker) = marker {
      code += &format!("marker='{}', ", marker);
    }
    code += ")";
    self.exec(code)
  }

  fn plot(&mut self,
          xdata: &[f64],
          ydata: &[f64],
          label: &Option<String>,
          color: &Option<String>,
          marker: &Option<String>,
          linestyle: &Option<String>,
          linewidth: &Option<f64>)
          -> io::Result<&mut Self> {
    let mut code = format!("plt.gca().plot({}, {}, ", to_pyvec(xdata), to_pyvec(ydata));
    if let &Some(ref label) = label {
      code += &format!("label='{}', ", label);
    }
    if let &Some(ref color) = color {
      code += &format!("color='{}', ", color);
    }
    if let &Some(ref marker) = marker {
      code += &format!("marker='{}', ", marker);
    }
    if let &Some(ref ls) = linestyle {
      code += &format!("linestyle='{}', ", ls);
    }
    if let &Some(ref lw) = linewidth {
      code += &format!("linewidth='{}', ", lw);
    }
    code += ")";
    self.exec(code)
  }

  fn set_style(&mut self, stylename: &str) -> io::Result<&mut Self> {
    self.exec(format!("plt.style.use('{}')", stylename))
  }

  fn savefig(&mut self, filename: &str) -> io::Result<&mut Self> {
    self.exec(format!("plt.savefig('{}')", filename))
  }

  fn show(&mut self) -> io::Result<&mut Self> {
    self.exec("plt.show()")
  }
}

impl Drop for Matplotlib {
  fn drop(&mut self) {
    let _ = self.child.kill();
  }
}

fn to_pyvec(data: &[f64]) -> String {
  let data: Vec<String> = data.iter().map(|x| format!("{}", x)).collect();
  format!("[{}]", data.join(","))
}