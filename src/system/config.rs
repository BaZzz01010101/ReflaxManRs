pub mod default {
  pub const STATIC_SAMPLES: i32 = 1;
  pub const MOTION_MIN_SAMPLES: i32 = -1;
  pub const MOTION_MAX_SAMPLES: i32 = -8;

  pub const STATIC_REFLECTIONS: u32 = 15;
  pub const MOTION_REFLECTIONS: u32 = 4;
  pub const SCRNSHOT_REFECTIONS: u32 = 20;

  pub const MIN_MOTION_FRAME_TIME: f32 = 0.010;
  pub const MAX_MOTION_FRAME_TIME: f32 = 0.020;

  pub const MIN_CHUNK_RENDER_TIME: u32 = 5;
  pub const MAX_CHUNK_RENDER_TIME: u32 = 20;
}