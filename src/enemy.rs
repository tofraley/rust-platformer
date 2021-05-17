use macroquad::prelude::*;
//use macroquad_platformer::*;
use crate::world::*;

pub struct Enemy {
  pub collider: Actor,
  pub speed: Vec2,
  pub health: i32,
}

impl Enemy {
  pub fn init(world: &mut World) -> Self {
    Enemy {
      collider: world.add_actor(vec2(80., 14. * 8.), 8, 8),
      speed: vec2(30., 0.),
      health: 3,
    }
  }
}
