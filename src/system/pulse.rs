use std::time::Instant;

use anyhow::{Result, Error, Context};

use super::render::Render;
use super::KEY_CODE;
use super::Default as Config;

use super::render::camera::{
  TURN_LEFT_MASK,
  TURN_RIGHT_MASK,
  TURN_UP_MASK,
  TURN_DOWN_MASK,
  SHIFT_LEFT_MASK,
  SHIFT_RIGHT_MASK,
  SHIFT_UP_MASK,
  SHIFT_DOWN_MASK,
  SHIFT_FORWARD_MASK,
  SHIFT_BACK_MASK,
};

#[derive(PartialEq)]
enum PULSE_STATE
{
  stNew,
  stCameraControl,
  stScreenshotResolutionSelection,
  stScreenshotSamplingSelection,
  stScreenshotRenderBegin,
  stScreenshotRenderProceed,
  stScreenshotRenderEnd,
  stScreenshotRenderSave,
  stScreenshotRenderCancelRequested,
}

impl Default for PULSE_STATE {
  fn default() -> PULSE_STATE {
    PULSE_STATE::stNew
  }
}

struct Resolution<'a>
{
  w: u32,
  h: u32,
  tip: &'a str,
}

const res: [Resolution; 9] = [
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

const ss: [SsRate; 9] = [
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

#[derive(Default)]
pub struct Pulse {
  render: Render,
  state: PULSE_STATE,
  controlFlags: u32,
  frameTimeAccumulator: f32,
  frameTime: f32,
  renderChunkInPixels: u32,
  scrnshotFileName: String,
  scrnshotWidth: u32,
  scrnshotHeight: u32,
  scrnshotSamples: i32,
  scrnshotProgress: f32,
  scrnshotStartTicks: u64,
  quitMessage: bool,
  motionDynSamples: i32,
  prevSamples: i32,
  prevInMotion: bool,
  prevCounter: Option<Instant>,
}

impl Pulse {
  pub fn new() -> Pulse {
    Pulse {
      render: Render::new(),
      motionDynSamples: Config::motionMinSamples,
      prevSamples: 0,
      prevInMotion: false,
      ..Default::default()
    }
  }

  pub fn init(&mut self) -> Result<()> {
    let exe_file_path = std::env::current_exe()?;
    let root_path = exe_file_path.parent().unwrap();
    self.render.init_scene(root_path)?;
    self.setState(PULSE_STATE::stCameraControl);

    Ok(())
  }

  pub fn exec(&mut self) -> Result<bool> {
    assert_ne!(self.render.image_width, 0);
    assert_ne!(self.render.image_height, 0);

    match self.state {
      PULSE_STATE::stNew => {
        //plint->sleep(10);
      }
      PULSE_STATE::stCameraControl => {
        self.proceedControl();
        return self.renderImage();
      }
      PULSE_STATE::stScreenshotResolutionSelection => {
        //
      }
      PULSE_STATE::stScreenshotSamplingSelection => {
        //plint->sleep(10);
      }
      PULSE_STATE::stScreenshotRenderBegin => {
        // scrnshotRenderBegin();
        // setState(stScreenshotRenderProceed);
      }
      PULSE_STATE::stScreenshotRenderProceed => {
        //
      }
      PULSE_STATE::stScreenshotRenderCancelRequested => {
        // if (screenshotRenderProceed())
        // setState(stScreenshotRenderSave);
      }
      PULSE_STATE::stScreenshotRenderSave => {
        // screenshotRenderSave();
        // setState(stScreenshotRenderEnd);
      }
      PULSE_STATE::stScreenshotRenderEnd => {
        // screenshotRenderEnd();
        // setState(stCameraControl);
      }
      _ => {}
    };

    Ok(false)
  }

  pub fn renderImage(&mut self) -> Result<bool> {
    let isComplete;

    let inMotion = self.controlFlags != 0 || self.render.camera.is_in_motion();

    if !self.render.in_progress || (inMotion && self.motionDynSamples != self.prevSamples) {
      if inMotion {
        if self.frameTime > Config::maxMotionFrameTime {
          self.motionDynSamples = i32::max(self.motionDynSamples - 1, Config::motionMaxSamples);
        } else if self.frameTime < Config::minMotionFrameTime {
          self.motionDynSamples = i32::min(self.motionDynSamples + 1, Config::motionMinSamples);
        }
      }

      let reflNum = if inMotion || self.prevInMotion { Config::motionReflections } else { Config::staticReflections };
      let sampleNum = if inMotion || self.prevInMotion { self.motionDynSamples } else { Config::staticSamples };

      self.render.begin_render(reflNum, sampleNum, !(inMotion || self.prevInMotion));
      self.renderChunkInPixels = 1;
      self.prevSamples = sampleNum;
      self.prevInMotion = inMotion;
    }

    let counter = Instant::now();
    isComplete = !self.render.render(self.renderChunkInPixels)?;
    let renderTime = counter.elapsed();
    let renderTimeMs = renderTime.as_millis() as u32;
    self.frameTimeAccumulator += renderTime.as_nanos() as f32 / 1_000_000_000.0;

    if !isComplete {
      if renderTimeMs < Config::minChunkRenderTime {
        self.renderChunkInPixels = u32::min(self.renderChunkInPixels * 2, self.render.image_height * self.render.image_width);
      } else if renderTimeMs > Config::maxChunkRenderTime {
        self.renderChunkInPixels = u32::max(self.renderChunkInPixels / 2, 1);
      }
    } else {
      self.frameTime = self.frameTimeAccumulator;
      self.frameTimeAccumulator = 0.0;
      println!("TIME: {}", self.frameTime);
    }

    Ok(isComplete)
  }

  fn proceedControl(&mut self) {
    let counter = Instant::now();

    if let Some(prevCounter) = self.prevCounter {
      let elapsed = counter.duration_since(prevCounter);
      let timePassedSec = elapsed.as_nanos() as f32 / 1_000_000_000.0;
      self.render.camera.proceed_control(self.controlFlags, timePassedSec);
    }

    self.prevCounter = Some(counter);
  }

  pub fn getRenderImagePixel(&self, x: u32, y: u32) -> [u8; 3] {
    self.render.get_pixel(x, y).rgb()
  }

  pub fn resize_image(&mut self, width: u32, height: u32) {
    self.render.resize_image(width, height);
  }

  fn setState(&mut self, state: PULSE_STATE) {
    self.state = state;
    //plint->invalidateMainWindow();
  }

  pub fn onKeyEvent(&mut self, key: KEY_CODE, isPressed: bool) {
    if self.state == PULSE_STATE::stCameraControl {
      let mut mask = 0;

      match key {
        KEY_CODE::KEY_LEFT => mask = TURN_LEFT_MASK,
        KEY_CODE::KEY_RIGHT => mask = TURN_RIGHT_MASK,
        KEY_CODE::KEY_UP => mask = TURN_DOWN_MASK,
        KEY_CODE::KEY_DOWN => mask = TURN_UP_MASK,
        KEY_CODE::KEY_W => mask = SHIFT_FORWARD_MASK,
        KEY_CODE::KEY_S => mask = SHIFT_BACK_MASK,
        KEY_CODE::KEY_A => mask = SHIFT_LEFT_MASK,
        KEY_CODE::KEY_D => mask = SHIFT_RIGHT_MASK,
        KEY_CODE::KEY_SPACE => mask = SHIFT_UP_MASK,
        KEY_CODE::KEY_CONTROL => mask = SHIFT_DOWN_MASK,
        KEY_CODE::KEY_F2 => {
          if isPressed {
            self.setState(PULSE_STATE::stScreenshotResolutionSelection);
          }
        }
        _ => {}
      }

      self.controlFlags = if isPressed { self.controlFlags | mask } else { self.controlFlags & !mask };
    } else if self.state == PULSE_STATE::stScreenshotResolutionSelection && isPressed {
      let mut index: i32 = -1;

      match key {
        KEY_CODE::KEY_1 => index = 0,
        KEY_CODE::KEY_2 => index = 1,
        KEY_CODE::KEY_3 => index = 2,
        KEY_CODE::KEY_4 => index = 3,
        KEY_CODE::KEY_5 => index = 4,
        KEY_CODE::KEY_6 => index = 5,
        KEY_CODE::KEY_7 => index = 6,
        KEY_CODE::KEY_8 => index = 7,
        KEY_CODE::KEY_9 => index = 8,
        KEY_CODE::KEY_ESCAPE => self.setState(PULSE_STATE::stCameraControl),
        _ => {}
      }

      if index >= 0 {
        self.scrnshotWidth = res[index as usize].w;
        self.scrnshotHeight = res[index as usize].h;
        self.setState(PULSE_STATE::stScreenshotSamplingSelection);
      }
    } else if self.state == PULSE_STATE::stScreenshotSamplingSelection && isPressed {
      let mut index: i32 = -1;

      match key {
        KEY_CODE::KEY_1 => index = 0,
        KEY_CODE::KEY_2 => index = 1,
        KEY_CODE::KEY_3 => index = 2,
        KEY_CODE::KEY_4 => index = 3,
        KEY_CODE::KEY_5 => index = 4,
        KEY_CODE::KEY_6 => index = 5,
        KEY_CODE::KEY_7 => index = 6,
        KEY_CODE::KEY_8 => index = 7,
        KEY_CODE::KEY_9 => index = 8,
        KEY_CODE::KEY_ESCAPE => self.setState(PULSE_STATE::stCameraControl),
        _ => {}
      }

      if index >= 0 {
        self.scrnshotSamples = ss[index as usize].rate;
        self.setState(PULSE_STATE::stScreenshotRenderBegin);
      }
    } else if self.state == PULSE_STATE::stScreenshotRenderProceed && isPressed {
      if key == KEY_CODE::KEY_ESCAPE {
        self.setState(PULSE_STATE::stScreenshotRenderCancelRequested);
      }
    } else if (self.state == PULSE_STATE::stScreenshotRenderCancelRequested && isPressed) {
      match key {
        KEY_CODE::KEY_Y => self.setState(PULSE_STATE::stCameraControl),
        KEY_CODE::KEY_N => self.setState(PULSE_STATE::stScreenshotRenderProceed),
        _ => {}
      }
    }
  }
}