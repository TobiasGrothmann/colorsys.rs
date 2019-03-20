use super::Hsl;
use crate::converters::{as_rounded_rgb_tuple, rgb_invert, rgb_to_hex, rgb_to_hsl};
use crate::error::ParseError;
use crate::grayscale::{rgb_grayscale, GrayScaleMethod};
use crate::normalize::{normalize_ratio, normalize_rgb, normalize_rgb_unit};
use crate::{from_str, AlphaColor, Color, ColorTuple, ColorTupleA, RgbUnit};

#[derive(Debug, PartialEq)]
pub struct Rgb {
  r: f32,
  g: f32,
  b: f32,
  a: Option<f32>,
}

impl Rgb {
  pub fn from(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::from_tuple(&(r, g, b))
  }

  pub fn grayscale(&self, method: GrayScaleMethod) -> Rgb {
    Rgb::from_tuple(&rgb_grayscale(&self.as_tuple(), method))
  }

  /// Try to parse string as hex color
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// assert_eq!(Rgb::from_hex_str("#e76B2c").unwrap(),Rgb::from(231.0,107.0,44.0));
  /// assert_eq!(Rgb::from_hex_str("fc0").unwrap(),Rgb::from_tuple(&(255.0,204.0,0.0)));
  /// assert!(Rgb::from_hex_str("cyan").is_err());
  /// ```
  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    match from_str::hex(s) {
      Ok(rgb_tuple) => Ok(Rgb::from_tuple(&rgb_tuple)),
      Err(err) => Err(err),
    }
  }
  /// Returns hexadecimal color string like in css. In lowercase with no reductions
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// let rgb1 = Rgb::from_tuple(&(231.0,107.0,44.0));
  /// assert_eq!(rgb1.to_css_hex_string(),"#e76b2c");
  ///
  /// let rgb2 = Rgb::from_hex_str("#0C7").unwrap();
  /// assert_eq!(rgb2.to_css_hex_string(),"#00cc77");
  /// ```
  pub fn to_css_hex_string(&self) -> String {
    let (r, g, b) = rgb_to_hex(&self.as_tuple());
    format!("#{}{}{}", r, g, b)
  }

  pub fn get_red(&self) -> f32 {
    self.r
  }
  pub fn get_green(&self) -> f32 {
    self.g
  }
  pub fn get_blue(&self) -> f32 {
    self.b
  }
  pub fn set_red(&mut self, val: f32) {
    self.r = normalize_rgb_unit(val);
  }
  pub fn set_green(&mut self, val: f32) {
    self.g = normalize_rgb_unit(val);
  }
  pub fn set_blue(&mut self, val: f32) {
    self.b = normalize_rgb_unit(val);
  }
}

impl std::str::FromStr for Rgb {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Rgb, ParseError> {
    match from_str::rgb(s) {
      Ok(rgb_tuple) => Ok(Rgb::from_tuple(&rgb_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl Color for Rgb {
  type Tuple = ColorTuple;
  type TupleA = ColorTupleA;

  fn new() -> Rgb {
    Rgb { r: 0.0, g: 0.0, b: 0.0, a: None }
  }

  fn to_rgb(&self) -> Rgb {
    *self
  }
  fn to_hsl(&self) -> Hsl {
    Hsl::from_tuple(&rgb_to_hsl(&self.as_tuple()))
  }
  /// Returns css string
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// let rgb = Rgb::from_tuple(&(225.0,101.7, 21.0));
  /// assert_eq!(rgb.to_css_string(), "rgb(225,102,21)");
  /// ```
  fn to_css_string(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.as_tuple());
    format!("rgb({},{},{})", r, g, b)
  }

  fn from_tuple(t: &ColorTuple) -> Rgb {
    let (r, g, b) = normalize_rgb(&t);
    Rgb { r, g, b, a: None }
  }
  fn from_tuple_with_alpha(t: &ColorTupleA) -> Rgb {
    let (r, g, b) = normalize_rgb(&(t.0, t.1, t.2));
    Rgb { r, g, b, a: Some(normalize_ratio(t.3)) }
  }
  fn as_tuple(&self) -> ColorTuple {
    (self.r, self.g, self.b)
  }

  fn lighten(&self, amt: f32) -> Rgb {
    self.to_hsl().lighten(amt).to_rgb()
  }
  fn saturate(&self, amt: f32) -> Rgb {
    self.to_hsl().saturate(amt).to_rgb()
  }
  fn adjust_hue(&self, amt: f32) -> Rgb {
    self.to_hsl().adjust_hue(amt).to_rgb()
  }
  fn adjust_color(&mut self, name: RgbUnit, val: f32) -> Rgb {
    let (r, g, b) = self.as_tuple();
    match name {
      RgbUnit::Red => self.set_red(r + val),
      RgbUnit::Green => self.set_green(g + val),
      RgbUnit::Blue => self.set_blue(b + val),
    }
  }

  fn invert(&self) -> Rgb {
    Rgb::from_tuple(&rgb_invert(&self.as_tuple()))
  }
}

impl AlphaColor for Rgb {
  fn get_alpha(&self) -> f32 {
    self.a.unwrap_or(1.0)
  }
  fn set_alpha(&self, a: f32) -> Rgb {
    Rgb { r: self.r, g: self.g, b: self.b, a: Some(normalize_ratio(a)) }
  }
  fn opacify(&self, a: f32) -> Rgb {
    self.set_alpha(self.get_alpha() + a)
  }
}