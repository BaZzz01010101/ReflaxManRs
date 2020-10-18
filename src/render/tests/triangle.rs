use super::{
  Color,
  Material,
  MaterialKind,
  Trace,
  Triangle,
};
use super::math::{ApproxEq, Vector3};
use super::math::constants::DELTA;

#[test]
fn trace() {
  let color = Color::new(1.0, 1.0, 1.0);
  let material = Material::new(MaterialKind::Metal, color, 1.0, 0.0);
  let v0 = Vector3::new(30.0, 0.0, 0.0);
  let v1 = Vector3::new(0.0, 30.0, 0.0);
  let v2 = Vector3::new(0.0, 0.0, 30.0);
  let triangle = Triangle::new([&v0, &v1, &v2], material.clone());

  let mut out_drop = Vector3::default();
  let mut out_norm = Vector3::default();
  let mut out_reflected_ray = Vector3::default();
  let mut out_distance: f32 = 0.0;
  let mut out_drop_material = material.clone();

  let trace_origin = Vector3::new(0.0, 0.0, 0.0);
  let trace_ray = Vector3::new(1.0, 1.0, 1.0);

  triangle.trace(
    &trace_origin,
    &trace_ray,
    Some(&mut out_drop),
    Some(&mut out_norm),
    Some(&mut out_reflected_ray),
    Some(&mut out_distance),
    Some(&mut out_drop_material),
  ).unwrap();

  let expected = Vector3::new(10.0, 10.0, 10.0);
  assert!(out_drop.approx_eq(&expected, DELTA), "Drop point.\n left: {}\n right: {}", out_drop, expected);

  out_norm.normalize();
  let expected = Vector3::new(0.5773503, 0.5773503, 0.5773503);
  assert!(out_norm.approx_eq(&expected, DELTA), "Drop point normal.\n left: {}\n right: {}", out_norm, expected);

  out_reflected_ray.normalize();
  let expected = Vector3::new(-0.5773503, -0.5773503, -0.5773503);
  assert!(out_reflected_ray.approx_eq(&expected, DELTA), "Reflected ray (normalized).\n left: {}\n right: {}", out_reflected_ray, expected);

  let expected = 17.320509;
  assert!(out_distance.approx_eq(expected, DELTA), "Distance.\n left: {}\n right: {}", out_distance, expected);

  assert_eq!(out_drop_material, material, "Material.");
}
