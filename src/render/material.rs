use super::Color;

#[derive(Debug, Clone)]
pub enum Kind {
  Metal,
  Dielectric,
}

#[derive(Debug, Clone)]
pub struct Material {
  pub kind: Kind,
  pub color: Color,
  pub reflectivity: f32,
  pub transparency: f32,
}

impl Material {
  pub fn new(kind: Kind, color: Color, reflectivity: f32, transparency: f32) -> Material {
    Material {
      kind,
      color,
      reflectivity,
      transparency
    }
  }
}