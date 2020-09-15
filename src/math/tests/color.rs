#[cfg(test)]
use super::super::Color;

#[test]
fn add() {
  let v1 = Color::from(1.0, -2.0, 3.0);
  let v2 = Color::from(3.0, 2.0, -1.0);
  assert!(v1 + v2 == Color::from(4.0, 0.0, 2.0));
}

#[test]
fn sub() {
  let v1 = Color::from(1.0, -2.0, 3.0);
  let v2 = Color::from(3.0, 2.0, -1.0);
  assert!(&v1 - &v2 == Color::from(-2.0, -4.0, 4.0));
  assert!(&v2 - &v1 == Color::from(2.0, 4.0, -4.0));
}

#[test]
fn mul() {
  let v = Color::from(1.0, -2.0, 3.0);
  assert!(&v * 2.0 == Color::from(2.0, -4.0, 6.0));
  assert!(&v * -2.0 == Color::from(-2.0, 4.0, -6.0));
}

#[test]
fn div() {
  let v = Color::from(1.0, -2.0, 3.0);
  assert!(&v / 2.0 == Color::from(0.5, -1.0, 1.5));
}

#[test]
fn rem() {
  let v1 = Color::from(1.0, -2.0, 3.0);
  let v2 = Color::from(-4.0, 5.0, -6.0);
  assert!(&v1 % &v2 == Color::from(-3.0, -6.0, -3.0));
  assert!(&v2 % &v1 == Color::from(3.0, 6.0, 3.0));
}

#[test]
fn add_assign() {
  let mut v1 = Color::from(1.0, 2.0, 3.0);
  v1 += Color::from(3.0, 2.0, 1.0);
  assert!(v1 == Color::from(4.0, 4.0, 4.0));
}

#[test]
fn sub_assign() {
  let mut v1 = Color::from(1.0, 2.0, 3.0);
  v1 -= Color::from(3.0, 2.0, 1.0);
  assert!(v1 == Color::from(-2.0, 0.0, 2.0));
}

#[test]
fn mul_assign() {
  let mut v1 = Color::from(1.0, -2.0, 3.0);
  v1 *= 2.0;
  assert!(v1 == Color::from(2.0, -4.0, 6.0));
}

#[test]
fn div_assign() {
  let mut v1 = Color::from(1.0, -2.0, 3.0);
  v1 /= 2.0;
  assert!(v1 == Color::from(0.5, -1.0, 1.5));
}
