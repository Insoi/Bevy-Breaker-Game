use bevy::prelude::*;

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5,-0.5);

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

pub fn spawn_ball() {
    return;
}