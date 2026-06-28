mod paddle; mod ball;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use paddle::{spawn_paddle, move_paddle};
use ball::{spawn_ball};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle))
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2d::default());

    // paddle(s)
    spawn_paddle(&mut commands, 0., KeyCode::KeyA, KeyCode::KeyD);
    spawn_ball();
    //spawn_paddle(&mut commands, 300., KeyCode::ArrowLeft, KeyCode::ArrowRight);
}