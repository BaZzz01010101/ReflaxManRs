use std::ops::{
  Add, AddAssign,
  Sub, SubAssign,
  Mul, MulAssign,
  Div, DivAssign,
  Neg,
};

use std::cmp::{PartialEq};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::iter::{IntoIterator, FromIterator, Flatten};
use std::convert::{From};
use std::fmt;

use super::constants::VERY_SMALL_NUMBER;
use super::Vector3;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix33 {
  el: [[f32; 3]; 3]
}

impl Matrix33 {
  pub fn from_yaw_pitch(yaw: f32, pitch: f32) -> Matrix33 {
    let yaw_sin = yaw.sin();
    let yaw_cos = yaw.cos();
    let pitch_sin = pitch.sin();
    let pitch_cos = pitch.cos();

    let ref front = Vector3::new(yaw_sin * pitch_cos, pitch_sin, yaw_cos * pitch_cos);
    let ref right = Vector3::new(0.0, 1.0, 0.0) % front;
    let ref up = front % right;

    Matrix33::from([&right.normalized(), &up.normalized(), &front.normalized()])
  }

  pub fn from_cols(
    col_0: impl AsRef<Vector3>,
    col_1: impl AsRef<Vector3>,
    col_2: impl AsRef<Vector3>
  ) -> Matrix33
  {
    Matrix33{
      el: [
        [col_0.as_ref().x, col_1.as_ref().x, col_2.as_ref().x],
        [col_0.as_ref().y, col_1.as_ref().y, col_2.as_ref().y],
        [col_0.as_ref().z, col_1.as_ref().z, col_2.as_ref().z],
      ]
    }
  }

  pub fn iter(&self) -> Flatten<Iter<'_, [f32; 3]>> {
    self.el.iter().flatten()
  }

  pub fn iter_mut(&mut self) -> Flatten<IterMut<'_, [f32; 3]>> {
    self.el.iter_mut().flatten()
  }

  pub fn det(&self) -> f32 {
    let m = self;

    m[0][0] * (m[1][1] * m[2][2] - m[2][1] * m[1][2]) +
      m[1][0] * (m[2][1] * m[0][2] - m[0][1] * m[2][2]) +
      m[2][0] * (m[0][1] * m[1][2] - m[0][2] * m[1][1])
  }

  pub fn inverted(&self) -> Matrix33 {
    let d = self.det();
    let m = self;
    assert!(d.abs() > VERY_SMALL_NUMBER);

    Matrix33 {
      el: [
        [
          (m[1][1] * m[2][2] - m[1][2] * m[2][1]) / d,
          (m[0][2] * m[2][1] - m[0][1] * m[2][2]) / d,
          (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / d,
        ],
        [
          (m[1][2] * m[2][0] - m[1][0] * m[2][2]) / d,
          (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / d,
          (m[0][2] * m[1][0] - m[0][0] * m[1][2]) / d,
        ],
        [
          (m[1][0] * m[2][1] - m[1][1] * m[2][0]) / d,
          (m[0][1] * m[2][0] - m[0][0] * m[2][1]) / d,
          (m[0][0] * m[1][1] - m[0][1] * m[1][0]) / d,
        ],
      ]
    }
  }

  pub fn invert(&mut self) {
    let d = self.det();
    assert!(d.abs() > VERY_SMALL_NUMBER);
    *self = self.inverted();
  }

  pub fn transposed(&self) -> Matrix33 {
    let m = self;
    Matrix33 {
      el: [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
      ]
    }
  }

  pub fn transpose(&mut self) {
    *self = self.transposed();
  }

  pub fn get_col(&self, col: usize) -> Vector3 {
    Vector3::new(self[0][col], self[1][col], self[2][col])
  }

  pub fn set_col(&mut self, col: usize, v: &Vector3) {
    self[0][col] = v.x;
    self[1][col] = v.y;
    self[2][col] = v.z;
  }
}

impl fmt::Display for Matrix33 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\n({}, {}, {})\n({}, {}, {})\n({}, {}, {})", self[0][0], self[0][1], self[0][2], self[1][0], self[1][1], self[1][2], self[2][0], self[2][1], self[2][2])
  }
}

impl Index<usize> for Matrix33 {
  type Output = [f32; 3];

  fn index(&self, i: usize) -> &Self::Output {
    &self.el[i]
  }
}

impl IndexMut<usize> for Matrix33 {
  fn index_mut(&mut self, i: usize) -> &mut [f32; 3] {
    &mut self.el[i]
  }
}

impl From<[f32; 9]> for Matrix33 {
  fn from(fa: [f32; 9]) -> Matrix33 {
    Matrix33 {
      el: [
        [fa[0], fa[1], fa[2]],
        [fa[3], fa[4], fa[5]],
        [fa[6], fa[7], fa[8]],
      ]
    }
  }
}

impl From<[&Vector3; 3]> for Matrix33 {
  fn from(va: [&Vector3; 3]) -> Matrix33 {
    Matrix33 {
      el: [
        [va[0].x, va[1].x, va[2].x],
        [va[0].y, va[1].y, va[2].y],
        [va[0].z, va[1].z, va[2].z],
      ],
    }
  }
}

impl<'a> FromIterator<&'a f32> for Matrix33 {
  #[inline]
  fn from_iter<I: IntoIterator<Item=&'a f32>>(iterable: I) -> Matrix33 {
    let mut m = Matrix33::default();
    let mut iter = iterable.into_iter();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = *iter.next().unwrap();
      }
    }

    m
  }
}

impl FromIterator<f32> for Matrix33 {
  fn from_iter<I: IntoIterator<Item=f32>>(iterable: I) -> Matrix33 {
    let mut m = Matrix33::default();
    let mut iter = iterable.into_iter();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = iter.next().unwrap();
      }
    }

    m
  }
}

impl AsRef<Matrix33> for Matrix33 {
  fn as_ref(&self) -> &Matrix33 {
    self
  }
}

impl Add<&Matrix33> for &Matrix33 {
  type Output = Matrix33;

  fn add(self, other: &Matrix33) -> Matrix33 {
    let mut m = Matrix33::default();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = self[i][j] + other[i][j];
      }
    }

    m
  }
}

impl Sub<&Matrix33> for &Matrix33 {
  type Output = Matrix33;

  fn sub(self, other: &Matrix33) -> Matrix33 {
    let mut m = Matrix33::default();
    let a = self;
    let b = other;

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = a[i][j] - b[i][j];
      }
    }

    m
  }
}

impl Mul<&Matrix33> for &Matrix33 {
  type Output = Matrix33;

  fn mul(self, other: &Matrix33) -> Matrix33 {
    let a = self;
    let b = other;
    let mut m = Matrix33::default();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
      }
    }

    m
  }
}

impl Mul<&Vector3> for &Matrix33 {
  type Output = Vector3;

  fn mul(self, other: &Vector3) -> Vector3 {
    let m = self;
    let v = other;

    Vector3 {
      x: v.x * m[0][0] + v.y * m[0][1] + v.z * m[0][2],
      y: v.x * m[1][0] + v.y * m[1][1] + v.z * m[1][2],
      z: v.x * m[2][0] + v.y * m[2][1] + v.z * m[2][2],
    }
  }
}

impl Mul<&f32> for &Matrix33 {
  type Output = Matrix33;

  fn mul(self, other: &f32) -> Matrix33 {
    let mut m = Matrix33::default();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = self[i][j] * other;
      }
    }

    m
  }
}

impl Mul<&Matrix33> for &f32 {
  type Output = Matrix33;

  fn mul(self, other: &Matrix33) -> Matrix33 {
    other * self
  }
}

impl Div<&f32> for &Matrix33 {
  type Output = Matrix33;

  fn div(self, other: &f32) -> Matrix33 {
    let mut m = Matrix33::default();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = self[i][j] / other;
      }
    }

    m
  }
}

impl AddAssign<&Matrix33> for &mut Matrix33 {
  fn add_assign(&mut self, other: &Matrix33) {
    **self = &**self + other;
  }
}

impl SubAssign<&Matrix33> for &mut Matrix33 {
  fn sub_assign(&mut self, other: &Matrix33) {
    **self = &**self - other;
  }
}

impl MulAssign<&Matrix33> for &mut Matrix33 {
  fn mul_assign(&mut self, other: &Matrix33) {
    **self = &**self * other;
  }
}

impl MulAssign<&f32> for &mut Matrix33 {
  fn mul_assign(&mut self, other: &f32) {
    **self = &**self * *other;
  }
}

impl DivAssign<&f32> for &mut Matrix33 {
  fn div_assign(&mut self, other: &f32) {
    **self = &**self / *other;
  }
}

impl Neg for &Matrix33 {
  type Output = Matrix33;

  fn neg(self) -> Matrix33 {
    let mut m = Matrix33::default();

    for i in 0..3 {
      for j in 0..3 {
        m[i][j] = -self[i][j];
      }
    }

    m
  }
}

impl_binop!(Add::add for [Matrix33, Matrix33] => Matrix33);
impl_binop!(Sub::sub for [Matrix33, Matrix33] => Matrix33);
impl_binop!(Mul::mul for [Matrix33, Matrix33] => Matrix33);
impl_binop!(Mul::mul for [Matrix33, Vector3] => Vector3);
impl_binop!(Mul::mul for [Matrix33, f32] => Matrix33);
impl_binop!(Mul::mul for [f32, Matrix33] => Matrix33);
impl_binop!(Div::div for [Matrix33, f32] => Matrix33);

impl_op_assign!(AddAssign::add_assign for [Matrix33, Matrix33]);
impl_op_assign!(SubAssign::sub_assign for [Matrix33, Matrix33]);
impl_op_assign!(MulAssign::mul_assign for [Matrix33, Matrix33]);
impl_op_assign!(MulAssign::mul_assign for [Matrix33, f32]);
impl_op_assign!(DivAssign::div_assign for [Matrix33, f32]);

impl_op!(Neg::neg for Matrix33 => Matrix33);

