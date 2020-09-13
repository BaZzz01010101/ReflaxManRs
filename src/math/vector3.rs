use std::ops::{
  Add, AddAssign,
  Sub, SubAssign,
  Mul, MulAssign,
  Div, DivAssign,
  Rem, RemAssign,
  Neg,
};

use std::cmp::{PartialEq};
use std::fmt;
use std::iter::{IntoIterator, FromIterator};

use super::constants::VERY_SMALL_NUMBER;

#[derive(Debug, Default, Clone)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn from(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
  }

  pub fn length(&self) -> f32 {
    (self.x + self.y + self.z).sqrt()
  }

  pub fn sq_length(&self) -> f32 {
    self.x + self.y + self.z
  }

  pub fn normalized(&self) -> Vector3 {
    let length = self.length();
    assert!(length > VERY_SMALL_NUMBER, "Vector length too small: {}", self);

    self / length
  }

  #[inline]
  pub fn normalize(&mut self) {
    *self = self.normalized();
  }

  pub fn reflected(&self, norm: &Vector3) -> Vector3 {
    let a = norm * norm;
    assert!(a > VERY_SMALL_NUMBER, "Value too small: {}", a);

    2.0 * (self - (self * norm / a) * norm) - self
  }

  #[inline]
  pub fn reflect(&mut self, norm: &Vector3) {
    *self = self.reflected(norm);
  }
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl<'a> FromIterator<&'a f32> for Vector3 {
  fn from_iter<I: IntoIterator<Item=&'a f32>>(iter: I) -> Vector3 {
    iter.into_iter().map(|el|{*el}).collect()
  }
}

impl FromIterator<f32> for Vector3 {
  fn from_iter<I: IntoIterator<Item=f32>>(iter: I) -> Vector3 {
    let mut v = Vector3::default();
    let mut it = iter.into_iter();
    v.x = it.next().unwrap();
    v.y = it.next().unwrap();
    v.z = it.next().unwrap();

    v
  }
}

impl AsRef<Vector3> for Vector3 {
  fn as_ref(&self) -> &Vector3 {
    self
  }
}

impl Add<&Vector3> for &Vector3 {
  type Output = Vector3;

  fn add(self, other: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub<&Vector3> for &Vector3 {
  type Output = Vector3;

  fn sub(self, other: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul<&Vector3> for &Vector3{
  type Output = f32;

  fn mul(self, other: &Vector3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Mul<&f32> for &Vector3 {
  type Output = Vector3;

  fn mul(self, other: &f32) -> Vector3 {
    Vector3 {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

impl Mul<&Vector3> for &f32 {
  type Output = Vector3;

  #[inline]
  fn mul(self, other: &Vector3) -> Vector3 {
    other * self
  }
}

impl Div<&f32> for &Vector3 {
  type Output = Vector3;

  fn div(self, other: &f32) -> Vector3 {
    Vector3 {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other,
    }
  }
}

impl Rem<&Vector3> for &Vector3 {
  type Output = Vector3;

  fn rem(self, other: &Vector3) -> Vector3 {
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
}

impl AddAssign<&Vector3> for &mut Vector3 {
  #[inline]
  fn add_assign(&mut self, other: &Vector3) {
    **self = &**self + other;
  }
}

impl SubAssign<&Vector3> for &mut Vector3 {
  #[inline]
  fn sub_assign(&mut self, other: &Vector3) {
    **self = &**self - other;
  }
}

impl MulAssign<&f32> for &mut Vector3 {
  #[inline]
  fn mul_assign(&mut self, other: &f32) {
    **self = &**self * other;
  }
}

impl DivAssign<&f32> for &mut Vector3 {
  #[inline]
  fn div_assign(&mut self, other: &f32) {
    **self = &**self / other;
  }
}

impl RemAssign<&Vector3> for &mut Vector3 {
  #[inline]
  fn rem_assign(&mut self, other: &Vector3) {
    **self = &**self % other;
  }
}

impl Neg for &Vector3 {
  type Output = Vector3;

  fn neg(self) -> Vector3 {
    Vector3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl PartialEq for Vector3 {
  fn eq(&self, other: &Vector3) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}

impl PartialEq<Vector3> for &mut Vector3 {
  fn eq(&self, other: &Vector3) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}

impl_binop!(Add::add for [Vector3, Vector3] => Vector3);
impl_binop!(Sub::sub for [Vector3, Vector3] => Vector3);
impl_binop!(Rem::rem for [Vector3, Vector3] => Vector3);
impl_binop!(Mul::mul for [Vector3, Vector3] => f32);
impl_binop!(Mul::mul for [Vector3, f32] => Vector3);
impl_binop!(Mul::mul for [f32, Vector3] => Vector3);
impl_binop!(Div::div for [Vector3, f32] => Vector3);

impl_op_assign!(AddAssign::add_assign for [Vector3, Vector3]);
impl_op_assign!(SubAssign::sub_assign for [Vector3, Vector3]);
impl_op_assign!(RemAssign::rem_assign for [Vector3, Vector3]);
impl_op_assign!(MulAssign::mul_assign for [Vector3, f32]);
impl_op_assign!(DivAssign::div_assign for [Vector3, f32]);

impl_op!(Neg::neg for Vector3 => Vector3);
