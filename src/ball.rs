use bevy::prelude::*;
use avian2d::prelude::*;
use rand::prelude::*;
use crate::bricks::Breakable;
use crate::GameLayer;
use crate::paddle::{Paddle, PADDLE_SIZE};

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 300.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5,-0.5);
const START_WITH_MULTIPLE_BALLS: bool = false;
const BALL_COUNT: i32 = 5000;

const MAX_BOUNCE_ANGLE: f32 = 45.0;
const MIN_BALL_VELOCITY_ANGLE: f32 = 8.0;
const CURVE_POWER: f32 = 2.0;

#[derive(Component)]
pub struct Ball;

pub fn spawn_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    custom_color: Color,
    starting_direction: Vec2,
    spawn_position: Vec3,
) {
    let ball_texture = asset_server.load("textures/circle.png");
    commands.spawn((
        Sprite {
            image: ball_texture,
            color: custom_color,
            custom_size: Some(BALL_SIZE),
            ..default()
        },
        Transform::from_translation(spawn_position),
        Ball,
        LinearVelocity(BALL_SPEED * starting_direction),
        RigidBody::Dynamic,
        Collider::circle(15.0),
        Restitution::new(1.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.),
        CollisionLayers::new(GameLayer::Ball, [GameLayer::Wall, GameLayer::Paddle, GameLayer::Bricks]),
    ));
}

pub fn setup_balls(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    if !START_WITH_MULTIPLE_BALLS {
        spawn_ball(commands, &asset_server, BALL_COLOR, BALL_INITIAL_DIRECTION, BALL_STARTING_POSITION);
        return;
    }

    let mut rng = rand::rng();
    let slice_size = std::f32::consts::TAU / BALL_COUNT as f32;
    let base_offset: f32 = rng.random_range(0.0..std::f32::consts::TAU);

    let spacing = BALL_SIZE.x * 1.5;
    let total_width = spacing * (BALL_COUNT as f32 - 1.0);
    let start_x = -total_width / 2.0;

    for i in 0..BALL_COUNT {
        let slice_start = base_offset + i as f32 * slice_size;
        let random_angle: f32 = rng.random_range(slice_start..slice_start + slice_size);
        let random_direction: Vec2 = Vec2::new(random_angle.cos(), random_angle.sin());
        let _spawn_position = BALL_STARTING_POSITION + Vec3::new(start_x + i as f32 * spacing, 0.0, 0.0);

        let random_color = Color::srgb(
          rng.random_range(0.0..1.0),
          rng.random_range(0.0..1.0),
          rng.random_range(0.0..1.0),
        );

        spawn_ball(commands, &asset_server, random_color, random_direction, BALL_STARTING_POSITION);
    }
}

fn clamp_ball_angle(direction: Vec2) -> Vec2 {
    let angle = direction.y.atan2(direction.x);
    let degree = angle.to_degrees();

    let mut degree = degree.rem_euclid(360.0);
    let min_angle = MIN_BALL_VELOCITY_ANGLE;

    for &axis in &[0.0, 90.0, 180.0, 270.0, 360.0] {
        let difference = degree - axis;

        if difference.abs() < min_angle {
            let push = min_angle - difference.abs();
            degree += push * difference.signum().max(if difference == 0.0 { 1.0 } else { difference.signum() });
        }
    }

    let clamped_angle = degree.to_radians();
    Vec2::new(clamped_angle.cos(), clamped_angle.sin())
}

pub fn maintain_ball_speed(
    mut query: Query<&mut LinearVelocity, With<Ball>>,
) {
    for mut velocity in query.iter_mut() {
        let current_speed = velocity.length();

        if current_speed > 0.0 {
            let direction = velocity.0 / current_speed;
            let clamped_direction = clamp_ball_angle(direction);
            velocity.0 = clamped_direction * BALL_SPEED;
        }
    }
}

pub fn detect_ball_collision(
    mut commands: Commands,
    collisions: Collisions,
    mut ball_query: Query<(Entity, &Transform, &mut LinearVelocity), With<Ball>>,
    mut breakable_query: Query<(Entity, &mut Breakable)>,
    paddle_query: Query<(Entity, &Transform), With<Paddle>>,
) {
    for (ball_entity, ball_transform, mut velocity) in &mut ball_query {
        for (entity, mut breakable) in breakable_query.iter_mut() {
            if collisions.contains(ball_entity, entity) && breakable.collision_cooldown.is_finished() {
                breakable.health -= 1;
                breakable.collision_cooldown.reset();

                if breakable.health <= 0 {
                    commands.entity(entity).despawn();
                }
            }
        }

        for (paddle_entity, paddle_transform) in &paddle_query {
            if collisions.contains(ball_entity, paddle_entity) {
                let offset = ((ball_transform.translation.x - paddle_transform.translation.x)
                    / (PADDLE_SIZE.x / 2.0))
                    .clamp(-1.0, 1.0);

                let max_angle_rad = MAX_BOUNCE_ANGLE.to_radians();
                let angle = offset.signum() * offset.abs().powf(CURVE_POWER) * max_angle_rad;

                let direction = Vec2::new(angle.sin(), angle.cos());
                let speed = velocity.length();

                velocity.0 = direction * speed;
            }
        }
    }
}

