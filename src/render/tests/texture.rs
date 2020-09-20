#[cfg(test)]
use std::env;
use std::path::{Path, PathBuf};
use super::super::texture::Texture;

fn get_resource_path() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join(file!()).parent().unwrap().join("res")
}

fn validate_texture(texture: Texture) {
  assert_eq!(texture.width, 16);
  assert_eq!(texture.height, 16);
  assert_eq!(texture.color_buffer.len(), 16 * 16 * 3);

  /* blue pixels*/
  assert_eq!(texture.get_rgb_pixel(0, 0).unwrap(), &[0, 0, 255]);
  assert_eq!(texture.get_rgb_pixel(7, 0).unwrap(), &[0, 0, 255]);
  assert_eq!(texture.get_rgb_pixel(0, 7).unwrap(), &[0, 0, 255]);
  assert_eq!(texture.get_rgb_pixel(7, 7).unwrap(), &[0, 0, 255]);

  /* green pixels*/
  assert_eq!(texture.get_rgb_pixel(8, 0).unwrap(), &[0, 255, 0]);
  assert_eq!(texture.get_rgb_pixel(15, 0).unwrap(), &[0, 255, 0]);
  assert_eq!(texture.get_rgb_pixel(8, 7).unwrap(), &[0, 255, 0]);
  assert_eq!(texture.get_rgb_pixel(15, 7).unwrap(), &[0, 255, 0]);

  /* black pixels*/
  assert_eq!(texture.get_rgb_pixel(0, 8).unwrap(), &[0, 0, 0]);
  assert_eq!(texture.get_rgb_pixel(7, 8).unwrap(), &[0, 0, 0]);
  assert_eq!(texture.get_rgb_pixel(0, 15).unwrap(), &[0, 0, 0]);
  assert_eq!(texture.get_rgb_pixel(7, 15).unwrap(), &[0, 0, 0]);

  /* red pixels*/
  assert_eq!(texture.get_rgb_pixel(8, 8).unwrap(), &[255, 0, 0]);
  assert_eq!(texture.get_rgb_pixel(15, 8).unwrap(), &[255, 0, 0]);
  assert_eq!(texture.get_rgb_pixel(8, 15).unwrap(), &[255, 0, 0]);
  assert_eq!(texture.get_rgb_pixel(15, 15).unwrap(), &[255, 0, 0]);

  /* out of bounds */
  assert!(texture.get_rgb_pixel(16, 0).is_err());
  assert!(texture.get_rgb_pixel(0, 16).is_err());
  assert!(texture.get_rgb_pixel(16, 16).is_err());
}

#[test]
fn load_tga_24_from_file() {
  let texture_path = get_resource_path().join("tex_24_bpp.tga");
  let texture = Texture::load_from_file(&texture_path).unwrap();
  validate_texture(texture);
}

#[test]
fn load_tga_32_from_file() {
  let texture_path = get_resource_path().join("tex_32_bpp.tga");
  let texture = Texture::load_from_file(&texture_path).unwrap();
  validate_texture(texture);
}