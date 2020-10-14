use crate::math::{Matrix33, Vector3};
use crate::render::{Camera, Scene, Color, Skybox, Texture, Material, MaterialKind, Pulse};
use std::path::Path;

use anyhow::{Result, Error, Context};
use std::rc::Rc;

use super::math::Rnd;
use super::math::constants::FAST_RAND_MAX;

thread_local! {
  pub static RND: Rc<Rnd> = Rc::new(Rnd::new());
}

#[derive(Default)]
pub struct Render {
  image: Vec<Color>,
  cur_x: u32,
  cur_y: u32,
  max_reflections: u32,
  samples: i32,
  is_additive: bool,
  camera_view: Matrix33,
  camera_eye: Vector3,

  pub camera: Camera,
  pub scene: Scene,
  pub image_width: u32,
  pub image_height: u32,
  pub additive_counter: i32,
  pub in_progress: bool,
}

impl Render {
  pub fn new() -> Render {
    Render {
      image: Vec::new(),
      cur_x: 0,
      cur_y: 0,
      max_reflections: 0,
      samples: 0,
      is_additive: false,
      camera_view: Matrix33::default(),
      camera_eye: Vector3::default(),
      camera: Camera::default(),
      scene: Scene::default(),
      image_width: 0,
      image_height: 0,
      additive_counter: 0,
      in_progress: false,
    }
  }

  pub fn init_scene(&mut self, root_path: &Path) -> Result<()> {
    let skybox_texture_path = root_path.join("textures/skybox.tga");
    let periodic_texture_path = root_path.join("textures/periodic.tga");
    let skybox_texture = Texture::load_from_file(&skybox_texture_path)?;
    let periodic_texture = Rc::new(Texture::load_from_file(&periodic_texture_path)?);

    let eye_pos = Vector3::new(7.427, 3.494, -3.773);
    let look_at = Vector3::new(6.5981, 3.127, -3.352);
    self.camera = Camera::new(eye_pos, look_at, 1.05);
    let skybox = Skybox::new(skybox_texture);
    let diffuse_light_color = Color::new(0.95, 0.95, 1.0);
    let diffuse_light_power = 0.15;
    let mut scene = Scene::new(skybox, diffuse_light_color, diffuse_light_power);

    scene.add_spot_light(Vector3::new(11.8e9, 4.26e9, 3.08e9), 3.48e8, Color::new(1.0, 1.0, 0.95), 0.85);
    //scene.addLight(Vector3::new(-1.26e9, 11.8e9, 1.08e9), 6.96e8, Color::new(1.0, 0.5, 0.5), 0.2);
    //scene.addLight(Vector3::new(11.8e9, 4.26e9, 3.08e9), 6.96e9, Color::new(1.0, 1.0, 0.95), 0.85);

    scene.add_sphere(Vector3::new(-1.25, 1.5, -0.25), 1.5, Material::new(MaterialKind::Metal, Color::new(1.0, 1.0, 1.0), 1.0, 0.0));
    scene.add_sphere(Vector3::new(0.15, 1.0, 1.75), 1.0, Material::new(MaterialKind::Metal, Color::new(1.0, 1.0, 1.0), 0.95, 0.0));

    scene.add_sphere(Vector3::new(-3.0, 0.6, -3.0), 0.6, Material::new(MaterialKind::Dielectric, Color::new(1.0, 1.0, 1.0), 0.0, 0.0));
    scene.add_sphere(Vector3::new(-0.5, 0.5, -2.5), 0.5, Material::new(MaterialKind::Dielectric, Color::new(0.5, 1.0, 0.15), 0.75, 0.0));
    scene.add_sphere(Vector3::new(1.0, 0.4, -1.5), 0.4, Material::new(MaterialKind::Dielectric, Color::new(0.0, 0.5, 1.0), 1.0, 0.0));

    scene.add_sphere(Vector3::new(1.8, 0.4, 0.1), 0.4, Material::new(MaterialKind::Metal, Color::new(1.0, 0.65, 0.45), 1.0, 0.0));
    scene.add_sphere(Vector3::new(1.7, 0.5, 1.9), 0.5, Material::new(MaterialKind::Metal, Color::new(1.0, 0.90, 0.60), 0.75, 0.0));
    scene.add_sphere(Vector3::new(0.6, 0.6, 4.2), 0.6, Material::new(MaterialKind::Metal, Color::new(0.9, 0.9, 0.9), 0.0, 0.0));

    let plane_vertices = (
      Vector3::new(-14.0, 0.0, -10.0),
      Vector3::new(-14.0, 0.0, 10.0),
      Vector3::new(14.0, 0.0, 10.0),
      Vector3::new(14.0, 0.0, -10.0),
    );

    let plane_uvs = (
      (0.0, 0.0),
      (0.0, 1.0),
      (1.0, 1.0),
      (1.0, 0.0),
    );

    scene.add_triangle([
      &plane_vertices.0,
      &plane_vertices.1,
      &plane_vertices.3,
    ], Material::new(MaterialKind::Dielectric, Color::new(1.0, 1.0, 1.0), 0.95, 0.0),
      Some((
        Rc::clone(&periodic_texture), [
          plane_uvs.0,
          plane_uvs.1,
          plane_uvs.3,
        ])),
    );

    scene.add_triangle([
      &plane_vertices.1,
      &plane_vertices.2,
      &plane_vertices.3,
    ], Material::new(MaterialKind::Dielectric, Color::new(1.0, 1.0, 1.0), 0.95, 0.0),
      Some((
        Rc::clone(&periodic_texture), [
          plane_uvs.1,
          plane_uvs.2,
          plane_uvs.3,
        ])),
    );

    self.scene = scene;

    Ok(())
  }

  pub fn resize_image(&mut self, width: u32, height: u32)  {
    assert!(width > 0, "Invalid argument");
    assert!(height > 0, "Invalid argument");

    let new_size = (width * height) as usize;
    const ZERO_COLOR: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

    if new_size > self.image.len() {
      self.image.resize(new_size, ZERO_COLOR);
    }

    self.image_width = width;
    self.image_height = height;
    self.additive_counter = 0;
    self.in_progress = false;
    self.cur_x = 0;
    self.cur_y = 0;
  }

  pub fn to_texture(&self) -> Texture {
    assert!(self.image_width > 0);
    assert!(self.image_height > 0);

    let size = self.image_width as usize * self.image_height as usize * 3;
    let mut color_buffer = Vec::with_capacity(size);
    self.image.iter().for_each(|c| color_buffer.extend(&c.rgb()));

    Texture {
      width: self.image_width,
      height: self.image_height,
      color_buffer,
    }
  }

  pub fn get_pixel(&self, x: u32, y: u32) -> Color {
    assert!(x < self.image_width, "Invalid argument");
    assert!(y < self.image_height, "Invalid argument");

    let idx = x as usize + y as usize * self.image_width as usize;
    let mut color = self.image[idx].clone();

    if self.additive_counter > 1 {
      color /= self.additive_counter as f32;
    }

    color
  }

  pub fn begin_render(&mut self, reflect_num: u32, sample_num: i32, additive: bool) {
    assert!(reflect_num > 0, "Invalid argument");
    assert_ne!(sample_num, 0, "Invalid argument");

    self.max_reflections = reflect_num;
    self.samples = sample_num;
    self.is_additive = additive;
    self.in_progress = true;
    self.cur_x = 0;
    self.cur_y = 0;
    self.camera_view = self.camera.view.clone();
    self.camera_eye = self.camera.eye.clone();

    if additive {
      self.additive_counter += 1;
    } else {
      self.additive_counter = 0;
    }
  }

  pub fn render(&mut self, pixels: u32) -> Result<bool> {
    assert!(pixels > 0, "Invalid argument");
    assert!(self.in_progress, "Invalid state");
    assert!(self.cur_x < self.image_width, "Invalid state");
    assert!(self.cur_y < self.image_height, "Invalid state");

    let mut pixels = pixels;
    let origin = &self.camera_eye;
    let sq_samples = i32::pow(self.samples, 2);
    let rz = self.image_width as f32 / 2.0 / f32::tan(&self.camera.fov / 2.0);
    let image_width_half = self.image_width as f32 / 2.0;
    let image_height_half = self.image_height as f32 / 2.0;

    while pixels > 0 {
      let rx = self.cur_x as f32 - image_width_half;
      let ry = self.cur_y as f32 - image_height_half;
      let mut ray = Vector3::new(rx, ry, rz);

      if self.samples < 0 {
        let down_samples = i32::abs(self.samples) as u32;

        if self.cur_x % down_samples == 0 &&
          self.cur_y % down_samples == 0
        {
          ray = &self.camera_view * ray;
          let traced_color = self.scene.trace(origin, &ray, self.max_reflections)?;
          let end_qx = u32::min(self.image_width, self.cur_x + down_samples);
          let end_qy = u32::min(self.image_height, self.cur_y + down_samples);

          for qx in self.cur_x..end_qx {
            for qy in self.cur_y..end_qy {
              let idx = (qx + qy * self.image_width) as usize;
              self.image[idx] = traced_color.clone();
            }
          }
        }
      } else {
        let mut fin_color: Color;
        let rnd = RND.with(|r| Rc::clone(r));
        let rnd_x = if self.is_additive { rnd.fastrand() as f32 / FAST_RAND_MAX  as f32 } else { 0.0 };
        let rnd_y = if self.is_additive { rnd.fastrand() as f32  / FAST_RAND_MAX  as f32 } else { 0.0 };
        fin_color = Color::new(0.0, 0.0, 0.0);

        for ssx in 0..self.samples {
          for ssy in 0..self.samples {
            let mut ray = Vector3::new(
              rx + ssx as f32 / self.samples as f32 + rnd_x,
              ry + ssy as f32 / self.samples as f32 + rnd_y,
              rz,
            );

            ray = &self.camera_view * &ray;
            fin_color += self.scene.trace(origin, &ray, self.max_reflections)?;
          }
        }

        fin_color /= sq_samples as f32;
        let idx = (self.cur_x + self.cur_y * self.image_width) as usize;

        if self.additive_counter > 1 {
          self.image[idx] += fin_color;
        } else {
          self.image[idx] = fin_color;
        }
      }

      pixels -= 1;
      self.cur_x += 1;

      if self.cur_x == self.image_width {
        self.cur_x = 0;
        self.cur_y += 1;
      }

      if self.cur_y == self.image_height {
        self.in_progress = false;
        break;
      }
    }

    Ok(self.in_progress)
  }

  pub fn render_all(&mut self, reflect_num: u32, sample_num: i32, additive: bool) -> Result<()> {
    self.begin_render(reflect_num, sample_num, additive);
    self.render(self.image_height)?;

    Ok(())
  }

  pub fn get_progress(&self) -> f32 {
    (self.cur_x + self.cur_y * self.image_width) as f32 * 100.0 / self.image_width as f32 / self.image_height as f32
  }
}