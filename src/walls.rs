use bevy::prelude::*;
use avian2d::prelude::*;
use crate::GameLayer;

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const WALL_THICKNESS: f32 = 10.0;
pub const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
pub const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
pub const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Component)]
pub struct GameWall;

#[derive(Bundle)]
struct WallBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
    rigid_body: RigidBody,
    collision_layers: CollisionLayers,
}

enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let vertical_wall_size = Vec2::new(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizontal_wall_size = Vec2::new(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        match self {
            WallLocation::Left | WallLocation::Right => vertical_wall_size,
            WallLocation::Top | WallLocation::Bottom => horizontal_wall_size,
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> Self {
        let size = location.size();
        Self {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(location.size()),
                ..default()
            },
            transform: Transform {
                translation: location.position().extend(0.0),
                ..default()
            },
            rigid_body: RigidBody::Static,
            collision_layers: CollisionLayers::new(GameLayer::Wall, [GameLayer::Ball]),
            collider: Collider::rectangle(size.x, size.y),
        }
    }
}

pub fn spawn_walls(commands: &mut Commands, ) {
    for location in [
        WallLocation::Left,
        WallLocation::Right,
        WallLocation::Top,
        WallLocation::Bottom,
    ] {
        commands.spawn((WallBundle::new(location), GameWall));
    }
}