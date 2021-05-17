use crate::world::*;
use macroquad::prelude::*;

pub struct Platform {
  pub collider: Solid,
  pub speed: f32,
}

impl Platform {
  pub fn init(world: &mut World) -> Self {
    Platform {
      collider: world.add_solid(vec2(170.0, 130.0), 32, 8),
      speed: 50.,
    }
  }
}
