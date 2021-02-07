use crate::consts::{
  B_BT601_FACTOR, B_REC2100_FACTOR, B_REC709_FACTOR, G_BT601_FACTOR, G_REC2100_FACTOR,
  G_REC709_FACTOR, R_BT601_FACTOR, R_REC2100_FACTOR, R_REC709_FACTOR
};

use super::Rgb;

pub enum GrayScaleMethod {
  Average,
  AverageProminent,
  Luminance,
  Rec709,
  Rec2100,
}

static BT601_FACTORS: [&f64; 3] = [&R_BT601_FACTOR, &G_BT601_FACTOR, &B_BT601_FACTOR];
static REC709_FACTORS: [&f64; 3] = [&R_REC709_FACTOR, &G_REC709_FACTOR, &B_REC709_FACTOR];
static REC2100_FACTORS: [&f64; 3] = [&R_REC2100_FACTOR, &G_REC2100_FACTOR, &B_REC2100_FACTOR];

fn mul(rgb: &mut Rgb, factors: &[&f64; 3] ) {
  let vals = &mut rgb.units.list;
  vals[0].value *= factors[0];
  vals[1].value *= factors[1];
  vals[2].value *= factors[2];
}

fn set(rgb: &mut Rgb, v: f64) {
  let mut vals = &mut rgb.units.list;
  vals[0].value = v;
  vals[1].value = v;
  vals[2].value = v;
}

fn rgb_to_grayscale_avg(rgb: &mut Rgb) {
  let u = &mut rgb.units;
  let y = (u[0] + u[1] + u[2]) / 3.0;
  set(rgb, y)
}

fn rgb_to_grayscale_avg_prom(rgb: &mut Rgb) {
  let max = rgb.units.max();
  let min = rgb.units.min();
  let y = (max.0 + min.0) / 2.0;
  set(rgb, y);
}

pub fn rgb_grayscale(rgb: &mut Rgb, method: GrayScaleMethod) {
  match method {
    GrayScaleMethod::Average => rgb_to_grayscale_avg(rgb),
    GrayScaleMethod::AverageProminent => rgb_to_grayscale_avg_prom(rgb),
    GrayScaleMethod::Luminance => mul(rgb, &BT601_FACTORS),
    GrayScaleMethod::Rec709 => mul(rgb, &REC709_FACTORS),
    GrayScaleMethod::Rec2100 => mul(rgb, &REC2100_FACTORS),
  }
}
