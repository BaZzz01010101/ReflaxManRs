#[cfg(test)]

use std::f32::EPSILON;
use std::io::Cursor;

use super::math::ApproxEq;
use super::math::Vector3;
use super::Texture;

use super::{
  Color,
  MaterialKind,
  Material,
  Sphere,
  Trace,
};

#[test]
fn trace() {
  let color = Color::new(1.0, 1.0, 1.0);
  let material = Material::new(MaterialKind::Metal, color, 1.0, 0.0);
  let sphere_center = Vector3::new(0.0, 0.0, 0.0);
  let sphere = Sphere::new(sphere_center, 1.0, material.clone());

  let mut out_drop = Vector3::default();
  let mut out_norm = Vector3::default();
  let mut out_reflected_ray = Vector3::default();
  let mut out_distance: f32 = 0.0;
  let mut out_drop_material = material.clone();

  let trace_origin = Vector3::new(0.0, 0.0, 3.0);
  let trace_ray = Vector3::new(0.0, 0.0, -1.0);

  sphere.trace(
    &trace_origin,
    &trace_ray,
    Some(&mut out_drop),
    Some(&mut out_norm),
    Some(&mut out_reflected_ray),
    Some(&mut out_distance),
    Some(&mut out_drop_material)
  ).unwrap();

  assert_eq!(out_drop, Vector3::new(0.0, 0.0, 1.0), "drop point");
  assert_eq!(out_norm.normalized(), Vector3::new(0.0, 0.0, 1.0), "drop point normal");
  assert_eq!(out_reflected_ray.normalized(), Vector3::new(0.0, 0.0, 1.0), "reflected ray");
  assert!(out_distance.approx_eq(2.0, EPSILON), "distance\n left: {}\n right: {}", out_distance, 2.0);
  assert_eq!(out_drop_material, material, "material");
}
