#[cfg(test)]
use std::io::Cursor;
use super::super::{Color, Texture};

const TEX_24_BPP: &[u8] = include_bytes!("res/tex_16x16_24_bpp.tga");
const TEX_32_BPP: &[u8] = include_bytes!("res/tex_16x16_32_bpp.tga");

fn validate_texture(texture: Texture) {
  assert_eq!(texture.width, 16);
  assert_eq!(texture.height, 16);
  assert_eq!(texture.color_buffer.len(), 16 * 16 * 3);

  /* blue pixels*/
  assert_eq!(texture.get_pixel_color(0, 0).unwrap(), Color::new(0.0, 0.0, 1.0));
  assert_eq!(texture.get_pixel_color(7, 0).unwrap(), Color::new(0.0, 0.0, 1.0));
  assert_eq!(texture.get_pixel_color(0, 7).unwrap(), Color::new(0.0, 0.0, 1.0));
  assert_eq!(texture.get_pixel_color(7, 7).unwrap(), Color::new(0.0, 0.0, 1.0));

  /* green pixels*/
  assert_eq!(texture.get_pixel_color(8, 0).unwrap(), Color::new(0.0, 1.0, 0.0));
  assert_eq!(texture.get_pixel_color(15, 0).unwrap(), Color::new(0.0, 1.0, 0.0));
  assert_eq!(texture.get_pixel_color(8, 7).unwrap(), Color::new(0.0, 1.0, 0.0));
  assert_eq!(texture.get_pixel_color(15, 7).unwrap(), Color::new(0.0, 1.0, 0.0));

  /* black pixels*/
  assert_eq!(texture.get_pixel_color(0, 8).unwrap(), Color::new(0.0, 0.0, 0.0));
  assert_eq!(texture.get_pixel_color(7, 8).unwrap(), Color::new(0.0, 0.0, 0.0));
  assert_eq!(texture.get_pixel_color(0, 15).unwrap(), Color::new(0.0, 0.0, 0.0));
  assert_eq!(texture.get_pixel_color(7, 15).unwrap(), Color::new(0.0, 0.0, 0.0));

  /* red pixels*/
  assert_eq!(texture.get_pixel_color(8, 8).unwrap(), Color::new(1.0, 0.0, 0.0));
  assert_eq!(texture.get_pixel_color(15, 8).unwrap(), Color::new(1.0, 0.0, 0.0));
  assert_eq!(texture.get_pixel_color(8, 15).unwrap(), Color::new(1.0, 0.0, 0.0));
  assert_eq!(texture.get_pixel_color(15, 15).unwrap(), Color::new(1.0, 0.0, 0.0));

  /* out of bounds */
  assert!(texture.get_pixel_color(16, 0).is_err());
  assert!(texture.get_pixel_color(0, 16).is_err());
  assert!(texture.get_pixel_color(16, 16).is_err());
}

#[test]
fn from_tga_24() {
  let stream = Cursor::new(TEX_24_BPP);
  let texture = Texture::from_tga(stream).unwrap();
  validate_texture(texture);
}

#[test]
fn from_tga_32() {
  let stream = Cursor::new(TEX_32_BPP);
  let texture = Texture::from_tga(stream).unwrap();
  validate_texture(texture);
}

#[test]
fn get_texel_color() {
  let stream = Cursor::new(TEX_24_BPP);
  let texture = Texture::from_tga(stream).unwrap();
  let texel_color = texture.get_texel_color(0.0, 0.0).unwrap();
  assert_eq!(texel_color, Color::new(0.0, 0.0, 1.0));
}

#[test]
fn get_filtered_texel_color() {
  let stream = Cursor::new(TEX_24_BPP);
  let texture = Texture::from_tga(stream).unwrap();
  let x = 0.5 - 0.5 / texture.width as f32;
  let y = 0.5 - 0.5 / texture.height as f32;
  let interpolated_texel_color = texture.get_texel_color(x, y).unwrap();
  assert_eq!(interpolated_texel_color, Color::new(0.25, 0.25, 0.25));
}