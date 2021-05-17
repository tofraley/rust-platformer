use macroquad::prelude::*;
//use macroquad_platformer::*;
use crate::world::*;

pub const MAX_JUMPS: i32 = 2;
pub const LEFT_INPUT: KeyCode = KeyCode::A;
pub const RIGHT_INPUT: KeyCode = KeyCode::D;
pub const JUMP_INPUT: KeyCode = KeyCode::Space;
pub const HANG_TIME: f32 = 0.0;
pub const ATK_COOL_DN: f32 = 1.;

#[derive(Debug)]
pub struct Player {
  pub collider: Actor,
  pub stance: Stance,
  pub speed: Vec2,
  pub jumps_left: i32,
  pub hang_before_cling: f32,
  pub atk_cool_dn: f32,
}

impl Player {
  pub fn init(world: &mut World) -> Self {
    Player {
      collider: world.add_actor(vec2(50.0, 13. * 8.), 8, 8),
      stance: Stance::InAir(InAir::Falling),
      speed: vec2(0., 0.),
      jumps_left: MAX_JUMPS,
      hang_before_cling: HANG_TIME,
      atk_cool_dn: 0.,
    }
  }

  pub fn transition(world: &World, player: &mut Player) -> Stance {
    let player_pos = world.actor_pos(player.collider);
    match &player.stance {
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
        let colliding_right = world.collide_check(player.collider, player_pos + vec2(1., 0.));
        let colliding_left = world.collide_check(player.collider, player_pos + vec2(-1., 0.));
        if (colliding_right && is_key_down(RIGHT_INPUT))
          || (colliding_left && is_key_down(LEFT_INPUT))
        {
          if player.hang_before_cling > 0.0 {
            player.hang_before_cling -= get_frame_time();
          } else {
            return Stance::Clinging(Clinging::PreCling);
          }
        }

        if world.collide_check(player.collider, player_pos + vec2(0., 1.)) {
          player.hang_before_cling = HANG_TIME;
          return Stance::OnGround;
        } else if player.jumps_left > 0 && is_key_pressed(JUMP_INPUT) {
          if colliding_left && is_key_down(RIGHT_INPUT) {
            player.speed.x += 100.;
          } else if colliding_right && is_key_down(LEFT_INPUT) {
            player.speed.x -= 100.;
          }
          return Stance::InAir(InAir::Jumping);
        } else {
          return Stance::InAir(InAir::Falling);
        }
      }
      Stance::Clinging(cling_state) => match cling_state {
        Clinging::PreCling => {
          player.hang_before_cling -= get_frame_time();
          if world.collide_check(player.collider, player_pos + vec2(1., 0.))
            && is_key_down(RIGHT_INPUT)
            && player.hang_before_cling <= 0.0
          {
            player.hang_before_cling = HANG_TIME;
            return Stance::Clinging(Clinging::Right);
          } else if world.collide_check(player.collider, player_pos + vec2(-1., 0.))
            && is_key_down(LEFT_INPUT)
            && player.hang_before_cling <= 0.0
          {
            player.hang_before_cling = HANG_TIME;
            return Stance::Clinging(Clinging::Left);
          } else {
            return Stance::InAir(InAir::Falling);
          }
        }
        Clinging::Right => {
          if is_key_pressed(JUMP_INPUT) {
            player.speed.x -= 100.;
            return Stance::InAir(InAir::Jumping);
          } else if world.collide_check(player.collider, player_pos + vec2(1., 0.))
            && is_key_down(RIGHT_INPUT)
          {
            return Stance::Clinging(Clinging::Right);
          } else {
            return Stance::InAir(InAir::Falling);
          }
        }
        Clinging::Left => {
          if is_key_pressed(JUMP_INPUT) {
            player.speed.x += 100.;
            return Stance::InAir(InAir::Jumping);
          } else if world.collide_check(player.collider, player_pos + vec2(-1., 0.))
            && is_key_down(LEFT_INPUT)
          {
            return Stance::Clinging(Clinging::Left);
          } else {
            return Stance::InAir(InAir::Falling);
          }
        }
      },
    }
  }
}

#[derive(Debug)]
pub enum Clinging {
  PreCling,
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
