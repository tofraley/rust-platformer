use macroquad::prelude::*;
use macroquad_platformer::*;

pub const MAX_JUMPS: i32 = 2;
pub const LEFT_INPUT: KeyCode = KeyCode::S;
pub const RIGHT_INPUT: KeyCode = KeyCode::F;
pub const JUMP_INPUT: KeyCode = KeyCode::Space;

#[derive(Debug)]
pub struct Player {
  pub collider: Actor,
  pub stance: Stance,
  pub speed: Vec2,
  pub jumps_left: i32,
}

impl Player {
  pub fn init(world: &mut World) -> Self {
    Player {
      collider: world.add_actor(vec2(50.0, 13. * 8.), 8, 8),
      stance: Stance::InAir(InAir::Falling),
      speed: vec2(0., 0.),
      jumps_left: MAX_JUMPS,
    }
  }

  pub fn transition(world: &World, player: &Player) -> Stance {
    let player_pos = world.actor_pos(player.collider);
    match player.stance {
      Stance::OnGround => {
        if is_key_pressed(KeyCode::Space) {
          return Stance::InAir(InAir::Jumping);
        } else if world.collide_check(player.collider, player_pos + vec2(0., 1.)) {
          return Stance::OnGround;
        } else {
          return Stance::InAir(InAir::Falling);
        }
      }
      Stance::InAir(_) => {
        if world.collide_check(player.collider, player_pos + vec2(1., 0.))
          && is_key_down(RIGHT_INPUT)
        {
          return Stance::Clinging(Clinging::Right);
        } else if world.collide_check(player.collider, player_pos + vec2(-1., 0.))
          && is_key_down(LEFT_INPUT)
        {
          return Stance::Clinging(Clinging::Left);
        } else if world.collide_check(player.collider, player_pos + vec2(0., 1.)) {
          return Stance::OnGround;
        } else if player.jumps_left > 0 && is_key_pressed(JUMP_INPUT) {
          return Stance::InAir(InAir::Jumping);
        } else {
          return Stance::InAir(InAir::Falling);
        }
      }
      Stance::Clinging(_) => {
        if world.collide_check(player.collider, player_pos + vec2(1., 0.))
          && is_key_down(RIGHT_INPUT)
        {
          return Stance::Clinging(Clinging::Right);
        } else if world.collide_check(player.collider, player_pos + vec2(-1., 0.))
          && is_key_down(LEFT_INPUT)
        {
          return Stance::Clinging(Clinging::Left);
        } else if is_key_pressed(JUMP_INPUT) {
          return Stance::InAir(InAir::Jumping);
        } else {
          return Stance::InAir(InAir::Falling);
        }
      }
    }
  }
}

#[derive(Debug)]
pub enum Clinging {
  Left,
  Right,
}

#[derive(Debug)]
pub enum InAir {
  Falling,
  Jumping,
}

#[derive(Debug)]
pub enum Stance {
  Clinging(Clinging),
  OnGround,
  InAir(InAir),
}
