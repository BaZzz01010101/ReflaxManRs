use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Instant};
use std::num::Wrapping;
use std::cell::RefCell;

use super::constants::FAST_RAND_MAX;

pub struct Rnd {
  g_seed: RefCell<i32>,
}

impl Rnd {
  pub fn new() -> Rnd {
    let mut hasher = DefaultHasher::new();
    Instant::now().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    let hash = hasher.finish();
    let seed = (hash ^ (hash >> 32)) as i32 & FAST_RAND_MAX;

    Rnd { g_seed: RefCell::new(seed) }
  }

  pub fn fastrand(&self) -> i32 {
    let seed = *self.g_seed.borrow();
    let seed = (Wrapping(214013) * Wrapping(seed) + Wrapping(2531011)).0;
    *self.g_seed.borrow_mut() = seed;

    (seed >> 16) & FAST_RAND_MAX
  }
}