use super::clamp;

#[test]
fn clamp_f32() {
  assert_eq!(clamp(1.5, -2.0, 1.0), 1.0);
  assert_eq!(clamp(-3.5, -2.0, 1.0), -2.0);
  assert_eq!(clamp(0.5, -2.0, 1.0), 0.5);
}

#[test]
fn clamp_i32() {
  assert_eq!(clamp(12, -3, 8), 8);
  assert_eq!(clamp(-5, -3, 8), -3);
  assert_eq!(clamp(2, -3, 8), 2);
}
