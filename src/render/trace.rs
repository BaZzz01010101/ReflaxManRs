use anyhow::Result;

use super::Material;
use super::math::Vector3;

pub trait Trace {
  fn trace<'a>(
    &'a self,
    origin: &Vector3,
    ray: &Vector3,
    out_drop: Option<&mut Vector3>,
    out_norm: Option<&mut Vector3>,
    out_reflected_ray: Option<&mut Vector3>,
    out_distance: Option<&mut f32>,
    out_drop_material: Option<&mut Material>,
  ) -> Result<bool>;
}