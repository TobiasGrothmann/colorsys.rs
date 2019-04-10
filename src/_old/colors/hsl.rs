use super::Rgb;
use crate::converters::{as_rounded_hsl_tuple, hsl_to_rgb, invert_hue};
use crate::error::ParseError;
use crate::normalize::{normalize_hsl, normalize_hue, normalize_percent, normalize_ratio};
use crate::{from_str, AlphaColor, Color, ColorTuple, ColorTupleA, RgbUnit};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
  h: f64,
  s: f64,
  l: f64,
  a: Option<f64>,
}

impl Hsl {
  pub fn from(h: f64, s: f64, l: f64) -> Hsl {
    Hsl::from_tuple(&(h, s, l))
  }

  pub fn grayscale(&self) -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: self.l, a: self.a }
  }

  pub fn get_hue(&self) -> f64 {
    self.h
  }
  pub fn get_saturation(&self) -> f64 {
    self.s
  }
  pub fn get_lightness(&self) -> f64 {
    self.l
  }

  pub fn set_hue(&self, val: f64) -> Hsl {
    Hsl { h: normalize_hue(val), s: self.s, l: self.l, a: self.a }
  }
  pub fn set_saturation(&self, val: f64) -> Hsl {
    Hsl { h: self.h, s: normalize_percent(val), l: self.l, a: self.a }
  }
  pub fn set_lightness(&self, val: f64) -> Hsl {
    Hsl { h: self.h, s: self.s, l: normalize_percent(val), a: self.a }
  }
}

impl std::str::FromStr for Hsl {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    match from_str::hsl(s) {
      Ok(hsl_tuple) => Ok(Hsl::from_tuple(&hsl_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl Color for Hsl {
  type Tuple = ColorTuple;
  type TupleA = ColorTupleA;

  fn new() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0, a: None }
  }

  // fn get_red(&self) -> f64 {
  //   self.to_rgb().get_red()
  // }
  // fn get_green(&self) -> f64 {
  //   self.to_rgb().get_green()
  // }
  // fn get_blue(&self) -> f64 {
  //   self.to_rgb().get_blue()
  // }
  // fn set_red(&self, val: f64) -> Hsl {
  //   self.to_rgb().set_red(val).to_hsl()
  // }
  // fn set_green(&self, val: f64) -> Hsl {
  //   self.to_rgb().set_green(val).to_hsl()
  // }
  // fn set_blue(&self, val: f64) -> Hsl {
  //   self.to_rgb().set_blue(val).to_hsl()
  // }

  fn to_rgb(&self) -> Rgb {
    Rgb::from_tuple(&hsl_to_rgb(&self.as_tuple()))
  }
  fn to_hsl(&self) -> Hsl {
    *self
  }
  fn to_css_string(&self) -> String {
    let (h, s, l) = as_rounded_hsl_tuple(&self.as_tuple());
    format!("hsl({},{}%,{}%)", h, s, l)
  }
  fn from_tuple(t: &ColorTuple) -> Hsl {
    let (h, s, l) = normalize_hsl(&t);
    Hsl { h, s, l, a: None }
  }
  fn from_tuple_with_alpha(t: &ColorTupleA) -> Hsl {
    let (h, s, l, a) = *t;
    let (h, s, l) = normalize_hsl(&(h, s, l));
    Hsl { h, s, l, a: Some(normalize_ratio(a)) }
  }
  fn as_tuple(&self) -> ColorTuple {
    (self.h, self.s, self.l)
  }
  fn lighten(&self, val: f64) -> Hsl {
    self.set_lightness(self.l + val)
  }
  fn saturate(&self, val: f64) -> Hsl {
    self.set_saturation(self.s + val)
  }
  fn adjust_hue(&self, hue: f64) -> Hsl {
    self.set_hue(self.h + hue)
  }
  fn adjust_color(&mut self, name: RgbUnit, val: f64) -> Hsl {
    self.to_rgb().adjust_color(name, val).to_hsl()
  }

  fn invert(&self) -> Hsl {
    Hsl { h: invert_hue(self.h), s: self.s, l: self.l, a: self.a }
  }
}

impl AlphaColor for Hsl {
  fn get_alpha(&self) -> f64 {
    self.a.unwrap_or(1.0)
  }
  fn set_alpha(&self, a: f64) -> Hsl {
    Hsl { h: self.h, s: self.s, l: self.l, a: Some(normalize_ratio(a)) }
  }
  fn opacify(&self, a: f64) -> Hsl {
    self.set_alpha(self.get_alpha() + a)
  }
}
