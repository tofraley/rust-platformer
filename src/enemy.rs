use macroquad::prelude::*;
use macroquad_platformer::*;

pub struct Enemy {
  pub collider: Actor,
  pub speed: Vec2,
}

impl Enemy {
  pub fn init(world: &mut World) -> Self {
    Enemy {
      collider: world.add_actor(vec2(80., 14. * 8.), 8, 8),
      speed: vec2(30., 0.),
    }
  }
}
