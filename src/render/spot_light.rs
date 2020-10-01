use super::math::Vector3;
use super::Color;

pub struct SpotLight {
  pub origin: Vector3,
  pub radius: f32,
  pub color: Color,
  pub power: f32,
}

impl SpotLight {
  pub fn new(origin: Vector3, radius: f32, color: Color, power: f32) -> SpotLight {
    SpotLight {
      origin,
      radius,
      color,
      power,
    }
  }
}