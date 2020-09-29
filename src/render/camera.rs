use std::f32::consts::{PI, FRAC_PI_2};

use super::math::{Vector3, Matrix33};
use super::math::clamp;

const MUL_PI_2: f32 = 2.0 * PI;

const TURN_ACCELERATION: f32 = 2.0;
const TURN_DECELERATION: f32 = 2.0;
const MAX_TURN_SPEED: f32 = 0.2;

const SHIFT_ACCELERATION: f32 = 50.0;
const SHIFT_DECELERATION: f32 = 50.0;
const MAX_SHIFT_SPEED: f32 = 10.0;

const TURN_LEFT_MASK: u32 = 1 << 0;
const TURN_RIGHT_MASK: u32 = 1 << 1;
const TURN_UP_MASK: u32 = 1 << 2;
const TURN_DOWN_MASK: u32 = 1 << 3;
const SHIFT_LEFT_MASK: u32 = 1 << 6;
const SHIFT_RIGHT_MASK: u32 = 1 << 7;
const SHIFT_UP_MASK: u32 = 1 << 8;
const SHIFT_DOWN_MASK: u32 = 1 << 9;
const SHIFT_FORWARD_MASK: u32 = 1 << 10;
const SHIFT_BACK_MASK: u32 = 1 << 11;

pub struct Camera {
  pub turn_rl_speed: f32,
  pub turn_ud_speed: f32,
  pub shift_rl_speed: f32,
  pub shift_ud_speed: f32,
  pub shift_fb_speed: f32,

  pub yaw: f32,
  pub pitch: f32,

  pub fov: f32,
  pub eye: Vector3,
  pub view: Matrix33,
}

impl Camera {
  pub fn new(eye: Vector3, at: Vector3, fov: f32) -> Camera {
    let up = Vector3::new(0.0, 1.0, 0.0);
    let oz = (at - &eye).normalized();
    let ox = (up % &oz).normalized();
    let oy = (&oz % &ox).normalized();

    let view = Matrix33::from_cols(&ox, &oy, &oy);
    let mut yaw = ox.z.acos();

    if ox.x < 0.0 {
      yaw = 2.0 * PI - yaw;
    }

    yaw -= FRAC_PI_2;
    let pitch = oz.y.asin();

    Camera {
      fov,
      eye,
      yaw,
      pitch,
      view,
      turn_rl_speed: 0.0,
      turn_ud_speed: 0.0,
      shift_rl_speed: 0.0,
      shift_ud_speed: 0.0,
      shift_fb_speed: 0.0,
    }
  }

  fn proceed_control(&mut self, control_flags: u32, time_passed_sec: f32) {
    let prev_turn_rl_speed = self.turn_rl_speed;
    let prev_turn_ud_speed = self.turn_ud_speed;
    let prev_shift_rl_speed = self.shift_rl_speed;
    let prev_shift_ud_speed = self.shift_ud_speed;
    let prev_shift_fb_speed = self.shift_fb_speed;

    match control_flags & (TURN_LEFT_MASK | TURN_RIGHT_MASK)
    {
      TURN_RIGHT_MASK => {
        let yaw_speed = self.turn_rl_speed + TURN_ACCELERATION * time_passed_sec;
        self.turn_rl_speed = clamp(yaw_speed, -MAX_TURN_SPEED, MAX_TURN_SPEED);
      }
      TURN_LEFT_MASK => {
        let yaw_speed = self.turn_rl_speed - TURN_ACCELERATION * time_passed_sec;
        self.turn_rl_speed = clamp(yaw_speed, -MAX_TURN_SPEED, MAX_TURN_SPEED);
      }
      _ => {
        if self.turn_rl_speed < 0.0 {
          self.turn_rl_speed = f32::min(0.0, self.turn_rl_speed + TURN_DECELERATION * time_passed_sec);
        } else if self.turn_rl_speed > 0.0 {
          self.turn_rl_speed = f32::max(0.0, self.turn_rl_speed - TURN_DECELERATION * time_passed_sec);
        }
      }
    }

    match control_flags & (TURN_UP_MASK | TURN_DOWN_MASK)
    {
      TURN_UP_MASK => {
        let pitch_speed = self.turn_ud_speed + TURN_ACCELERATION * time_passed_sec;
        self.turn_ud_speed = clamp(pitch_speed, -MAX_TURN_SPEED, MAX_TURN_SPEED);
      }
      TURN_DOWN_MASK => {
        let pitch_speed = self.turn_ud_speed - TURN_ACCELERATION * time_passed_sec;
        self.turn_ud_speed = clamp(pitch_speed, -MAX_TURN_SPEED, MAX_TURN_SPEED);
      }
      _ => {
        if self.turn_ud_speed < 0.0 {
          self.turn_ud_speed = f32::min(0.0, self.turn_ud_speed + TURN_DECELERATION * time_passed_sec);
        } else if self.turn_ud_speed > 0.0 {
          self.turn_ud_speed = f32::max(0.0, self.turn_ud_speed - TURN_DECELERATION * time_passed_sec);
        }
      }
    }

    match control_flags & (SHIFT_LEFT_MASK | SHIFT_RIGHT_MASK)
    {
      SHIFT_RIGHT_MASK => {
        if self.shift_rl_speed < 0.0 {
          let shift_rl_speed = self.shift_rl_speed + (SHIFT_DECELERATION + SHIFT_ACCELERATION) * time_passed_sec;
          self.shift_rl_speed = clamp(shift_rl_speed, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
        } else {
          let shift_rl_speed = self.shift_rl_speed + SHIFT_ACCELERATION * time_passed_sec;
          self.shift_rl_speed = clamp(shift_rl_speed, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
        }
      }

      SHIFT_LEFT_MASK => {
        if self.shift_rl_speed > 0.0 {
          self.shift_rl_speed = clamp(self.shift_rl_speed - (SHIFT_DECELERATION + SHIFT_ACCELERATION) * time_passed_sec, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
        } else {
          self.shift_rl_speed = clamp(self.shift_rl_speed - SHIFT_ACCELERATION * time_passed_sec, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
        }
      }
      _ => {
        if self.shift_rl_speed < 0.0 {
          self.shift_rl_speed = f32::min(0.0, self.shift_rl_speed + SHIFT_DECELERATION * time_passed_sec);
        } else if self.shift_rl_speed > 0.0 {
          self.shift_rl_speed = f32::max(0.0, self.shift_rl_speed - SHIFT_DECELERATION * time_passed_sec);
        }
      }
    }

    match control_flags & (SHIFT_UP_MASK | SHIFT_DOWN_MASK)
    {
      SHIFT_UP_MASK => {
        self.shift_ud_speed = clamp(self.shift_ud_speed + SHIFT_ACCELERATION * time_passed_sec, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
      }
      SHIFT_DOWN_MASK => {
        self.shift_ud_speed = clamp(self.shift_ud_speed - SHIFT_ACCELERATION * time_passed_sec, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
      }
      _ => {
        if self.shift_ud_speed < 0.0 {
          self.shift_ud_speed = f32::min(0.0, self.shift_ud_speed + SHIFT_DECELERATION * time_passed_sec);
        } else if self.shift_ud_speed > 0.0 {
          self.shift_ud_speed = f32::max(0.0, self.shift_ud_speed - SHIFT_DECELERATION * time_passed_sec);
        }
      }
    }

    match control_flags & (SHIFT_BACK_MASK | SHIFT_FORWARD_MASK)
    {
      SHIFT_FORWARD_MASK => {
        self.shift_fb_speed = clamp(self.shift_fb_speed + SHIFT_ACCELERATION * time_passed_sec, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
      }
      SHIFT_BACK_MASK => {
        self.shift_fb_speed = clamp(self.shift_fb_speed - SHIFT_ACCELERATION * time_passed_sec, -MAX_SHIFT_SPEED, MAX_SHIFT_SPEED);
      }
      _ => {
        if self.shift_fb_speed < 0.0 {
          self.shift_fb_speed = f32::min(0.0, self.shift_fb_speed + SHIFT_DECELERATION * time_passed_sec);
        } else if self.shift_fb_speed > 0.0 {
          self.shift_fb_speed = f32::max(0.0, self.shift_fb_speed - SHIFT_DECELERATION * time_passed_sec);
        }
      }
    }

    self.yaw += time_passed_sec * MUL_PI_2 * (self.turn_rl_speed + prev_turn_rl_speed) / 2.0;
    self.pitch = clamp(self.pitch + time_passed_sec * MUL_PI_2 * (self.turn_ud_speed + prev_turn_ud_speed) / 2.0, -0.95 * FRAC_PI_2, 0.95 * FRAC_PI_2);

    if self.yaw >= MUL_PI_2 {
      self.yaw -= MUL_PI_2;
    } else if self.yaw <= -MUL_PI_2 {
      self.yaw += MUL_PI_2;
    }

    self.view = Matrix33::from_yaw_pitch(self.yaw, self.pitch);

    if self.shift_rl_speed.abs() > f32::EPSILON ||
      self.shift_ud_speed.abs() > f32::EPSILON ||
      self.shift_fb_speed.abs() > f32::EPSILON
    {
      let right = self.view.get_col(0);
      let up = Vector3::new(0.0, 1.0, 0.0);
      let front = (right % &up).normalized();

      let mid_shift_rl_speed = 0.5 * (self.shift_rl_speed + prev_shift_rl_speed);
      let mid_shift_ud_speed = 0.5 * (self.shift_ud_speed + prev_shift_ud_speed);
      let mid_shift_fb_speed = 0.5 * (self.shift_fb_speed + prev_shift_fb_speed);

      let mut shift =
        &mid_shift_rl_speed * self.view.get_col(0) +
          &mid_shift_ud_speed * up +
          &mid_shift_fb_speed * front;

      let shift_sq_length = shift.sq_length();

      if shift_sq_length > MAX_SHIFT_SPEED * MAX_SHIFT_SPEED {
        shift *= MAX_SHIFT_SPEED / shift_sq_length.sqrt();
      }

      self.eye += shift * time_passed_sec;
    }
  }

  fn is_in_motion(&self) -> bool {
    return self.turn_rl_speed.abs() > f32::EPSILON ||
      self.turn_ud_speed.abs() > f32::EPSILON ||
      self.shift_rl_speed.abs() > f32::EPSILON ||
      self.shift_ud_speed.abs() > f32::EPSILON ||
      self.shift_fb_speed.abs() > f32::EPSILON;
  }
}
