use std::vec;
use sdl2::render::{CanvasBuilder, WindowCanvas, Texture, TextureAccess};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use core::convert::From;

#[macro_use]
mod macros;

mod math;

fn main() {
  let mut width: u32 = 800;
  let mut height: u32 = 600;
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
  let mut tex: Texture = texture_creator.create_texture(PixelFormatEnum::RGB888, TextureAccess::Static, width, height).unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut pixel_buffer_size: usize = (width * height * 4) as usize;
  let mut pixel_buffer_pitch: usize = (width * 4) as usize;
  let mut pixel_data = vec![128u8; pixel_buffer_size];

  'running: loop {
    tex.update(None, &*pixel_data, pixel_buffer_pitch).unwrap();
    canvas.copy(&tex, None, None).unwrap();
    canvas.present();

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
          pixel_buffer_size = (width * height * 4) as usize;
          pixel_buffer_pitch = (width * 4) as usize;
          pixel_data.resize(pixel_buffer_size, 128);
          tex = texture_creator.create_texture(PixelFormatEnum::RGB888, TextureAccess::Static, width, height).unwrap();
        }
        _ => {}
      }
    }
  }
}
