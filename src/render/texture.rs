use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader, Seek, SeekFrom};
use std::convert::TryInto;

use anyhow::{Result, Error, Context};
use byteorder::{ReadBytesExt, LittleEndian};

use super::math::clamp;
use super::Color;

#[derive(Default)]
struct TGAFileHeader
{
  ident_size: i8,
  color_map_type: i8,
  image_type: i8,
  _color_map_origin: i16,
  _color_map_length: i16,
  _color_map_bits_per_entry: i8,
  x_offset: i16,
  y_offset: i16,
  x_size: i16,
  y_size: i16,
  bits_per_pixel: i8,
  image_descriptor: i8,
}

pub struct Texture {
  pub width: u32,
  pub height: u32,
  pub color_buffer: Vec<u8>,
}

impl Texture {
  pub fn load_from_file(path: &Path) -> Result<Texture> {
    let extension = path
      .extension().ok_or(Error::msg("File has no extension"))?
      .to_str().ok_or(Error::msg("Invalid file extension"))?;

    match extension {
      // ".bmp" => Texture::load_from_bmp_file(path),
      "tga" => {
        let file = File::open(path)?;
        let stream = BufReader::new(file);

        Texture::from_tga(stream)
      }
      _ => Result::Err(Error::msg("File not supported")),
    }
  }

  pub(in super) fn from_tga(mut stream: impl Read + Seek) -> Result<Texture> {
    let header = TGAFileHeader {
      ident_size: stream.read_i8()?,
      color_map_type: stream.read_i8()?,
      image_type: stream.read_i8()?,
      _color_map_origin: stream.read_i16::<LittleEndian>()?,
      _color_map_length: stream.read_i16::<LittleEndian>()?,
      _color_map_bits_per_entry: stream.read_i8()?,
      x_offset: stream.read_i16::<LittleEndian>()?,
      y_offset: stream.read_i16::<LittleEndian>()?,
      x_size: stream.read_i16::<LittleEndian>()?,
      y_size: stream.read_i16::<LittleEndian>()?,
      bits_per_pixel: stream.read_i8()?,
      image_descriptor: stream.read_i8()?,
    };

    if header.color_map_type != 0 {
      return Result::Err(Error::msg("Images with color map are not supported"));
    }

    if header.image_type != 2 {
      return Result::Err(Error::msg("Only uncompressed RGB images are supported"));
    }

    if header.x_offset != 0 || header.y_offset != 0 {
      return Result::Err(Error::msg("Images with non zero offset are not supported"));
    }

    if header.bits_per_pixel != 24 && header.bits_per_pixel != 32 {
      return Result::Err(Error::msg("Only images with 24 and 32 bits per pixel are supported"));
    }

    if header.image_descriptor & 0b00110000 != 0 {
      return Result::Err(Error::msg("Image origin is not supported"));
    }

    let width = header.x_size as u32;
    let height = header.y_size as u32;

    let color_buffer_offset = header.ident_size as i64 +
      header._color_map_length as i64 * header._color_map_bits_per_entry as i64 / 8;

    stream.seek(SeekFrom::Current(color_buffer_offset))
      .context("Failed to find pixels data. The file is possibly corrupted.")?;

    let pixel_count = width as i32 * height as i32;
    let bytes_per_pixel = header.bits_per_pixel as usize / 8;
    let image_color_buffer_size = pixel_count as usize * header.bits_per_pixel as usize / 8;
    let mut image_color_buffer: Vec<u8> = vec![0u8; image_color_buffer_size];

    stream.read_exact(image_color_buffer.as_mut_slice())
      .context("Failed to load pixels data. The file is possibly corrupted.")?;

    let texture_color_buffer_size = pixel_count as usize * 3;
    let mut texture_color_buffer: Vec<u8> = Vec::with_capacity(texture_color_buffer_size);

    for pixel in image_color_buffer.chunks(bytes_per_pixel) {
      match *pixel {
        [b, g, r] => {
          texture_color_buffer.extend_from_slice(&[r, g, b]);
        }
        [b, g, r, a] => {
          texture_color_buffer.extend_from_slice(&[
            (r as u32 * a as u32 / 255) as u8,
            (g as u32 * a as u32 / 255) as u8,
            (b as u32 * a as u32 / 255) as u8,
          ]);
        }
        _ => {
          return Result::Err(Error::msg("Failed to convert pixels into RGB format"));
        }
      }
    }

    Result::Ok(Texture {
      width,
      height,
      color_buffer: texture_color_buffer,
    })
  }

  fn _save_to_bmp_file(&self, _path: &Path) -> Result<()> {
    Result::Err(Error::msg("Not implemented"))
  }

  fn _save_to_tga_file(&self, _path: &Path) -> Result<()> {
    Result::Err(Error::msg("Not implemented"))
  }

  pub fn get_pixel_color(&self, x: u32, y: u32) -> Result<Color> {
    if x >= self.width || y >= self.height {
      return Result::Err(Error::msg("Pixel position out of bounds"));
    }

    let index = (x + y * self.width) as usize * 3;
    let rgb: [u8; 3] = self.color_buffer[index..index + 3].try_into()?;
    let color = Color::from_rgb(&rgb);

    Result::Ok(color)
  }


  pub fn get_texel_color(&self, u: f32, v: f32) -> Result<Color> {
    const UPPER_BOUND: f32 = 1.0 - f32::EPSILON;

    if u < 0.0 || u > UPPER_BOUND || v < 0.0 || v >= UPPER_BOUND {
      return Result::Err(Error::msg("Texel position out of bounds"));
    }

    let fx = clamp(u, 0.0, UPPER_BOUND) * self.width as f32;
    let fy = clamp(v, 0.0, UPPER_BOUND) * self.height as f32;
    let x = fx as u32;
    let y = fy as u32;

    // bilinear filtering
    let color = if x < self.width - 1 && y < self.height - 1 {
      let color_00 = self.get_pixel_color(x, y)?;
      let color_01 = self.get_pixel_color(x, y + 1)?;
      let color_10 = self.get_pixel_color(x + 1, y)?;
      let color_11 = self.get_pixel_color(x + 1, y + 1)?;

      let x_fract = fx.fract();
      let y_fract = fy.fract();
      let x_fract_inv = 1.0 - x_fract;
      let y_fract_inv = 1.0 - y_fract;

      (color_00 * x_fract_inv + color_10 * x_fract) * y_fract_inv + (color_01 * x_fract_inv + color_11 * x_fract) * y_fract
    } else {
      self.get_pixel_color(x, y)?
    };

    Result::Ok(color)
  }
}
