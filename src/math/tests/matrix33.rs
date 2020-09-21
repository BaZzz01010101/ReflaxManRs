#[cfg(test)]
use super::{Vector3, Matrix33};

#[test]
fn iter() {
  let m = Matrix33::from([1.0, 3.0, 1.5, 2.1, 5.0, 11.0, 5.2, 6.1, 9.9]);
  let a: [f32; 9] = [1.0, 3.0, 1.5, 2.1, 5.0, 11.0, 5.2, 6.1, 9.9];

  for (m_el, a_el) in m.iter().zip(a.iter()) {
    assert_eq!(m_el, a_el);
  }
}

#[test]
fn index() {
  let m = Matrix33::from([1.0, 3.0, 1.5, 2.1, 5.0, 11.0, 5.2, 6.1, 9.9]);
  let a: [[f32; 3]; 3] = [[1.0, 3.0, 1.5], [2.1, 5.0, 11.0], [5.2, 6.1, 9.9]];

  for i in 0..3 {
    for j in 0..3 {
      assert_eq!(m[i][j], a[i][j]);
    }
  }
}

#[test]
fn det() {
  let m = Matrix33::from([1.0, 9.0, 7.0, 3.0, 4.0, 6.0, 8.0, 2.0, 5.0]);
  let det = m.det();
  assert_eq!(det, 123.0);
}

#[test]
fn invert() {
  let mut m = Matrix33::from([1.0, -9.0, 7.0, 3.0, -4.0, 6.0, -8.0, 2.0, 5.0]);
  m.invert();
  assert_eq!(m, Matrix33::from([-32.0 / 353.0, 59.0 / 353.0, -26.0 / 353.0, -63.0 / 353.0, 61.0 / 353.0, 15.0 / 353.0, -26.0 / 353.0, 70.0 / 353.0, 23.0 / 353.0]));
}

#[test]
fn transpose() {
  let mut m = Matrix33::from([1.0, 9.0, 7.0, 3.0, 4.0, 6.0, 8.0, 2.0, 5.0]);
  m.transpose();
  assert_eq!(m, Matrix33::from([1.0, 3.0, 8.0, 9.0, 4.0, 2.0, 7.0, 6.0, 5.0]));
}

#[test]
fn get_col() {
  let m = Matrix33::from([1.0, 3.0, 1.5, 2.1, 5.0, 11.0, 5.2, 6.1, 9.9]);
  assert_eq!(m.get_col(0), Vector3::new(1.0, 2.1, 5.2));
  assert_eq!(m.get_col(1), Vector3::new(3.0, 5., 6.1));
  assert_eq!(m.get_col(2), Vector3::new(1.5, 11.0, 9.9));
}

#[test]
fn set_col() {
  let mut m = Matrix33::from([1.0, 3.0, 1.5, 2.1, 5.0, 11.0, 5.2, 6.1, 9.9]);
  m.set_col(0, &Vector3::new(0.1, 0.2, 0.3));
  assert_eq!(m.get_col(0), Vector3::new(0.1, 0.2, 0.3));
}

#[test]
fn add() {
  let m1 = Matrix33::from([1.0, 3.0, -1.5, 2.1, -5.0, 11.0, 5.2, 6.1, 9.9]);
  let m2 = Matrix33::from([1.0, 3.0, 1.5, 2.1, -5.0, 11.0, -5.2, 6.1, 9.9]);
  let expected = Matrix33::from([2.0, 6.0, 0.0, 4.2, -10.0, 22.0, 0.0, 12.2, 19.8]);
  assert_eq!(&m1 + &m2, expected);
  assert_eq!(m1.clone() + &m2, expected);
  assert_eq!(&m1 + m2.clone(), expected);
  assert_eq!(m1 + m2, expected);
}

#[test]
fn sub() {
  let m1 = Matrix33::from([2.0, 6.0, 0.0, 4.2, -10.0, 22.0, 0.0, 12.2, 19.8]);
  let m2 = Matrix33::from([1.0, 3.0, 1.5, 2.1, -5.0, 11.0, -5.2, 6.1, 9.9]);
  let expected = Matrix33::from([1.0, 3.0, -1.5, 2.1, -5.0, 11.0, 5.2, 6.1, 9.9]);
  assert_eq!(&m1 - &m2, expected);
  assert_eq!(&m1 - m2.clone(), expected);
  assert_eq!(m1.clone() - &m2, expected);
  assert_eq!(m1 - m2, expected);
}

#[test]
fn mul_matrix() {
  let m1 = Matrix33::from([1.0, 3.0, -2.0, 2.0, -5.0, 11.0, 5.0, 6.0, 9.0]);
  let m2 = Matrix33::from([2.0, 6.0, 0.0, 4.0, -10.0, 22.0, 0.0, 12.0, 19.0]);
  let expected = Matrix33::from([14.0, -48.0, 28.0, -16.0, 194.0, 99.0, 34.0, 78.0, 303.0]);
  assert_eq!(&m1 * &m2, expected);
  assert_eq!(m1.clone() * &m2, expected);
  assert_eq!(&m1 * m2.clone(), expected);
  assert_eq!(m1 * m2, expected);
}

#[test]
fn mul_vector() {
  let mut m = Matrix33::from([1.0, 3.0, -2.0, 2.0, -5.0, 11.0, 5.0, 6.0, 9.0]);
  let mut v = Vector3::new(2.0, 6.0, 3.0);
  let expected = Vector3::new(14.0, 7.0, 73.0);
  assert_eq!(&mut m * &v, expected);
  assert_eq!(&mut m * &mut v, expected);
  assert_eq!(&mut m * v.clone(), expected);
  assert_eq!(&m * &v, expected);
  assert_eq!(m.clone() * &v, expected);
  assert_eq!(&m * v.clone(), expected);
  assert_eq!(&m * v.clone(), expected);
  assert_eq!(m * v, expected);
}

#[test]
fn mul_float() {
  let m = Matrix33::from([2.0, 6.0, 0.0, 4.2, -10.0, 22.0, 0.0, 12.2, 19.8]);
  let expected = Matrix33::from([4.0, 12.0, 0.0, 8.4, -20.0, 44.0, 0.0, 24.4, 39.6]);
  assert_eq!(&m * 2.0, expected);
  assert_eq!(2.0 * &m, expected);
  assert_eq!(m.clone() * 2.0, expected);
  assert_eq!(2.0 * m, expected);
}

#[test]
fn div_float() {
  let m = Matrix33::from([2.0, 6.0, 0.0, 4.2, -10.0, 22.0, 0.0, 12.2, 19.8]);
  let expected = Matrix33::from([1.0, 3.0, 0.0, 2.1, -5.0, 11.0, 0.0, 6.1, 9.9]);
  assert_eq!(&m / 2.0, expected);
  //assert_eq!(m / 2.0, expected);
}

#[test]
fn add_assign() {
  let mut m1 = Matrix33::from([1.0, 3.0, -1.5, 2.1, -5.0, 11.0, 5.2, 6.1, 9.9]);
  let m2 = Matrix33::from([1.0, 3.0, 1.5, 2.1, -5.0, 11.0, -5.2, 6.1, 9.9]);
  let expected = Matrix33::from([2.0, 6.0, 0.0, 4.2, -10.0, 22.0, 0.0, 12.2, 19.8]);
  m1 += m2;
  assert_eq!(m1, expected);
}

#[test]
fn sub_assign() {
  let mut m1 = Matrix33::from([2.0, 6.0, 0.0, 4.2, -10.0, 22.0, 0.0, 12.2, 19.8]);
  let m2 = Matrix33::from([1.0, 3.0, 1.5, 2.1, -5.0, 11.0, -5.2, 6.1, 9.9]);
  let expected = Matrix33::from([1.0, 3.0, -1.5, 2.1, -5.0, 11.0, 5.2, 6.1, 9.9]);
  m1 -= m2;
  assert_eq!(m1, expected);
}

#[test]
fn mul_assign() {
  let mut m1 = Matrix33::from([1.0, 3.0, -2.0, 2.0, -5.0, 11.0, 5.0, 6.0, 9.0]);
  let m2 = Matrix33::from([2.0, 6.0, 0.0, 4.0, -10.0, 22.0, 0.0, 12.0, 19.0]);
  let expected = Matrix33::from([14.0, -48.0, 28.0, -16.0, 194.0, 99.0, 34.0, 78.0, 303.0]);
  m1 *= m2;
  assert_eq!(m1, expected);
}

#[test]
fn neg() {
  let m1 = &mut Matrix33::from([1.0, 3.0, -2.0, 2.0, -5.0, 11.0, 5.0, 6.0, 9.0]);
  let expected = Matrix33::from([-1.0, -3.0, 2.0, -2.0, 5.0, -11.0, -5.0, -6.0, -9.0]);
  let m2 = -m1;
  assert_eq!(m2, expected);
}
