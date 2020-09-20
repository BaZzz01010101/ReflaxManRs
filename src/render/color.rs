use std::ops::{
  Add, AddAssign,
  Sub, SubAssign,
  Mul, MulAssign,
  Div, DivAssign,
};

use std::cmp::{PartialEq};
use std::fmt;
use std::iter::{IntoIterator, FromIterator};

use crate::math::clamp;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b }
  }

  pub fn from_rgb(rgb: &[u8; 3]) -> Color {
    let [r, g, b] = *rgb;

    Color {
      r: r as f32 * 255.999,
      g: g as f32 * 255.999,
      b: b as f32 * 255.999,
    }
  }

  pub fn clamp(&mut self)
  {
    self.r = clamp(self.r, 0.0, 1.0);
    self.g = clamp(self.g, 0.0, 1.0);
    self.b = clamp(self.b, 0.0, 1.0);
  }

  pub fn rgb(&self) -> [u8; 3] {
    [
      (self.r * 255.999) as u8,
      (self.g * 255.999) as u8,
      (self.b * 255.999) as u8,
    ]
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.r, self.g, self.b)
  }
}

impl<'a> FromIterator<&'a f32> for Color {
  fn from_iter<I: IntoIterator<Item=&'a f32>>(iter: I) -> Color {
    iter.into_iter().map(|el| { *el }).collect()
  }
}

impl FromIterator<f32> for Color {
  fn from_iter<I: IntoIterator<Item=f32>>(iter: I) -> Color {
    let mut v = Color::default();
    let mut it = iter.into_iter();
    v.r = it.next().unwrap();
    v.g = it.next().unwrap();
    v.b = it.next().unwrap();

    v
  }
}

impl AsRef<Color> for Color {
  fn as_ref(&self) -> &Color {
    self
  }
}

impl Add<&Color> for &Color {
  type Output = Color;

  fn add(self, other: &Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b,
    }
  }
}

impl Sub<&Color> for &Color {
  type Output = Color;

  fn sub(self, other: &Color) -> Color {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b,
    }
  }
}

impl Mul<&Color> for &Color {
  type Output = Color;

  fn mul(self, other: &Color) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b,
    }
  }
}

impl Mul<&f32> for &Color {
  type Output = Color;

  fn mul(self, other: &f32) -> Color {
    Color {
      r: self.r * other,
      g: self.g * other,
      b: self.b * other,
    }
  }
}

impl Mul<&Color> for &f32 {
  type Output = Color;

  #[inline]
  fn mul(self, other: &Color) -> Color {
    other * self
  }
}

impl Div<&f32> for &Color {
  type Output = Color;

  fn div(self, other: &f32) -> Color {
    Color {
      r: self.r / other,
      g: self.g / other,
      b: self.b / other,
    }
  }
}

impl AddAssign<&Color> for &mut Color {
  #[inline]
  fn add_assign(&mut self, other: &Color) {
    **self = &**self + other;
  }
}

impl SubAssign<&Color> for &mut Color {
  #[inline]
  fn sub_assign(&mut self, other: &Color) {
    **self = &**self - other;
  }
}

impl MulAssign<&Color> for &mut Color {
  #[inline]
  fn mul_assign(&mut self, other: &Color) {
    **self = &**self * other;
  }
}

impl MulAssign<&f32> for &mut Color {
  #[inline]
  fn mul_assign(&mut self, other: &f32) {
    **self = &**self * other;
  }
}

impl DivAssign<&f32> for &mut Color {
  #[inline]
  fn div_assign(&mut self, other: &f32) {
    **self = &**self / other;
  }
}

impl_binop!(Add::add for [Color, Color] => Color);
impl_binop!(Sub::sub for [Color, Color] => Color);
impl_binop!(Mul::mul for [Color, Color] => Color);
impl_binop!(Mul::mul for [Color, f32] => Color);
impl_binop!(Mul::mul for [f32, Color] => Color);
impl_binop!(Div::div for [Color, f32] => Color);

impl_op_assign!(AddAssign::add_assign for [Color, Color]);
impl_op_assign!(SubAssign::sub_assign for [Color, Color]);
impl_op_assign!(MulAssign::mul_assign for [Color, Color]);
impl_op_assign!(MulAssign::mul_assign for [Color, f32]);
impl_op_assign!(DivAssign::div_assign for [Color, f32]);
