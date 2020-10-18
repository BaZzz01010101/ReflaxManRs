use std::time::Instant;

use anyhow::{Error, Result};
use rusttype::Font;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{TextureAccess, WindowCanvas};
use sdl2::surface::Surface;

use system::KeyCode;
use system::App;

use crate::math::constants::VERY_SMALL_NUMBER;

#[macro_use]
mod macros;

mod math;
mod render;
mod system;

fn main() {
  if let Err(err) = run() {
    println!("FAILED:\n{}", err);
  }
}

fn run() -> Result<()> {
  let mut width: u32 = 640;
  let mut height: u32 = 480;
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window("ReflaxManRs", width, height)
    .position_centered()
    .resizable()
    .build()
    .unwrap();

  let mut canvas: WindowCanvas = window.into_canvas()
    .target_texture()
    .build()
    .unwrap();

  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();
  let texture_creator = canvas.texture_creator();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut app = App::new();
  app.init()?;
  app.resize_image(width, height);
  let font_data = include_bytes!("../assets/fonts/arial.ttf");
  let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
  let mut time = Instant::now();
  let mut image_surface = Surface::new(width, height, PixelFormatEnum::RGBA8888).map_err(Error::msg)?;
  let mut final_texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Static, width, height)?;

  'running: loop {
    let mut need_repaint = app.pulse()?;

    if need_repaint {
      image_surface = rebuild_image_surface(&app);
    }

    if need_repaint || time.elapsed().as_millis() > 100 {
      time = Instant::now();
      let final_surface = render_all_text(&app, &image_surface, &font, width, height);
      final_texture = texture_creator.create_texture_from_surface(final_surface).unwrap();
      need_repaint = true;
    }

    if need_repaint {
      canvas.copy(&final_texture, None, None).unwrap();
      canvas.present();
    }

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => {
          break 'running;
        }
        Event::KeyDown { keycode: Some(key), .. } => {
          handle_key_message(&mut app, key, true);
        }
        Event::KeyUp { keycode: Some(key), .. } => {
          handle_key_message(&mut app, key, false);
        }
        Event::Window { win_event: WindowEvent::Resized(w, h), .. } => {
          println!("Resize: {} x {}", w, h);
          width = w as u32;
          height = h as u32;
          app.resize_image(width, height);
        }
        _ => {}
      }
    }
  }

  Ok(())
}

fn handle_key_message(app: &mut App, key_code: Keycode, is_down: bool) {
  match key_code {
    Keycode::Left => app.handle_key_event(KeyCode::KeyLeft, is_down),
    Keycode::Right => app.handle_key_event(KeyCode::KeyRight, is_down),
    Keycode::Up => app.handle_key_event(KeyCode::KeyUp, is_down),
    Keycode::Down => app.handle_key_event(KeyCode::KeyDown, is_down),
    Keycode::W => app.handle_key_event(KeyCode::KeyW, is_down),
    Keycode::S => app.handle_key_event(KeyCode::KeyS, is_down),
    Keycode::A => app.handle_key_event(KeyCode::KeyA, is_down),
    Keycode::D => app.handle_key_event(KeyCode::KeyD, is_down),
    Keycode::Space => app.handle_key_event(KeyCode::KeySpace, is_down),
    Keycode::LCtrl => app.handle_key_event(KeyCode::KeyControl, is_down),
    Keycode::F2 => app.handle_key_event(KeyCode::KeyF2, is_down),
    Keycode::Num1 => app.handle_key_event(KeyCode::Key1, is_down),
    Keycode::Num2 => app.handle_key_event(KeyCode::Key2, is_down),
    Keycode::Num3 => app.handle_key_event(KeyCode::Key3, is_down),
    Keycode::Num4 => app.handle_key_event(KeyCode::Key4, is_down),
    Keycode::Num5 => app.handle_key_event(KeyCode::Key5, is_down),
    Keycode::Num6 => app.handle_key_event(KeyCode::Key6, is_down),
    Keycode::Num7 => app.handle_key_event(KeyCode::Key7, is_down),
    Keycode::Num8 => app.handle_key_event(KeyCode::Key8, is_down),
    Keycode::Num9 => app.handle_key_event(KeyCode::Key9, is_down),
    Keycode::Escape => app.handle_key_event(KeyCode::KeyEscape, is_down),
    Keycode::Y => app.handle_key_event(KeyCode::KeyY, is_down),
    Keycode::N => app.handle_key_event(KeyCode::KeyN, is_down),
    _ => {}
  }
}

fn render_line(surface: &mut Surface, font: &Font, size: f32, rgb: [u8; 3], text: &str, x: u32, y: u32) {
  let scale = rusttype::Scale::uniform(size);
  let position = rusttype::point(x as f32, y as f32);
  let glyphs: Vec<_> = font.layout(text, scale, position).collect();
  let width = surface.width() as i32;
  let pixels = surface.without_lock_mut().unwrap();

  for glyph in glyphs {
    if let Some(bounding_box) = glyph.pixel_bounding_box() {
      glyph.draw(|x, y, v| {
        let based_x = x as i32 + bounding_box.min.x;
        let based_y = y as i32 + bounding_box.min.y;
        let idx = (based_x + width * based_y) as usize * 4;

        if idx + 4 < pixels.len() {
          let old = &pixels[idx..idx + 4];
          let a0 = old[0] as u32;
          let b0 = old[1] as u32;
          let g0 = old[2] as u32;
          let r0 = old[3] as u32;

          let new = [(v * 255.0) as u8, rgb[0], rgb[1], rgb[2]];
          let a1 = new[0] as u32;
          let b1 = new[1] as u32;
          let g1 = new[2] as u32;
          let r1 = new[3] as u32;

          let new = [
            u32::min(a1 + a0 * (255 - a1) / 255, 255) as u8,
            u32::min((b1 * a1 + b0 * a0 * (255 - a1) / 255) / 255, 255) as u8,
            u32::min((g1 * a1 + g0 * a0 * (255 - a1) / 255) / 255, 255) as u8,
            u32::min((r1 * a1 + r0 * a0 * (255 - a1) / 255) / 255, 255) as u8,
          ];

          pixels[idx..idx + 4].copy_from_slice(&new);
        }
      });
    }
  }
}

fn rebuild_image_surface<'a, 'b>(app: &'a App) -> Surface<'b> {
  let (width, height) = app.get_render_image_size();
  let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA8888).unwrap();
  let pixel_data = surface.without_lock_mut().unwrap();

  for x in 0..width {
    for y in 0..height {
      let rgb = app.get_render_image_pixel(x, height - y - 1);
      let idx = (x + y * width) as usize * 4;
      pixel_data[idx] = 255;
      pixel_data[idx + 1] = rgb[2];
      pixel_data[idx + 2] = rgb[1];
      pixel_data[idx + 3] = rgb[0];
    }
  }

  surface
}

fn render_all_text<'a, 'b>(app: &'a App, background_surface: &Surface, font: &Font, width: u32, height: u32) -> Surface<'b> {
  let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA8888).unwrap();
  let center = surface.rect().center();
  let bk_aspect = background_surface.width() as f32 / background_surface.height() as f32;
  let aspect = surface.width() as f32 / surface.height() as f32;

  let dst_rect = if aspect > VERY_SMALL_NUMBER {
    let (width, height) = if bk_aspect < aspect {
      (surface.width(), (surface.width() as f32 / bk_aspect) as u32)
    } else {
      ((surface.height() as f32 * bk_aspect) as u32, surface.height())
    };

    Rect::from_center(center, width, height)
  } else {
    Rect::from_center(center, 1, 1)
  };

  background_surface.blit_scaled(None, &mut surface, Some(dst_rect)).unwrap();
  let mut y = 12;
  let x = 10;
  const FONT_SIZE: f32 = 10.5;
  let metrics = font.v_metrics(rusttype::Scale::uniform(FONT_SIZE));
  let line_height = (metrics.ascent - metrics.descent + metrics.line_gap) as u32 + 3;

  app.get_current_screen_text().iter().for_each(|line| {
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x + 1, y + 1);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x + 1, y - 1);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x - 1, y + 1);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x - 1, y - 1);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x, y + 1);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x + 1, y);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x, y - 1);
    render_line(&mut surface, &font, FONT_SIZE, [170, 170, 170], line, x - 1, y);
    render_line(&mut surface, &font, FONT_SIZE, [8, 8, 8], line, x, y);
    y += line_height;
  });

  surface
}
