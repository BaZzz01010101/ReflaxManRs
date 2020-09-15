#[cfg(test)]
use super::super::Color;

#[test]
fn add() {
  let c1 = Color::from(1.0, -2.0, 3.0);
  let c2 = Color::from(3.0, 2.0, -1.0);
  assert_eq!(c1 + c2, Color::from(4.0, 0.0, 2.0));
}

#[test]
fn sub() {
  let c1 = Color::from(1.0, -2.0, 3.0);
  let c2 = Color::from(3.0, 2.0, -1.0);
  assert_eq!(&c1 - &c2, Color::from(-2.0, -4.0, 4.0));
  assert_eq!(&c2 - &c1, Color::from(2.0, 4.0, -4.0));
}

#[test]
fn mul() {
  let c = Color::from(1.0, -2.0, 3.0);
  assert_eq!(&c * 2.0, Color::from(2.0, -4.0, 6.0));
  assert_eq!(&c * -2.0, Color::from(-2.0, 4.0, -6.0));
}

#[test]
fn mul_color() {
  let c1 = Color::from(0.5, 0.2, 0.5);
  let c2 = Color::from(0.8, 0.5, 1.0);
  assert_eq!(&c1 * &c2, Color::from(0.4, 0.1, 0.5));
}

#[test]
fn dic() {
  let c = Color::from(1.0, -2.0, 3.0);
  assert_eq!(&c / 2.0, Color::from(0.5, -1.0, 1.5));
}

#[test]
fn add_assign() {
  let mut c1 = Color::from(1.0, 2.0, 3.0);
  c1 += Color::from(3.0, 2.0, 1.0);
  assert_eq!(c1, Color::from(4.0, 4.0, 4.0));
}

#[test]
fn sub_assign() {
  let mut c1 = Color::from(1.0, 2.0, 3.0);
  c1 -= Color::from(3.0, 2.0, 1.0);
  assert_eq!(c1, Color::from(-2.0, 0.0, 2.0));
}

#[test]
fn mul_assign() {
  let mut c1 = Color::from(1.0, -2.0, 3.0);
  c1 *= 2.0;
  assert_eq!(c1, Color::from(2.0, -4.0, 6.0));
}

#[test]
fn dic_assign() {
  let mut c1 = Color::from(1.0, -2.0, 3.0);
  c1 /= 2.0;
  assert_eq!(c1, Color::from(0.5, -1.0, 1.5));
}

#[test]
fn clamp() {
  let mut c = Color::from(1.001, -2.0, 0.75);
  c.clamp();
  assert_eq!(c, Color::from(1.0, 0.0, 0.75));
}
