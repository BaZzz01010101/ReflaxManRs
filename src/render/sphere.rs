use anyhow::{Result, Error, Context};

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
    out_drop: Option<&mut Vector3>,
    out_norm: Option<&mut Vector3>,
    out_reflected_ray: Option<&mut Vector3>,
    out_distance: Option<&mut f32>,
    out_drop_material: Option<&mut Material>,
  ) -> Result<bool>
  {
    let vco = origin - &self.center;
    let a = ray.sq_length();
    let b = 2.0 * ray * &vco;
    let c = vco.sq_length() - self.sq_radius;
    let d = b * b - 4.0 * a * c;


    if d < 0.0 {
      return Ok(false);
    }

    if a < VERY_SMALL_NUMBER {
      return Ok(false);
      //return Result::Err(Error::msg("Ray is too short"));
    }

    let t = (-b - d.sqrt()) / (2.0 * a);

    if t < VERY_SMALL_NUMBER {
      return Ok(false);
      //return Result::Err(Error::msg("Invalid tracing conditions"));
    }

    let full_ray = ray * t;
    let distance = full_ray.length();

    if distance < DELTA {
      return Ok(false);
    }

    let drop = origin + &full_ray;
    let norm = &drop - &self.center;

    if let Some(out_distance) = out_distance {
      *out_distance = distance;
    }

    if let Some(out_drop) = out_drop {
      *out_drop = drop;
    }

    if let Some(out_norm) = out_norm {
      *out_norm = norm.clone();
    }

    if let Some(out_reflected_ray) = out_reflected_ray {
      *out_reflected_ray = full_ray.reflected(&norm);
    }

    if let Some(out_drop_material) = out_drop_material {
      *out_drop_material = self.material.clone();
    }

    Ok(true)
  }
}