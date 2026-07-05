use bevy::prelude::*;
use avian2d::prelude::*;
use crate::GameLayer;
use crate::walls::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL};

const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 30.0;
const GAP_BETWEEN_BRICKS_AND_WALLS: f32 = 50.0;

const HEALTH_AMOUNT_COLOR: [Color; 4] = [
    Color::srgb(0.3, 0.3, 0.9), // 1 - blue
    Color::srgb(1.0, 0.9, 0.2), // 2 - yellow
    Color::srgb(1.0, 0.55, 0.1), // 3 - orange
    Color::srgb(0.85, 0.2, 0.2), // 4 - red
];

#[derive(Component)]
pub struct Breakable {
    pub health: i32,
    pub collision_cooldown: Timer,
}

fn spawn_brick(
    commands: &mut Commands,
    //asset_server: Res<AssetServer>
    brick_pos: Vec2,
    health_amount: i32,
) {
    let brick_color = HEALTH_AMOUNT_COLOR[(health_amount - 1) as usize];

    commands.spawn((
        Sprite {
            color: brick_color,
            custom_size: Some(BRICK_SIZE),
            ..default()
        },
        Transform::from_translation(brick_pos.extend(0.0)),
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Bricks, [GameLayer::Ball]),
        Collider::rectangle(BRICK_SIZE.x, BRICK_SIZE.y),
        Breakable {
            health: health_amount,
            collision_cooldown: Timer::from_seconds(0.2, TimerMode::Once)
        },
    ));
}

pub fn update_brick_appearance(
    mut query: Query<(&Breakable, &mut Sprite), Changed<Breakable>>,
) {
    for (breakable, mut sprite) in query.iter_mut() {
        sprite.color = HEALTH_AMOUNT_COLOR[(breakable.health - 1) as usize];
    }
}

pub fn update_breakable_timers(
    time: Res<Time>,
    mut query: Query<&mut Breakable>,
) {
    for mut breakable in query.iter_mut() {
        breakable.collision_cooldown.tick(time.delta());
    }
}

pub fn setup_formation(
    commands: &mut Commands,
    //asset_server: Res<AssetServer>,
    //level_index: u32,
) {
    let bricks_total_width = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BRICKS_AND_WALLS;
    let bricks_total_height = (TOP_WALL - BOTTOM_WALL) - GAP_BETWEEN_BRICKS_AND_CEILING - GAP_BETWEEN_PADDLE_AND_BRICKS;

    let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
    let columns = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

    let grid_width = columns as f32 * BRICK_SIZE.x + (columns - 1) as f32 * GAP_BETWEEN_BRICKS;

    let center_x = (LEFT_WALL + RIGHT_WALL) * 0.5;
    let offset_x = center_x - grid_width * 0.5 + BRICK_SIZE.x * 0.5;

    let offset_y = TOP_WALL - GAP_BETWEEN_BRICKS_AND_CEILING - BRICK_SIZE.y * 0.5;

    for row in 0..rows {
        for column in 0..columns {
            let brick_pos: Vec2 = vec2(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y - row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            spawn_brick(commands, brick_pos, 4);
        }
    }
    //spawn_breakable(&mut commands, asset_server);
}