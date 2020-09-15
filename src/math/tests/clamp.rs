#[cfg(test)]
use super::super::clamp::clamp;

#[test]
fn clamp_f32() {
  assert_eq!(clamp(1.5, 0.0, 1.0), 1.0);
}

#[test]
fn clamp_i32() {
  assert_eq!(clamp(2, 0, 1), 1);
}
