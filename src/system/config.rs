pub mod Default {
  pub const staticSamples: i32 = 1;
  pub const motionMinSamples: i32 = -1;
  pub const motionMaxSamples: i32 = -8;

  pub const staticReflections: u32 = 15;
  pub const motionReflections: u32 = 4;
  pub const scrnshotRefections: u32 = 20;

  pub const minMotionFrameTime: f32 = 0.010;
  pub const maxMotionFrameTime: f32 = 0.020;

  pub const minChunkRenderTime: u32 = 5;
  pub const maxChunkRenderTime: u32 = 20;
}