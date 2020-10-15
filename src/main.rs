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
use system::KEY_CODE;

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
        Event::Quit { .. } => {
          break 'running;
        }
        Event::KeyDown { keycode: Some(key), .. } => {
          OnKeyMessage(&mut pulse, key, true);
        }
        Event::KeyUp { keycode: Some(key), .. } => {
          OnKeyMessage(&mut pulse, key, false);
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

fn OnKeyMessage(pulse: &mut Pulse, Key: Keycode, isDown: bool)
{
  match Key {
    Keycode::Left => pulse.onKeyEvent(KEY_CODE::KEY_LEFT, isDown),
    Keycode::Right => pulse.onKeyEvent(KEY_CODE::KEY_RIGHT, isDown),
    Keycode::Up => pulse.onKeyEvent(KEY_CODE::KEY_UP, isDown),
    Keycode::Down => pulse.onKeyEvent(KEY_CODE::KEY_DOWN, isDown),
    Keycode::W => pulse.onKeyEvent(KEY_CODE::KEY_W, isDown),
    Keycode::S => pulse.onKeyEvent(KEY_CODE::KEY_S, isDown),
    Keycode::A => pulse.onKeyEvent(KEY_CODE::KEY_A, isDown),
    Keycode::D => pulse.onKeyEvent(KEY_CODE::KEY_D, isDown),
    Keycode::Space => pulse.onKeyEvent(KEY_CODE::KEY_SPACE, isDown),
    Keycode::LCtrl => pulse.onKeyEvent(KEY_CODE::KEY_CONTROL, isDown),
    Keycode::F2 => pulse.onKeyEvent(KEY_CODE::KEY_F2, isDown),
    Keycode::Num0 => pulse.onKeyEvent(KEY_CODE::KEY_1, isDown),
    Keycode::Num2 => pulse.onKeyEvent(KEY_CODE::KEY_2, isDown),
    Keycode::Num3 => pulse.onKeyEvent(KEY_CODE::KEY_3, isDown),
    Keycode::Num4 => pulse.onKeyEvent(KEY_CODE::KEY_4, isDown),
    Keycode::Num5 => pulse.onKeyEvent(KEY_CODE::KEY_5, isDown),
    Keycode::Num6 => pulse.onKeyEvent(KEY_CODE::KEY_6, isDown),
    Keycode::Num7 => pulse.onKeyEvent(KEY_CODE::KEY_7, isDown),
    Keycode::Num8 => pulse.onKeyEvent(KEY_CODE::KEY_8, isDown),
    Keycode::Num9 => pulse.onKeyEvent(KEY_CODE::KEY_9, isDown),
    Keycode::Escape => pulse.onKeyEvent(KEY_CODE::KEY_ESCAPE, isDown),
    Keycode::Y => pulse.onKeyEvent(KEY_CODE::KEY_Y, isDown),
    Keycode::N => pulse.onKeyEvent(KEY_CODE::KEY_N, isDown),
    _ => {}
  }
}
