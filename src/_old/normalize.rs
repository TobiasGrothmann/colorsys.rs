use super::ColorTuple;
use std::f64::EPSILON;

struct NormalizeArg<T> {
  pub val: T,
  pub max: T,
  pub min: T,
}

fn normalize<T: std::cmp::PartialOrd>(arg: NormalizeArg<T>) -> T {
  let NormalizeArg { val, max, min } = arg;
  if val < min {
    return min;
  }
  if val > max {
    return max;
  }
  val
}

pub fn normalize_ratio(r: f64) -> f64 {
  normalize(NormalizeArg { val: r, max: 1.0, min: 0.0 })
}

pub fn normalize_percent(r: f64) -> f64 {
  normalize(NormalizeArg { val: r, max: 100.0, min: 0.0 })
}

pub fn normalize_rgb_unit(u: f64) -> f64 {
  normalize(NormalizeArg { val: u, max: 255.0, min: 0.0 })
}

pub fn normalize_hue(h: f64) -> f64 {
  let h = normalize(NormalizeArg { val: h, max: 360.0, min: 0.0 });
  if (h - 360.0).abs() < EPSILON {
    0.0
  } else {
    h
  }
}

pub fn normalize_hsl(hsl_tuple: &ColorTuple) -> ColorTuple {
  let (h, s, l) = hsl_tuple;
  (normalize_hue(*h), normalize_percent(*s), normalize_percent(*l))
}

pub fn normalize_rgb(rgb_tuple: &ColorTuple) -> ColorTuple {
  let (r, g, b) = rgb_tuple;
  (normalize_rgb_unit(*r), normalize_rgb_unit(*g), normalize_rgb_unit(*b))
}

pub fn bound_ratio(r: f64) -> f64 {
  let mut n = r;
  loop {
    let less = n < 0.0;
    let bigger = n > 1.0;
    if !less && !bigger {
      break n;
    }
    if less {
      n += 1.0;
    } else {
      n -= 1.0;
    }
  }
}
