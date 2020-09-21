use super::math::Vector3;
use super::math::constants::{VERY_SMALL_NUMBER, DELTA};
use super::{Material, Trace};

pub struct Sphere {
  pub center: Vector3,
  pub radius: f32,
  pub sq_radius: f32,
  pub material: Material,
}

impl Sphere {
  pub fn new(center: Vector3, radius: f32, material: Material) -> Sphere {
    Sphere {
      center,
      radius,
      sq_radius: radius * radius,
      material,
    }
  }
}

impl Trace for Sphere {
  fn trace<'a>(
    &'a self,
    origin: &Vector3,
    ray: &Vector3,
    out_drop: &mut Vector3,
    out_norm: &mut Vector3,
    out_reflected_ray: &mut Vector3,
    out_distance: &mut f32,
    out_drop_material: &mut &'a Material,
  ) -> bool
  {
    let vco = origin - &self.center;
    let a = ray.sq_length();
    let b = 2.0 * ray * &vco;
    let c = vco.sq_length() - self.sq_radius;
    let d = b * b - 4.0 * a * c;

    if d >= 0.0 && a > VERY_SMALL_NUMBER
    {
      let t = (-b - d.sqrt()) / (2.0 * a);

      if t > VERY_SMALL_NUMBER
      {
        let full_ray = ray * t;
        let distance = full_ray.length();

        if distance > DELTA
        {
          let drop = origin + &full_ray;
          let norm = &drop - &self.center;

          *out_distance = distance;
          *out_drop = drop;
          *out_norm = norm.clone();
          *out_reflected_ray = full_ray.reflected(&norm);
          *out_drop_material = &self.material;

          return true;
        }
      }
    }

    return false;
  }
}