mod paddle;
mod ball;
mod walls;
mod bricks;
mod shop;
mod inventory;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use paddle::{DragState, spawn_paddle, move_paddle, handle_paddle_drag};
use ball::{setup_balls, detect_ball_collision, maintain_ball_speed};
use walls::{spawn_walls};
use bricks::{setup_formation, update_brick_appearance, update_breakable_timers};

//TODO: Learn what custom plugins are and use them
//TODO: Differentiate the Brick entity with the breakables entity (naming convention is a bit confusing atm)
//TODO: Remove Ball when it touches floor
//TODO: Check if there's no ball entities loaded, if not then fire a restart game function
//TODO: Add levels inside the JSON which then gets parsed
//TODO: Rewrite the spawning of bricks
//TODO: Change up ball physics

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Ball,
    Wall,
    Bricks,
    Paddle,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .init_resource::<DragState>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                handle_paddle_drag,
                move_paddle,
                detect_ball_collision,
                update_breakable_timers,
                update_brick_appearance,
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
    spawn_paddle(&mut commands, 0.); // player 1
    //spawn_paddle(&mut commands, 300.); // player 2

    // ect. ..
    spawn_walls(&mut commands);
    setup_balls(&mut commands, asset_server);
    setup_formation(&mut commands);
}