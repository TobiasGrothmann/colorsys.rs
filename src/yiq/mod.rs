use crate::Rgb;

#[derive(Default, Debug)]
pub struct Yiq {
  /// Brightness or luminance
  y: f64,
  i: f64,
  q: f64,
}

impl Yiq {
  pub fn new(y: f64, i: f64, q: f64) -> Self {
    Yiq { y, i, q }
  }
}

impl From<&Yiq> for Rgb {
  fn from(yiq: &Yiq) -> Self {
    yiq_to_rgb(yiq)
  }
}

impl From<&Rgb> for Yiq {
  fn from(rgb: &Rgb) -> Self {
    rgb_to_yiq(rgb)
  }
}


fn yiq_to_rgb(yiq: &Yiq) -> Rgb {
  let Yiq { y, i, q } = yiq;
  let r = y + 0.956 * i + 0.623 * q;
  let g = y - 0.272 * i - 0.648 * q;
  let b = y - 1.105 * i + 1.705 * q;

  Default::default()
}


fn rgb_to_yiq(rgb: &Rgb) -> Yiq {
  let [r, g, b]: [f64; 3] = rgb.as_ratio().into();

  let y = 0.299 * r + 0.587 * g + 0.114 * b;
  let i = 0.596 * r - 0.274 * g - 0.322 * b;
  let q = 0.211 * r - 0.522 * g + 0.311 * b;
  println!(">>> {} {} {}", y, i, q);
  Default::default()
}


#[cfg(test)]
mod test {
  use crate::Rgb;
  use crate::yiq::{rgb_to_yiq, yiq_to_rgb};

  #[test]
  fn yik_to_rgb_test() {
    rgb_to_yiq(&Rgb::from_hex_str("#ffcc00").unwrap());
    rgb_to_yiq(&Rgb::from_hex_str("#00ffcc").unwrap());
  }
}

