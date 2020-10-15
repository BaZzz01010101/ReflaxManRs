use std::time::Instant;

use anyhow::{Result, Error, Context};

use crate::render::Render;
use crate::system::Default as Config;

#[derive(Default)]
pub struct Pulse {
  render: Render,
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

    Ok(())
  }

  pub fn exec(&mut self) -> Result<bool> {
    assert_ne!(self.render.image_width, 0);
    assert_ne!(self.render.image_height, 0);

    self.renderImage()
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

  pub fn getRenderImagePixel(&self, x: u32, y: u32) -> [u8; 3] {
    self.render.get_pixel(x, y).rgb()
  }

  pub fn resize_image(&mut self, width: u32, height: u32) {
    self.render.resize_image(width, height);
  }
}