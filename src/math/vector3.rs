use std::ops::{
  Add, AddAssign,
  Sub, SubAssign,
  Mul, MulAssign,
  Div, DivAssign,
  Rem, RemAssign,
  Neg,
};

use std::marker::Sized;
use std::cmp::{PartialEq};
use std::fmt;

use super::constants::VERY_SMALL_NUMBER;

pub struct Vector3 {
  x: f32,
  y: f32,
  z: f32,
}

impl Vector3 {
  pub fn new() -> Vector3 {
    Vector3 { x: 0., y: 0., z: 0. }
  }

  pub fn from(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
  }

  pub fn length(&self) -> f32 {
    (self.x + self.y + self.z).sqrt()
  }

  pub fn sq_length(&self) -> f32 {
    self.x + self.y + self.z
  }

  pub fn normalize(&mut self) {
    let length = self.length();
    assert!(length > VERY_SMALL_NUMBER, "Vector length too small: {}", self);

    *self /= length;
  }

  pub fn normalized(&self) -> Vector3 {
    let length = self.length();
    assert!(length > VERY_SMALL_NUMBER, "Vector length too small: {}", self);

    self / length
  }

  pub fn reflect(&mut self, norm: &Vector3) {
    let a = norm * norm;
    assert!(a > VERY_SMALL_NUMBER, "Value too small: {}", a);
    *self = 2.0 * (&*self - (&*self * norm / a) * norm) - &*self;
  }

  pub fn reflected(&self, norm: &Vector3) -> Vector3 {
    let a = norm * norm;
    assert!(a > VERY_SMALL_NUMBER, "Value too small: {}", a);

    2.0 * (self - (self * norm / a) * norm) - self
  }
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl AsRef<Vector3> for Vector3 {
  fn as_ref(&self) -> &Vector3 {
    self
  }
}

impl<T> Add<T> for &Vector3 where T: AsRef<Vector3> {
  type Output = Vector3;

  fn add(self, other: T) -> Vector3 {
    Vector3 {
      x: self.x + other.as_ref().x,
      y: self.y + other.as_ref().y,
      z: self.z + other.as_ref().z,
    }
  }
}

impl<T> Sub<T> for &Vector3 where T: AsRef<Vector3> {
  type Output = Vector3;

  fn sub(self, other: T) -> Vector3 {
    Vector3 {
      x: self.x - other.as_ref().x,
      y: self.y - other.as_ref().y,
      z: self.z - other.as_ref().z,
    }
  }
}

impl<T> Mul<T> for &Vector3 where T: AsRef<Vector3> {
  type Output = f32;

  fn mul(self, other: T) -> f32 {
    self.x * other.as_ref().x + self.y * other.as_ref().y + self.z * other.as_ref().z
  }
}

impl Mul<f32> for &Vector3 {
  type Output = Vector3;

  fn mul(self, other: f32) -> Vector3 {
    Vector3 {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;

  #[inline]
  fn mul(self, other: f32) -> Vector3 {
    &self * other
  }
}

impl Mul<&Vector3> for f32 {
  type Output = Vector3;

  #[inline]
  fn mul(self, other: &Vector3) -> Vector3 {
    other * self
  }
}

impl Mul<Vector3> for f32 {
  type Output = Vector3;

  #[inline]
  fn mul(self, other: Vector3) -> Vector3 {
    other * self
  }
}

impl Div<f32> for &Vector3 {
  type Output = Vector3;

  fn div(self, other: f32) -> Vector3 {
    Vector3 {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other,
    }
  }
}

impl Div<f32> for Vector3 {
  type Output = Vector3;

  #[inline]
  fn div(self, other: f32) -> Vector3 {
    &self / other
  }
}

impl<T> Rem<T> for &Vector3 where T: AsRef<Vector3> + Sized {
  type Output = Vector3;

  fn rem(self, other: T) -> Vector3 {
    Vector3 {
      x: self.y * other.as_ref().z - self.z * other.as_ref().y,
      y: self.z * other.as_ref().x - self.x * other.as_ref().z,
      z: self.x * other.as_ref().y - self.y * other.as_ref().x,
    }
  }
}

impl<T> AddAssign<T> for &mut Vector3 where T: AsRef<Vector3> {
  #[inline]
  fn add_assign(&mut self, other: T) {
    **self = &**self + other;
  }
}

impl<T> SubAssign<T> for &mut Vector3 where T: AsRef<Vector3> {
  #[inline]
  fn sub_assign(&mut self, other: T) {
    **self = &**self - other;
  }
}

impl MulAssign<f32> for &mut Vector3 {
  #[inline]
  fn mul_assign(&mut self, other: f32) {
    **self = &**self * other;
  }
}

impl MulAssign<f32> for Vector3 {
  #[inline]
  fn mul_assign(&mut self, other: f32) {
    *self = &*self * other;
  }
}

impl DivAssign<f32> for &mut Vector3 {
  #[inline]
  fn div_assign(&mut self, other: f32) {
    **self = &**self / other;
  }
}

impl DivAssign<f32> for Vector3 {
  #[inline]
  fn div_assign(&mut self, other: f32) {
    *self = &*self / other;
  }
}

impl<T> RemAssign<T> for &mut Vector3 where T: AsRef<Vector3> {
  #[inline]
  fn rem_assign(&mut self, other: T) {
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

impl_binop!(Add::add for Vector3, Vector3);
impl_binop!(Sub::sub for Vector3, Vector3);
impl_binop!(Rem::rem for Vector3, Vector3);
impl_binop!(Mul::mul for Vector3, f32);
impl_binop_assign!(AddAssign::add_assign for Vector3);
impl_binop_assign!(SubAssign::sub_assign for Vector3);
impl_binop_assign!(RemAssign::rem_assign for Vector3);
