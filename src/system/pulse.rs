use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;

use crate::math::constants::VERY_SMALL_NUMBER;

use super::default as Config;
use super::KeyCode;
use super::render::camera::{
  SHIFT_BACK_MASK,
  SHIFT_DOWN_MASK,
  SHIFT_FORWARD_MASK,
  SHIFT_LEFT_MASK,
  SHIFT_RIGHT_MASK,
  SHIFT_UP_MASK,
  TURN_DOWN_MASK,
  TURN_LEFT_MASK,
  TURN_RIGHT_MASK,
  TURN_UP_MASK,
};
use super::render::Render;

#[derive(PartialEq)]
enum State
{
  New,
  CameraControl,
  ScreenshotResolutionSelection,
  ScreenshotSamplingSelection,
  ScreenshotRenderBegin,
  ScreenshotRenderProceed,
  ScreenshotRenderEnd,
  ScreenshotRenderSave,
  ScreenshotRenderCancelRequested,
}

struct Resolution<'a>
{
  w: u32,
  h: u32,
  tip: &'a str,
}

const RESOLUTIONS: [Resolution; 9] = [
  Resolution { w: 800, h: 600, tip: "(4:3)" },
  Resolution { w: 1024, h: 768, tip: "(4:3)" },
  Resolution { w: 1280, h: 960, tip: "(4:3)" },
  Resolution { w: 1280, h: 800, tip: "(16:10)" },
  Resolution { w: 1680, h: 1050, tip: "(16:10)" },
  Resolution { w: 1920, h: 1200, tip: "(16:10)" },
  Resolution { w: 1280, h: 720, tip: "(HD)" },
  Resolution { w: 1920, h: 1080, tip: "(Full HD)" },
  Resolution { w: 7680, h: 4320, tip: "(Super Hi Vision 16:9)" },
];

struct SsRate<'a>
{
  rate: i32,
  tip: &'a str,
}

const SUPERSAMPLE_RATES: [SsRate; 9] = [
  SsRate { rate: 1, tip: "(fast but rough)" },
  SsRate { rate: 2, tip: "" },
  SsRate { rate: 4, tip: "" },
  SsRate { rate: 8, tip: "" },
  SsRate { rate: 16, tip: "" },
  SsRate { rate: 32, tip: "" },
  SsRate { rate: 64, tip: "" },
  SsRate { rate: 128, tip: "" },
  SsRate { rate: 256, tip: "(slow but smooth)" },
];

pub struct Pulse {
  render: Render,
  state: State,
  last_pulse_time: Instant,
  control_flags: u32,
  frame_time_accumulator: f32,
  frame_time: f32,
  render_chunk_in_pixels: u32,
  screenshot_file_name: PathBuf,
  screenshot_width: u32,
  screenshot_height: u32,
  screenshot_samples: i32,
  screenshot_progress: f32,
  screenshot_start_ticks: Option<Instant>,
  motion_dyn_samples: i32,
  prev_samples: i32,
  prev_in_motion: bool,
  window_width: u32,
  window_height: u32,
}

impl Pulse {
  pub fn new() -> Pulse {
    Pulse {
      render: Render::new(),
      state: State::New,
      last_pulse_time: Instant::now(),
      control_flags: 0,
      frame_time_accumulator: 0.0,
      frame_time: 0.0,
      render_chunk_in_pixels: 0,
      screenshot_file_name: PathBuf::default(),
      screenshot_width: 0,
      screenshot_height: 0,
      screenshot_samples: 0,
      screenshot_progress: 0.0,
      screenshot_start_ticks: None,
      motion_dyn_samples: 0,
      prev_samples: 0,
      prev_in_motion: false,
      window_width: 1,
      window_height: 1,
    }
  }

  pub fn init(&mut self) -> Result<()> {
    let exe_file_path = std::env::current_exe()?;
    let root_path = exe_file_path.parent().unwrap();
    self.render.init_scene(root_path)?;
    self.set_state(State::CameraControl);

    Ok(())
  }

  pub fn pulse(&mut self) -> Result<bool> {
    assert_ne!(self.render.image_width, 0);
    assert_ne!(self.render.image_height, 0);
    let elapsed = self.last_pulse_time.elapsed();
    self.last_pulse_time += elapsed;

    match self.state {
      State::New |
      State::ScreenshotResolutionSelection |
      State::ScreenshotSamplingSelection => {
        std::thread::sleep(Duration::from_millis(10));
      }
      State::CameraControl => {
        self.proceed_control(elapsed);
        return self.render_image();
      }
      State::ScreenshotRenderBegin => {
        self.screenshot_render_begin()?;
        self.set_state(State::ScreenshotRenderProceed);
      }
      State::ScreenshotRenderProceed |
      State::ScreenshotRenderCancelRequested => {
        let is_complete = self.screenshot_render_proceed()?;

        if is_complete {
          self.set_state(State::ScreenshotRenderSave);
        }

        return Ok(is_complete);
      }
      State::ScreenshotRenderSave => {
        self.screenshot_render_save()?;
        self.set_state(State::ScreenshotRenderEnd);
      }
      State::ScreenshotRenderEnd => {
        self.screenshot_render_end()?;
        self.set_state(State::CameraControl);
      }
    };

    Ok(false)
  }

  pub fn render_image(&mut self) -> Result<bool> {
    let is_complete;

    let in_motion = self.control_flags != 0 || self.render.camera.is_in_motion();

    if self.render.is_complete || (in_motion && self.motion_dyn_samples != self.prev_samples) {
      if in_motion {
        if self.frame_time > Config::MAX_MOTION_FRAME_TIME {
          self.motion_dyn_samples = i32::max(self.motion_dyn_samples - 1, Config::MOTION_MAX_SAMPLES);
        } else if self.frame_time < Config::MIN_MOTION_FRAME_TIME {
          self.motion_dyn_samples = i32::min(self.motion_dyn_samples + 1, Config::MOTION_MIN_SAMPLES);
        }
      }

      let reflections = if in_motion || self.prev_in_motion { Config::MOTION_REFLECTIONS } else { Config::STATIC_REFLECTIONS };
      let samples = if in_motion || self.prev_in_motion { self.motion_dyn_samples } else { Config::STATIC_SAMPLES };
      self.render.begin_render(reflections, samples, !(in_motion || self.prev_in_motion));
      self.render_chunk_in_pixels = 1;
      self.prev_samples = samples;
      self.prev_in_motion = in_motion;
    }

    let counter = Instant::now();
    is_complete = self.render.render(self.render_chunk_in_pixels)?;
    let render_time = counter.elapsed();
    let render_time_ms = render_time.as_millis() as u32;
    self.frame_time_accumulator += render_time.as_nanos() as f32 / 1_000_000_000.0;

    if !is_complete {
      if render_time_ms < Config::MIN_CHUNK_RENDER_TIME {
        self.render_chunk_in_pixels = u32::min(self.render_chunk_in_pixels * 2, self.render.image_height * self.render.image_width);
      } else if render_time_ms > Config::MAX_CHUNK_RENDER_TIME {
        self.render_chunk_in_pixels = u32::max(self.render_chunk_in_pixels / 2, 1);
      }
    } else {
      self.frame_time = self.frame_time_accumulator;
      self.frame_time_accumulator = 0.0;
    }

    Ok(is_complete)
  }

  fn proceed_control(&mut self, elapsed: Duration) {
    let time_passed_sec = elapsed.as_nanos() as f32 / 1_000_000_000.0;
    self.render.camera.proceed_control(self.control_flags, time_passed_sec);
  }

  pub fn get_render_image_pixel(&self, x: u32, y: u32) -> [u8; 3] {
    self.render.get_pixel(x, y).rgb()
  }

  pub fn get_render_image_size(&self) -> (u32, u32) {
    (self.render.image_width, self.render.image_height)
  }

  pub fn resize_image(&mut self, width: u32, height: u32) {
    self.window_width = width;
    self.window_height = height;

    match &self.state {
      State::New |
      State::CameraControl => {
        self.set_state(State::CameraControl);
        self.render.resize_image(width, height);
      }
      _ => {}
    }
  }

  fn set_state(&mut self, state: State) {
    self.state = state;
  }

  pub fn handle_key_event(&mut self, key: KeyCode, is_pressed: bool) {
    if self.state == State::CameraControl {
      let mut mask = 0;

      match key {
        KeyCode::KeyLeft => mask = TURN_LEFT_MASK,
        KeyCode::KeyRight => mask = TURN_RIGHT_MASK,
        KeyCode::KeyUp => mask = TURN_DOWN_MASK,
        KeyCode::KeyDown => mask = TURN_UP_MASK,
        KeyCode::KeyW => mask = SHIFT_FORWARD_MASK,
        KeyCode::KeyS => mask = SHIFT_BACK_MASK,
        KeyCode::KeyA => mask = SHIFT_LEFT_MASK,
        KeyCode::KeyD => mask = SHIFT_RIGHT_MASK,
        KeyCode::KeySpace => mask = SHIFT_UP_MASK,
        KeyCode::KeyControl => mask = SHIFT_DOWN_MASK,
        KeyCode::KeyF2 => {
          if is_pressed {
            self.set_state(State::ScreenshotResolutionSelection);
          }
        }
        _ => {}
      }

      self.control_flags = if is_pressed { self.control_flags | mask } else { self.control_flags & !mask };
    } else if self.state == State::ScreenshotResolutionSelection && is_pressed {
      let mut index: i32 = -1;

      match key {
        KeyCode::Key1 => index = 0,
        KeyCode::Key2 => index = 1,
        KeyCode::Key3 => index = 2,
        KeyCode::Key4 => index = 3,
        KeyCode::Key5 => index = 4,
        KeyCode::Key6 => index = 5,
        KeyCode::Key7 => index = 6,
        KeyCode::Key8 => index = 7,
        KeyCode::Key9 => index = 8,
        KeyCode::KeyEscape => self.set_state(State::CameraControl),
        _ => {}
      }

      if index >= 0 {
        self.screenshot_width = RESOLUTIONS[index as usize].w;
        self.screenshot_height = RESOLUTIONS[index as usize].h;
        self.set_state(State::ScreenshotSamplingSelection);
      }
    } else if self.state == State::ScreenshotSamplingSelection && is_pressed {
      let mut index: i32 = -1;

      match key {
        KeyCode::Key1 => index = 0,
        KeyCode::Key2 => index = 1,
        KeyCode::Key3 => index = 2,
        KeyCode::Key4 => index = 3,
        KeyCode::Key5 => index = 4,
        KeyCode::Key6 => index = 5,
        KeyCode::Key7 => index = 6,
        KeyCode::Key8 => index = 7,
        KeyCode::Key9 => index = 8,
        KeyCode::KeyEscape => self.set_state(State::CameraControl),
        _ => {}
      }

      if index >= 0 {
        self.screenshot_samples = SUPERSAMPLE_RATES[index as usize].rate;
        self.set_state(State::ScreenshotRenderBegin);
      }
    } else if self.state == State::ScreenshotRenderProceed && is_pressed {
      if key == KeyCode::KeyEscape {
        self.set_state(State::ScreenshotRenderCancelRequested);
      }
    } else if self.state == State::ScreenshotRenderCancelRequested && is_pressed {
      match key {
        KeyCode::KeyY => self.set_state(State::CameraControl),
        KeyCode::KeyN => self.set_state(State::ScreenshotRenderProceed),
        _ => {}
      }
    }
  }

  pub fn get_current_screen_text(&self) -> Vec<String> {
    let mut screen_text = Vec::new();

    match &self.state {
      State::CameraControl => {
        let str1 = format!("Resolution : {}x{}", self.render.image_width, self.render.image_height);
        screen_text.push(str1);

        if self.frame_time < 10.0 {
          screen_text.push(format!("Frame time: {:.0} ms", self.frame_time * 1000.0));
        } else {
          screen_text.push(format!("Frame time: {:.03} s", self.frame_time));
        }

        screen_text.push(format!("Blended frames : {}", self.render.additive_counter));
        screen_text.push(String::from(" "));
        screen_text.push(String::from("WSAD : move"));
        screen_text.push(String::from("Cursor keys: turn"));
        screen_text.push(String::from("Space : ascent"));
        screen_text.push(String::from("Ctrl : descent"));
        screen_text.push(String::from(" "));
        screen_text.push(String::from("F2 : save screenshot"));
      }
      State::ScreenshotResolutionSelection => {
        screen_text.push(String::from("Select screenshot resolution (keys 1-9)"));
        screen_text.push(String::from(" "));

        RESOLUTIONS.iter().enumerate().for_each(|(i, res)| {
          screen_text.push(format!("{} : {}x{} {}", i + 1, res.w, res.h, res.tip));
        });

        screen_text.push(String::from(" "));
        screen_text.push(String::from("ESC : cancel"));
      }
      State::ScreenshotSamplingSelection => {
        screen_text.push(String::from("Select supersampling rate (keys 1-9)"));
        screen_text.push(String::from(" "));

        SUPERSAMPLE_RATES.iter().enumerate().for_each(|(i, ss)| {
          screen_text.push(format!("{} : {}x{} {}", i + 1, ss.rate, ss.rate, ss.tip));
        });

        screen_text.push(String::from(" "));
        screen_text.push(String::from("ESC : cancel"));
      }
      State::ScreenshotRenderBegin |
      State::ScreenshotRenderProceed |
      State::ScreenshotRenderSave |
      State::ScreenshotRenderEnd |
      State::ScreenshotRenderCancelRequested => {
        screen_text.push(String::from("Saving screenshot:"));
        screen_text.push(self.screenshot_file_name.file_name().unwrap().to_str().unwrap().to_owned());
        screen_text.push(format!("Resolution: {}x{}", self.screenshot_width, self.screenshot_height));
        screen_text.push(format!("SSAA: {}x{}", self.screenshot_samples, self.screenshot_samples));
        screen_text.push(String::from(""));
        screen_text.push(format!("Progress: {:.2} %", self.screenshot_progress));

        if self.screenshot_progress > VERY_SMALL_NUMBER {
          if let Some(screenshot_start_ticks) = self.screenshot_start_ticks {
            let time_passed = Instant::now().duration_since(screenshot_start_ticks);
            let time_left = (time_passed.as_secs_f32() * (100.0 / self.screenshot_progress - 1.0)) as u32;

            let hr = time_left / 3600;
            let time_left = time_left % 3600;
            let min = time_left / 60;
            let time_left = time_left % 60;
            let sec = time_left;

            screen_text.push(format!("Estimated time left: {} h {:02} m {:02} s", hr, min, sec));
            screen_text.push(String::from(""));
          }
        }

        if self.state == State::ScreenshotRenderCancelRequested {
          screen_text.push(String::from("Do you want to cancel ? ( Y / N ) "));
        } else {
          screen_text.push(String::from("Press ESC to cancel"));
        }
      }
      _ => {
        panic!();
      }
    }

    screen_text
  }

  fn screenshot_render_begin(&mut self) -> Result<()> {
    let system_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let name = format!("screenshot_{:08X}.bmp", system_time);
    self.screenshot_file_name = std::env::current_exe()?.parent().unwrap().join(name);

    self.render.resize_image(self.screenshot_width, self.screenshot_height);
    self.screenshot_start_ticks = Some(Instant::now());
    self.render.begin_render(Config::SCREENSHOT_REFLECTIONS, self.screenshot_samples, false);
    self.render_chunk_in_pixels = 1;

    Ok(())
  }

  fn screenshot_render_proceed(&mut self) -> Result<bool> {
    let now = Instant::now();
    let is_complete = self.render.render(self.render_chunk_in_pixels)?;
    let elapsed_ms = now.elapsed().as_millis() as u32;

    if elapsed_ms < Config::MIN_CHUNK_RENDER_TIME {
      self.render_chunk_in_pixels = u32::min(self.render_chunk_in_pixels * 2, self.render.image_height * self.render.image_width);
    } else if elapsed_ms > Config::MAX_CHUNK_RENDER_TIME {
      self.render_chunk_in_pixels = u32::max(self.render_chunk_in_pixels / 2, 1);
    }

    self.screenshot_progress = self.render.get_progress();

    Ok(is_complete)
  }

  fn screenshot_render_save(&mut self) -> Result<()> {
    self.render.to_texture().save_to_file(&self.screenshot_file_name)
  }

  fn screenshot_render_end(&mut self) -> Result<()> {
    self.screenshot_progress = 0.0;
    self.screenshot_width = 0;
    self.screenshot_height = 0;
    self.screenshot_samples = 0;
    self.render.resize_image(self.window_width, self.window_height);

    Ok(())
  }
}