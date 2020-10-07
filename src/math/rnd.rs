use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Instant};
use std::num::Wrapping;

const FAST_RAND_MASK: i32 = 0x7FFF;

struct Rnd {
  g_seed: i32,
}

impl Rnd {
  pub fn new() -> Rnd {
    let mut hasher = DefaultHasher::new();
    Instant::now().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    let hash = hasher.finish();
    let seed = (hash ^ (hash >> 32)) as i32 & FAST_RAND_MASK;

    Rnd { g_seed: seed }
  }

  // generate pseudo-random numbers 0-32767
  pub fn fastrand(&mut self) -> i32 {
    self.g_seed = (Wrapping(214013) * Wrapping(self.g_seed) + Wrapping(2531011)).0;

    (self.g_seed >> 16) & FAST_RAND_MASK
  }
}