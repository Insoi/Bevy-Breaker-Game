mod paddle;
mod ball;
mod walls;
mod breakables;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use paddle::{spawn_paddle, move_paddle};
use ball::{setup_balls, detect_ball_collision, maintain_ball_speed};
use walls::{spawn_walls};
use breakables::{setup_formation};

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Ball,
    Wall,
    Paddle,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                detect_ball_collision,
                maintain_ball_speed.after(detect_ball_collision),
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