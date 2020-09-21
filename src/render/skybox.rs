use anyhow::{Result};

use super::math::Vector3;
use super::math::constants::VERY_SMALL_NUMBER;
use super::{Color, Texture};

pub struct Skybox {
  half_tile_width: f32,
  half_tile_height: f32,
  texture: Texture,
}

impl Skybox {
  pub fn from_texture(texture: Texture) -> Skybox {
    Skybox {
      half_tile_width: 1.0 / 8.0 - /*1.0 / texture.width as f32 -*/ f32::EPSILON,
      half_tile_height: 1.0 / 6.0 - /*1.0 / texture.height as f32 -*/ f32::EPSILON,
      texture,
    }
  }

  pub fn trace(&self, ray: &Vector3) -> Result<Color> {
    //uv origins of cube sides centers in skybox texture
    const LEFT_U: f32 = 1.0 / 8.0;
    const LEFT_V: f32 = 3.0 / 6.0;
    const FRONT_U: f32 = 3.0 / 8.0;
    const FRONT_V: f32 = 3.0 / 6.0;
    const RIGHT_U: f32 = 5.0 / 8.0;
    const RIGHT_V: f32 = 3.0 / 6.0;
    const BACK_U: f32 = 7.0 / 8.0;
    const BACK_V: f32 = 3.0 / 6.0;
    const TOP_U: f32 = 3.0 / 8.0;
    const TOP_V: f32 = 5.0 / 6.0;
    const BOTTOM_U: f32 = 3.0 / 8.0;
    const BOTTOM_V: f32 = 1.0 / 6.0;

    let ray = ray.normalized();
    let ax = ray.x.abs() + VERY_SMALL_NUMBER;
    let ay = ray.y.abs() + VERY_SMALL_NUMBER;
    let az = ray.z.abs() + VERY_SMALL_NUMBER;
    let mut u: f32 = 0.0;
    let mut v: f32 = 0.0;

    if az >= ax && az >= ay {
      if ray.z > 0.0 {
        u = FRONT_U + ray.x / az * self.half_tile_width;
        v = FRONT_V + ray.y / az * self.half_tile_height;
      } else {
        u = BACK_U - ray.x / az * self.half_tile_width;
        v = BACK_V + ray.y / az * self.half_tile_height;
      }
    } else if ax >= ay && ax >= az {
      if ray.x > 0.0 {
        u = RIGHT_U - ray.z / ax * self.half_tile_width;
        v = RIGHT_V + ray.y / ax * self.half_tile_height;
      } else {
        u = LEFT_U + ray.z / ax * self.half_tile_width;
        v = LEFT_V + ray.y / ax * self.half_tile_height;
      }
    } else {
      if ray.y > 0.0 {
        u = TOP_U + ray.x / ay * self.half_tile_width;
        v = TOP_V - ray.z / ay * self.half_tile_height;
      } else {
        u = BOTTOM_U + ray.x / ay * self.half_tile_width;
        v = BOTTOM_V + ray.z / ay * self.half_tile_height;
      }
    }

    self.texture.get_texel_color(u, v)
  }
}

