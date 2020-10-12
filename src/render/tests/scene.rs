#[cfg(test)]
use std::f32::EPSILON;
use std::io::Cursor;
use std::rc::Rc;

use super::math::ApproxEq;
use super::math::Vector3;
use super::Texture;

use super::{
  Color,
  MaterialKind,
  Material,
  Sphere,
  Skybox,
  Scene,
  Trace,
};

const SKYBOX_24_BPP: &[u8] = include_bytes!("res/skybox_32x24_24_bpp.tga");

#[test]
fn trace() {
  let color = Color::new(1.0, 1.0, 1.0);
  let material = Material::new(MaterialKind::Dielectric, color, 0.5, 0.0);
  let sphere_center = Vector3::new(0.0, 0.0, 0.0);
  let skybox_texture_stream = Cursor::new(SKYBOX_24_BPP);
  let skybox_texture = Texture::from_tga(skybox_texture_stream).unwrap();
  let skybox = Skybox::new(skybox_texture);
  let mut scene = Scene::new(skybox, Color::new(1.0, 1.0, 1.0), 0.0);
  scene.add_spot_light(Vector3::new(100.0, 100.0, 100.0), 10.0, Color::new(1.0, 1.0, 1.0), 10.0);
  scene.add_sphere(sphere_center, 1.0, material.clone());

  scene.add_triangle([
    &Vector3::new(10.0, 0.0, 0.0),
    &Vector3::new(0.0, 10.0, 0.0),
    &Vector3::new(0.0, 0.0, 10.0),
  ], material, None);

  let trace_origin = Vector3::new(30.0, 30.0, 30.0);
  let trace_ray = Vector3::new(-1.0, -1.0, -1.0);
  let color = scene.trace(&trace_origin, &trace_ray, 10).unwrap();
  assert_eq!(color, Color::new(1.0, 1.0, 1.0), "Hit color of triangle");

  let trace_ray = Vector3::new(1.0, 1.0, 1.0);
  let color = scene.trace(&trace_origin, &trace_ray, 10).unwrap();
  assert_eq!(color, Color::new(0.0, 0.0, 0.0), "Hit color of skybox");

}
