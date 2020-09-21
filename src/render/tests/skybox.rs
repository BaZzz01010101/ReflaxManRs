#[cfg(test)]
use std::io::Cursor;

use crate::math::Vector3;
use super::super::{Color, Texture, Skybox};

const SKYBOX_24_BPP: &[u8] = include_bytes!("res/skybox_32x24_24_bpp.tga");

#[test]
fn trace_left() {
  let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  let texture = Texture::from_tga(stream).unwrap();
  let skybox = Skybox::from_texture(texture);
  let left_color = skybox.trace(&Vector3::new(-2.0, 0.0, 0.0)).unwrap();
  assert_eq!(left_color, Color::new(1.0, 1.0, 0.0));
}


#[test]
fn trace_front() {
  let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  let texture = Texture::from_tga(stream).unwrap();
  let skybox = Skybox::from_texture(texture);
  let front_color = skybox.trace(&Vector3::new(0.0, 0.0, 2.0)).unwrap();
  assert_eq!(front_color, Color::new(0.0, 1.0, 0.0));
}

#[test]
fn trace_right() {
  let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  let texture = Texture::from_tga(stream).unwrap();
  let skybox = Skybox::from_texture(texture);
  let right_color = skybox.trace(&Vector3::new(2.0, 0.0, 0.0)).unwrap();
  assert_eq!(right_color, Color::new(0.0, 1.0, 1.0));
}

#[test]
fn trace_back() {
  let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  let texture = Texture::from_tga(stream).unwrap();
  let skybox = Skybox::from_texture(texture);
  let back_color = skybox.trace(&Vector3::new(0.0, 0.0, -2.0)).unwrap();
  assert_eq!(back_color, Color::new(1.0, 0.0, 1.0));
}

#[test]
fn trace_top() {
  let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  let texture = Texture::from_tga(stream).unwrap();
  let skybox = Skybox::from_texture(texture);
  let top_color = skybox.trace(&Vector3::new(0.0, 2.0, 0.0)).unwrap();
  assert_eq!(top_color, Color::new(1.0, 0.0, 0.0));
}

#[test]
fn trace_bottom() {
  let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  let texture = Texture::from_tga(stream).unwrap();
  let skybox = Skybox::from_texture(texture);
  let bottom_color = skybox.trace(&Vector3::new(0.0, -2.0, 0.0)).unwrap();
  assert_eq!(bottom_color, Color::new(0.0, 0.0, 1.0));
}

#[test]
fn trace_front_left_interpolated () {
  // let stream = Cursor::new(SKYBOX_24_BPP.to_vec());
  // let texture = Texture::from_tga(stream).unwrap();
  // //println!("{}", texture.get_texel_color(0.234375, 0.5).unwrap());
  // let skybox = Skybox::from_texture(texture);
  // let front_left_color = skybox.trace(&Vector3::new(-1.0, 0.0, 0.8192)).unwrap();
  // assert_eq!(front_left_color, Color::new(0.0, 0.0, 0.0));
}
