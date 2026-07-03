use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use crate::Collider;
use rand::prelude::*;

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_HALF_SIZE: Vec2 = Vec2::new(15.0, 15.0);
const BALL_SPEED: f32 = 250.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5,-0.5);
const BALL_PERFORMANCE_TEST: bool = true;

#[derive(Component)]
pub struct Ball;
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

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
        Transform {
            translation: BALL_STARTING_POSITION,
            ..default()
        },
        Ball,
        Velocity(BALL_SPEED * starting_direction),
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

    for _ in 0..10_000 {
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

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
    time: Res<Time<Fixed>>
) {
    let delta_time: f32 = time.delta_secs();

    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    });
}

pub fn check_ball_collisions(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    collider_query: Query<(&Transform, &Collider), Without<Ball>>,
) {
    for (mut ball_transform, mut ball_velocity, _) in &mut ball_query {
        // creating an axis-aligned bounding box (what aabb stands for) in 2D space. It's edges are always fully straight and cannot be rotated
        // first parameter is the center point position (in world space)
        // second parameter is basically setting the size in radius of the square from the center point
        let ball_aabb = Aabb2d::new(
            ball_transform.translation.truncate(),
            BALL_HALF_SIZE,
        );

        for (collider_transform, collider) in &collider_query {
            // creating an AABB for the collider
            let collider_aabb = Aabb2d::new(
                collider_transform.translation.truncate(),
                collider.size / 2.0,
            );

            // overlap check
            if ball_aabb.intersects(&collider_aabb) {
                // collision depth to find out which side was hit
                let min_dist = ball_aabb.min.max(collider_aabb.min);
                let max_dist = ball_aabb.max.min(collider_aabb.max);
                let overlap = max_dist - min_dist;

                // bouncing along the axis
                if overlap.x < overlap.y {
                    // hit from the left or right side
                    ball_velocity.x *= -1.0;

                    // pushing  ball out of the collision to prevent it sticking on
                    if ball_transform.translation.x < collider_transform.translation.x {
                        ball_transform.translation.x -= overlap.x;
                    } else {
                        ball_transform.translation.x += overlap.x;
                    }
                } else {
                    // hit from top or bottom side
                    ball_velocity.y *= -1.0;

                    // pushing  ball out of the collision to prevent it sticking on
                    if ball_transform.translation.y < collider_transform.translation.y {
                        ball_transform.translation.y -= overlap.y;
                    } else {
                        ball_transform.translation.y += overlap.y;
                    }
                }
            }
        }
    }
}