use bevy::prelude::*;
use avian2d::prelude::*;
use rand::prelude::*;
use crate::GameLayer;
use crate::walls::GameWall;

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 250.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5,-0.5);
const BALL_PERFORMANCE_TEST: bool = false;

#[derive(Component)]
pub struct Ball;

pub fn spawn_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    custom_color: Color,
    starting_direction: Vec2,
) {
    let ball_texture = asset_server.load("textures/circle.png");
    commands.spawn((
        Sprite {
            image: ball_texture,
            color: custom_color,
            custom_size: Some(BALL_SIZE),
            ..default()
        },
        Transform::from_translation(BALL_STARTING_POSITION),
        Ball,
        LinearVelocity(BALL_SPEED * starting_direction),
        RigidBody::Dynamic,
        Collider::circle(15.0),
        Restitution::new(1.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.),
        CollisionLayers::new(GameLayer::Ball, [GameLayer::Wall, GameLayer::Paddle]),
    ));
}

pub fn setup_balls(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    if !BALL_PERFORMANCE_TEST {
        spawn_ball(commands, &asset_server, BALL_COLOR, BALL_INITIAL_DIRECTION);
        return;
    }

    let mut rng = rand::rng();

    for _ in 0..10000 {
        let random_angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let random_direction: Vec2 = Vec2::new(random_angle.cos(), random_angle.sin());
        let random_color = Color::srgb(
          rng.random_range(0.0..1.0),
          rng.random_range(0.0..1.0),
          rng.random_range(0.0..1.0),
        );

        spawn_ball(commands, &asset_server, random_color, random_direction);
    }
}

pub fn maintain_ball_speed(
    mut query: Query<&mut LinearVelocity, With<Ball>>,
) {
    for mut velocity in query.iter_mut() {
        let current_speed = velocity.length();
        if current_speed > 0.0 {
            velocity.0 = velocity.0 / current_speed * BALL_SPEED;
        }
    }
}

pub fn detect_ball_collision(
    collisions: Collisions,
    ball_query: Query<Entity, With<Ball>>,
    wall_query: Query<Entity, With<GameWall>>
) {
    for ball in &ball_query {
        for wall in &wall_query {
            if collisions.contains(ball, wall) {
                //print!("ball hit wall wow")
            }
        }
    }
}