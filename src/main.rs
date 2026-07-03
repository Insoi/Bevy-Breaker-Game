mod paddle;
mod ball;
mod walls;
mod breakables;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use paddle::{spawn_paddle, move_paddle};
use ball::{setup_balls, apply_velocity, check_ball_collisions};
use walls::{spawn_walls};
use breakables::{setup_formation};

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