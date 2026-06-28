use bevy::prelude::*;

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 8.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5,-25.);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

pub fn spawn_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let ball_texture = asset_server.load("textures/circle.png");
    commands.spawn((
        Sprite {
            image: ball_texture,
            color: BALL_COLOR,
            custom_size: Some(BALL_SIZE),
            ..default()
        },
        Transform {
            translation: BALL_STARTING_POSITION,
            ..default()
        },
        Ball,
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));
}

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time<Fixed>>
) {
    let delta_time: f32 = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x + delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}