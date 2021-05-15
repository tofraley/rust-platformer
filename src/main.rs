use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

mod enemy;
mod platform;
mod player;
use enemy::*;
use platform::*;
use player::*;

#[macroquad::main("Platformer")]
async fn main() {
    let tiled_map = load_map().await;
    let static_colliders = load_static_colliders(&tiled_map);

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);

    let mut player = Player::init(&mut world);
    let mut enemy = Enemy::init(&mut world);
    let mut platform = Platform::init(&mut world);

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 320.0, 152.0));

    loop {
        clear_background(BLACK);
        set_camera(&camera);

        tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, 320.0, 152.0), None);
        draw_platform(&world, &tiled_map, &platform);
        draw_player(&world, &tiled_map, &player);
        draw_enemy(&world, &tiled_map, &enemy);

        update_player(&mut world, &mut player, &enemy);
        update_enemy(&mut world, &mut enemy);
        update_platform(&mut world, &mut platform);

        next_frame().await
    }
}

async fn load_map() -> tiled::Map {
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    return tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();
}

fn load_static_colliders(tiled_map: &tiled::Map) -> Vec<bool> {
    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        static_colliders.push(tile.is_some());
    }
    static_colliders
}

fn draw_platform(world: &World, map: &tiled::Map, platform: &Platform) {
    let pos = world.solid_pos(platform.collider);
    map.spr_ex(
        "tileset",
        Rect::new(6.0 * 8.0, 0.0, 32.0, 8.0),
        Rect::new(pos.x, pos.y, 32.0, 8.0),
    )
}

fn draw_player(world: &World, map: &tiled::Map, player: &Player) {
    // sprite id from tiled
    const PLAYER_SPRITE: u32 = 120;

    let pos = world.actor_pos(player.collider);
    if player.speed.x >= 0.0 {
        map.spr("tileset", PLAYER_SPRITE, Rect::new(pos.x, pos.y, 8.0, 8.0));
    } else {
        map.spr(
            "tileset",
            PLAYER_SPRITE,
            Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
        );
    }
}

fn draw_enemy(world: &World, map: &tiled::Map, enemy: &Enemy) {
    // sprite id from tiled
    const ENEMY_SPRITE: u32 = 120;

    let pos = world.actor_pos(enemy.collider);
    if enemy.speed.x >= 0.0 {
        map.spr("tileset", ENEMY_SPRITE, Rect::new(pos.x, pos.y, 8.0, 8.0));
    } else {
        map.spr(
            "tileset",
            ENEMY_SPRITE,
            Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
        );
    }
}

fn update_player(world: &mut World, player: &mut Player, enemy: &Enemy) {
    let player_pos = world.actor_pos(player.collider);
    let enemy_pos = world.actor_pos(enemy.collider);
    let (player_x, player_y) = player_pos.into();
    let (enemy_x, enemy_y) = enemy_pos.into();
    let player_rect = Rect::new(player_x, player_y, 8., 8.);
    let enemy_rect = Rect::new(enemy_x, enemy_y, 8., 8.);
    let overlapping = player_rect.overlaps(&enemy_rect);
    if overlapping {
        player.speed.y = -120.;
    }
    player.stance = Player::transition(&world, &player);

    if is_key_down(RIGHT_INPUT) {
        player.speed.x = 100.0;
    } else if is_key_down(LEFT_INPUT) {
        player.speed.x = -100.0;
    } else {
        player.speed.x = 0.;
    }

    match &player.stance {
        Stance::OnGround => {
            player.jumps_left = MAX_JUMPS;
            player.speed.y = 0.;
        }
        Stance::InAir(air_stance) => match air_stance {
            InAir::Jumping => {
                if player.jumps_left > 0 {
                    player.jumps_left -= 1;
                    player.speed.y = -120.;
                }
            }
            InAir::Falling => player.speed.y += 500. * get_frame_time(),
        },
        Stance::Clinging(_) => {
            player.jumps_left = MAX_JUMPS;
            player.speed.x = 0.;
            player.speed.y = 0.;
        }
    }

    player.speed.y = clamp(player.speed.y, -500., 200.);

    // debug ui
    //let debug_text = format!(
    //    "player: stance: {:?}, jumps left: {}, pos: {}",
    //    player.stance, player.jumps_left, player_pos
    //);
    //draw_text_ex(
    //    &debug_text,
    //    40.0,
    //    16.0,
    //    TextParams {
    //        font_size: 12,
    //        font_scale: 0.5,
    //        ..Default::default()
    //    },
    //);

    world.move_h(player.collider, player.speed.x * get_frame_time());
    world.move_v(player.collider, player.speed.y * get_frame_time());
}

fn update_enemy(world: &mut World, enemy: &mut Enemy) {
    let pos = world.actor_pos(enemy.collider);
    let left_bound = 80.;
    let right_bound = 104.;
    if enemy.speed.x > 1. && pos.x >= right_bound {
        enemy.speed.x *= -1.;
    }
    if enemy.speed.x < -1. && pos.x <= left_bound {
        enemy.speed.x *= -1.;
    }
    //let debug_text = format!("enemy pos: {}", pos);
    //draw_text_ex(
    //    &debug_text,
    //    40.0,
    //    20.0,
    //    TextParams {
    //        font_size: 12,
    //        font_scale: 0.5,
    //        ..Default::default()
    //    },
    //);

    world.move_h(enemy.collider, enemy.speed.x * get_frame_time());
    world.move_v(enemy.collider, enemy.speed.y * get_frame_time());
}

fn update_platform(world: &mut World, platform: &mut Platform) {
    world.solid_move(platform.collider, platform.speed * get_frame_time(), 0.0);
    let pos = world.solid_pos(platform.collider);
    if platform.speed > 1. && pos.x >= 220. {
        platform.speed *= -1.;
    }
    if platform.speed < -1. && pos.x <= 150. {
        platform.speed *= -1.;
    }
}
