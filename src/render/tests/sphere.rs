#[cfg(test)]

use super::math::Vector3;

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
  let m = material.clone();
  let sphere_center = Vector3::new(0.0, 0.0, 0.0);
  let sphere = Sphere::new(sphere_center, 1.0, material.clone());

  let mut out_drop = Vector3::default();
  let mut out_norm = Vector3::default();
  let mut out_reflected_ray = Vector3::default();
  let mut out_distance: f32 = 0.0;
  let mut out_drop_material: &Material = &m;

  let trace_origin = Vector3::new(0.0, 0.0, 3.0);
  let trace_ray = Vector3::new(0.0, 0.0, -1.0);

  sphere.trace(&trace_origin, &trace_ray, &mut out_drop, &mut out_norm, &mut out_reflected_ray, &mut out_distance, &mut out_drop_material);
  assert_eq!(out_drop, Vector3::new(0.0, 0.0, 1.0), "drop point");
  assert_eq!(out_norm.normalized(), Vector3::new(0.0, 0.0, 1.0), "drop point normal");
  assert_eq!(out_reflected_ray.normalized(), Vector3::new(0.0, 0.0, 1.0), "reflected ray");
  assert_eq!(out_distance, 2.0, "distance");
  assert_eq!(out_drop_material, &material, "material");
}
