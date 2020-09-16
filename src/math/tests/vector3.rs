#[cfg(test)]
use super::super::Vector3;

#[test]
fn length() {
  let v = Vector3::new(2.0, -4.0, 4.0);
  assert_eq!(v.length(), 6.0);
}

#[test]
fn sq_length() {
  let v = Vector3::new(2.0, -4.0, 4.0);
  assert_eq!(v.sq_length(), 36.0);
}

#[test]
fn add() {
  let v1 = Vector3::new(1.0, -2.0, 3.0);
  let v2 = Vector3::new(3.0, 2.0, -1.0);
  assert_eq!(v1 + v2, Vector3::new(4.0, 0.0, 2.0));
}

#[test]
fn sub() {
  let v1 = Vector3::new(1.0, -2.0, 3.0);
  let v2 = Vector3::new(3.0, 2.0, -1.0);
  assert_eq!(&v1 - &v2, Vector3::new(-2.0, -4.0, 4.0));
  assert_eq!(&v2 - &v1, Vector3::new(2.0, 4.0, -4.0));
}

#[test]
fn mul() {
  let v = Vector3::new(1.0, -2.0, 3.0);
  assert_eq!(&v * 2.0, Vector3::new(2.0, -4.0, 6.0));
  assert_eq!(&v * -2.0, Vector3::new(-2.0, 4.0, -6.0));
}

#[test]
fn div() {
  let v = Vector3::new(1.0, -2.0, 3.0);
  assert_eq!(&v / 2.0, Vector3::new(0.5, -1.0, 1.5));
}

#[test]
fn rem() {
  let v1 = Vector3::new(1.0, -2.0, 3.0);
  let v2 = Vector3::new(-4.0, 5.0, -6.0);
  assert_eq!(&v1 % &v2, Vector3::new(-3.0, -6.0, -3.0));
  assert_eq!(&v2 % &v1, Vector3::new(3.0, 6.0, 3.0));
}

#[test]
fn add_assign() {
  let mut v1 = Vector3::new(1.0, 2.0, 3.0);
  v1 += Vector3::new(3.0, 2.0, 1.0);
  assert_eq!(v1, Vector3::new(4.0, 4.0, 4.0));
}

#[test]
fn sub_assign() {
  let mut v1 = Vector3::new(1.0, 2.0, 3.0);
  v1 -= Vector3::new(3.0, 2.0, 1.0);
  assert_eq!(v1, Vector3::new(-2.0, 0.0, 2.0));
}

#[test]
fn mul_assign() {
  let mut v1 = Vector3::new(1.0, -2.0, 3.0);
  v1 *= 2.0;
  assert_eq!(v1, Vector3::new(2.0, -4.0, 6.0));
}

#[test]
fn div_assign() {
  let mut v1 = Vector3::new(1.0, -2.0, 3.0);
  v1 /= 2.0;
  assert_eq!(v1, Vector3::new(0.5, -1.0, 1.5));
}

#[test]
fn rem_assign() {
  let mut v1 = Vector3::new(1.0, -2.0, 3.0);
  v1 %= Vector3::new(-4.0, 5.0, -6.0);
  assert_eq!(v1, Vector3::new(-3.0, -6.0, -3.0));
}

#[test]
fn neg() {
  let v1 = Vector3::new(1.0, -2.0, 3.0);
  assert_eq!(-&v1, Vector3::new(-1.0, 2.0, -3.0));
}
