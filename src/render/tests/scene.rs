use std::io::Cursor;

use super::{
  Color,
  Material,
  MaterialKind,
  Scene,
  Skybox,
};
use super::math::ApproxEq;
use super::math::constants::DELTA;
use super::math::Vector3;
use super::Texture;

const SKYBOX_24_BPP: &[u8] = include_bytes!("res/skybox_32x24_24_bpp.tga");

#[test]
fn trace_plastic_triangle() {
  let skybox_texture_stream = Cursor::new(SKYBOX_24_BPP);
  let skybox_texture = Texture::from_tga(skybox_texture_stream).unwrap();
  let skybox = Skybox::new(skybox_texture);
  let mut scene = Scene::new(skybox, Color::new(1.0, 1.0, 1.0), 1.0);
  scene.add_spot_light(Vector3::new(100.0, 100.0, 100.0), 10.0, Color::new(1.0, 1.0, 1.0), 1.0);
  let material = Material::new(MaterialKind::Dielectric, Color::new(1.0, 1.0, 1.0), 1.0, 0.0);

  scene.add_triangle([
    &Vector3::new(10.0, 0.0, 0.0),
    &Vector3::new(0.0, 10.0, 0.0),
    &Vector3::new(0.0, 0.0, 10.0),
  ], material, None);

  let trace_origin = Vector3::new(30.0, 30.0, 30.0);
  let trace_ray = Vector3::new(-1.0, -1.0, -1.0);
  let color = scene.trace(&trace_origin, &trace_ray, 10).unwrap();
  assert_eq!(color, Color::new(1.0, 1.0, 1.0), "Hit color of triangle");
}

#[test]
fn trace_metal_ball() {
  let skybox_texture_stream = Cursor::new(SKYBOX_24_BPP);
  let skybox_texture = Texture::from_tga(skybox_texture_stream).unwrap();
  let skybox = Skybox::new(skybox_texture);
  let mut scene = Scene::new(skybox, Color::new(1.0, 1.0, 1.0), 1.0);
  //scene.add_spot_light(Vector3::new(100.0, 100.0, 100.0), 10.0, Color::new(1.0, 1.0, 1.0), 1.0);
  let material = Material::new(MaterialKind::Metal, Color::new(1.0, 1.0, 1.0), 1.0, 0.0);
  scene.add_sphere(Vector3::new(0.0, 0.0, 0.0), 1.0, material);

  let trace_origin = Vector3::new(10.0, 0.0, 0.0);
  let trace_ray = Vector3::new(-1.0, 0.0, 0.0);
  let color = scene.trace(&trace_origin, &trace_ray, 10).unwrap();
  let expected = Color::new(0.2, 1.0, 1.0);
  assert!(color.approx_eq(&expected, DELTA), "Hit color of skybox\n left: {}\n right: {}", color, expected);
}
