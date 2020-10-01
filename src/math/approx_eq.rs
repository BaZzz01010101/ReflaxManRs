pub trait ApproxEq<T> {
  fn approx_eq(self, right: T, delta: f32) -> bool;
}

impl ApproxEq<f32> for f32 {
  fn approx_eq(self, right: f32, delta: f32) -> bool {
    f32::abs(self - right) < delta
  }
}
