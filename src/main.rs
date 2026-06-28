use bevy::{prelude::*, math::vec3};

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Paddle;

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2d::default());

    // paddle
    commands.spawn((
        Sprite {
            color: PADDLE_COLOR,
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        Transform {
            translation: vec3(0., PADDLE_START_Y,  0.),
            ..default()
        },
        Paddle,
    ));
}