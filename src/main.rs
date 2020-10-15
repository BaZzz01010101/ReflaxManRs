use std::vec;
use sdl2::render::{CanvasBuilder, WindowCanvas, Texture, TextureAccess};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use core::convert::From;

use anyhow::{Result, Context};

#[macro_use]
mod macros;

mod math;
mod render;
mod system;

use system::Pulse;

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

  let mut canvas: WindowCanvas = CanvasBuilder::from(window)
    .build()
    .unwrap();

  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();
  let texture_creator = canvas.texture_creator();
  let mut tex: Texture = texture_creator.create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, width, height).unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut pixel_buffer_size: usize = (width * height * 3) as usize;
  let mut pixel_buffer_pitch: usize = (width * 3) as usize;
  let mut pixel_data = vec![128u8; pixel_buffer_size];
  let mut pulse = Pulse::new();
  pulse.init()?;
  pulse.resize_image(width, height);

  'running: loop {
    if let Ok(true) = pulse.exec() {
      for x in 0..width {
        for y in 0..height {
          let mut rgb = pulse.getRenderImagePixel(x, y);
          let idx = (x + y * width) as usize * 3;
          pixel_data[idx..idx + 3].copy_from_slice(&rgb);
        }
      }

      tex.update(None, &*pixel_data, pixel_buffer_pitch).unwrap();
      canvas.copy_ex(&tex, None, None, 0.0, None, false, true).unwrap();
      canvas.present();
    }

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running;
        }
        Event::Window { win_event: WindowEvent::Resized(w, h), .. } => {
          println!("Resize: {} x {}", w, h);
          width = w as u32;
          height = h as u32;
          pulse.resize_image(width, height);
          pixel_buffer_size = (width * height * 3) as usize;
          pixel_buffer_pitch = (width * 3) as usize;
          pixel_data.resize(pixel_buffer_size, 128);
          tex = texture_creator.create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, width, height).unwrap();
        }
        _ => {}
      }
    }
  }

  Ok(())
}
