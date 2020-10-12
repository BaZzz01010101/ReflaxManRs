use std::rc::Rc;
use std::f32::consts::{PI, FRAC_PI_2};

use anyhow::{Result, Error, Context};

use super::math::{Vector3, Matrix33, clamp};
use super::math::constants::{DELTA, VERY_SMALL_NUMBER};
use super::{Trace, Texture, Material};

#[derive(Default, Clone)]
pub struct Triangle {
  // vertexes
  v: [Vector3; 3],
  // vertexes texture u-coords
  tu: [f32; 3],
  // vertexes texture v-coords
  tv: [f32; 3],
  material: Material,
  norm: Vector3,
  texture: Option<Rc<Texture>>,
  ax_transform: Matrix33,
  tuv_transform: Matrix33,
}

impl Triangle {
  pub fn new(vertices: [&Vector3; 3], material: Material) -> Triangle
  {
    let ax = vertices[2] - vertices[0];
    let ay = vertices[1] - vertices[0];
    let norm = (&ay % &ax).normalized();
    let ax_transform = Matrix33::from_cols(ax, ay, -&norm).inverted();
    let mut tuv_transform = ax_transform.clone();

    Triangle {
      v: [
        vertices[0].clone(),
        vertices[1].clone(),
        vertices[2].clone(),
      ],
      tu: [0.0, 0.0, 0.0],
      tv: [0.0, 0.0, 0.0],
      material,
      norm,
      texture: None,
      ax_transform,
      tuv_transform,
    }
  }

  pub fn set_texture(
    &mut self,
    texture: Rc<Texture>,
    texture_u_points: [f32; 3],
    texture_v_points: [f32; 3])
  {
    self.texture = Some(Rc::clone(&texture));

    self.tu = texture_u_points;
    self.tv = texture_v_points;
    let v1 = Vector3::new(self.tu[0], self.tv[0], 0.0);
    let v2 = Vector3::new(self.tu[1], self.tv[1], 0.0);
    let v3 = Vector3::new(self.tu[2], self.tv[2], 0.0);
    self.tuv_transform = Matrix33::from_cols(&v3 - &v1, &v2 - &v1, Vector3::new(0.0, 0.0, -1.0));
  }
}

impl Trace for Triangle {
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
    let transformed_origin = &self.ax_transform * (origin - &self.v[0]);
    let transformed_ray = &self.ax_transform * ray;

    if transformed_ray.z.abs() < VERY_SMALL_NUMBER {
      return Ok(false);
    }

    let t = -transformed_origin.z / transformed_ray.z;

    if t < VERY_SMALL_NUMBER {
      return Ok(false);
    }

    let u = transformed_origin.x + t * transformed_ray.x;
    let v = transformed_origin.y + t * transformed_ray.y;

    if u < 0.0 || v < 0.0 || u + v >= 1.0 {
      return Ok(false);
    }

    let full_ray = ray * t;
    let sq_distance = full_ray.sq_length();

    if sq_distance < DELTA * DELTA {
      return Ok(false);
    }

    if let Some(out_distance) = out_distance {
      *out_distance = sq_distance.sqrt();
    }

    if let Some(out_drop) = out_drop {
      *out_drop = origin + &full_ray;
    }

    if let Some(out_norm) = out_norm {
      *out_norm = self.norm.clone();
    }

    if let Some(out_reflected_ray) = out_reflected_ray {
      *out_reflected_ray = full_ray.reflected(&self.norm);
    }

    if let Some(out_drop_material) = out_drop_material {
      if let Some(texture) = &self.texture {
        let texture_vector = &self.tuv_transform * Vector3::new(u, v, 0.0);
        (*out_drop_material).color = texture.get_texel_color(self.tu[0] + texture_vector.x, self.tv[0] + texture_vector.y)?;
        (*out_drop_material).kind = self.material.kind.clone();
        (*out_drop_material).reflectivity = self.material.reflectivity;
        (*out_drop_material).transparency = self.material.transparency;
      } else {
        *out_drop_material = self.material.clone();
      }
    }

    Ok(true)
  }
}
