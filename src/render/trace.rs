use crate::math::Vector3;
use crate::render::Material;

pub trait Trace {
  fn trace<'a>(
    &'a self,
    origin: &Vector3,
    ray: &Vector3,
    out_drop: &mut Vector3,
    out_norm: &mut Vector3,
    out_reflected_ray: &mut Vector3,
    out_distance: &mut f32,
    out_drop_material: &mut &'a Material,
  ) -> bool;
}