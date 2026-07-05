use bevy::prelude::*;
use avian2d::prelude::*;
use crate::GameLayer;
use crate::walls::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL};

const BRICK_COLOR: Color = Color::srgb(0.3, 0.3, 0.8);
const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_WALLS: f32 = 20.0;

#[derive(Component)]
pub struct Breakable {
    health: i32,
}

fn spawn_brick(
    commands: &mut Commands,
    //asset_server: Res<AssetServer>
    brick_pos: Vec2,
    health_amount: i32,
) {
    println!("spawning brick: {brick_pos}");
    commands.spawn((
        Sprite {
            color: BRICK_COLOR,
            custom_size: Some(BRICK_SIZE),
            ..default()
        },
        Transform::from_translation(brick_pos.extend(0.0)),
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Bricks, [GameLayer::Ball]),
        Collider::rectangle(BRICK_SIZE.x, BRICK_SIZE.y),
        Breakable { health: health_amount },
    ));
}

pub fn setup_formation(
    commands: &mut Commands,
    //asset_server: Res<AssetServer>,
    //level_index: u32,
) {
    let offset_x: f32 = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_WALLS + BRICK_SIZE.x * 0.5;
    let offset_y: f32 = TOP_WALL - GAP_BETWEEN_BRICKS_AND_CEILING - BRICK_SIZE.y * 0.5;

    let bricks_total_width: f32 = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BRICKS_AND_WALLS;
    let bricks_total_height: f32 = (TOP_WALL - BOTTOM_WALL) - GAP_BETWEEN_BRICKS_AND_CEILING - GAP_BETWEEN_PADDLE_AND_BRICKS;

    let rows: i32 = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
    let columns: i32 = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

    for row in 0..rows {
        for column in 0..columns {
            let brick_pos: Vec2 = vec2(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y - row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            spawn_brick(commands, brick_pos, 1);
        }
    }
    //spawn_breakable(&mut commands, asset_server);
}