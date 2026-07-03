use bevy::prelude::*;
use crate::Collider;

const BREAKABLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.8);
const BREAKABLE_SIZE: Vec2 = Vec2::new(100.0, 30.0);

#[derive(Component)]
pub struct Breakable {
    health: u32,
}

fn spawn_breakable(
    commands: &mut Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Sprite {
            color: BREAKABLE_COLOR,
            custom_size: Some(BREAKABLE_SIZE),
            ..default()
        },
        Transform {
            translation: vec3(0., 100., 0.),
            ..default()
        },
        Breakable { health: 2 },
        Collider { size: BREAKABLE_SIZE },
    ));
}

pub fn setup_formation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_index: u32,
) {
    spawn_breakable(&mut commands, asset_server);
}