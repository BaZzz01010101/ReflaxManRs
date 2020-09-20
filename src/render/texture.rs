use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader, Seek, SeekFrom};
use anyhow::{Result, Error, Context};
use byteorder::{ReadBytesExt, LittleEndian};
use std::convert::TryInto;

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
    let extension = path.extension().ok_or(Error::msg("File has no extension"))?;
    let extension = extension.to_str().ok_or(Error::msg("File extension is not valid UTF8 string"))?;

    match extension {
      // ".bmp" => Texture::load_from_bmp_file(path),
      "tga" => Texture::load_from_tga_file(path),
      _ => Result::Err(Error::msg("File not supported")),
    }
  }

  fn load_from_tga_file(path: &Path) -> Result<Texture> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    let header = TGAFileHeader {
      ident_size: buf_reader.read_i8()?,
      color_map_type: buf_reader.read_i8()?,
      image_type: buf_reader.read_i8()?,
      _color_map_origin: buf_reader.read_i16::<LittleEndian>()?,
      _color_map_length: buf_reader.read_i16::<LittleEndian>()?,
      _color_map_bits_per_entry: buf_reader.read_i8()?,
      x_offset: buf_reader.read_i16::<LittleEndian>()?,
      y_offset: buf_reader.read_i16::<LittleEndian>()?,
      x_size: buf_reader.read_i16::<LittleEndian>()?,
      y_size: buf_reader.read_i16::<LittleEndian>()?,
      bits_per_pixel: buf_reader.read_i8()?,
      image_descriptor: buf_reader.read_i8()?,
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

    buf_reader.seek(SeekFrom::Current(color_buffer_offset))
      .context("Failed to find pixels data. The file is possibly corrupted.")?;

    let pixel_count = width as i32 * height as i32;
    let bytes_per_pixel = header.bits_per_pixel as usize / 8;
    let image_color_buffer_size = pixel_count as usize * header.bits_per_pixel as usize / 8;
    let mut image_color_buffer: Vec<u8> = vec![0u8; image_color_buffer_size];

    buf_reader.read_exact(image_color_buffer.as_mut_slice())
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

  pub fn get_rgb_pixel(&self, x: u32, y: u32) -> Result<&[u8; 3]> {
    if x >= self.width || y >= self.height {
      return Result::Err(Error::msg("Pixel position out of bounds"));
    }

    let first_byte = (x as usize + y as usize * self.width as usize) * 3;

    Ok(self.color_buffer[first_byte..first_byte + 3].try_into()?)
  }

  fn _save_to_bmp_file(&self, _path: &Path) -> Result<()> {
    Result::Err(Error::msg("Not implemented"))
  }

  fn _save_to_tga_file(&self, _path: &Path) -> Result<()> {
    Result::Err(Error::msg("Not implemented"))
  }
}