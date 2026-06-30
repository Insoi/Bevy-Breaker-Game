mod paddle; mod ball; mod walls;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use paddle::{spawn_paddle, move_paddle};
use ball::{spawn_ball, apply_velocity};
use walls::{spawn_walls};
use crate::ball::{check_ball_collisions, setup_balls};

#[derive(Component)]
pub struct Collider {
    size: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2d::default());

    // paddle(s)
    spawn_paddle(&mut commands, 0., KeyCode::ArrowLeft, KeyCode::ArrowRight);
    spawn_walls(&mut commands);
    setup_balls(&mut commands, asset_server);
    //spawn_paddle(&mut comman-ds, 300., KeyCode::ArrowLeft, KeyCode::ArrowRight);
}