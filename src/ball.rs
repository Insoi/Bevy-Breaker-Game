use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use crate::Collider;

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 300.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5,-0.5);

#[derive(Component)]
pub struct Ball {
    size: Vec2,
}
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
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));
}

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time<Fixed>>
) {
    let delta_time: f32 = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}

pub fn check_ball_collisions(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    collider_query: Query<(&Transform, &Collider), Without<Ball>>,
) {
    for (mut ball_transform, mut ball_velocity, ball) in &mut ball_query {
        // creating an axis-aligned bounding box (what aabb stands for) in 2D space. It's edges are always fully straight and cannot be rotated
        // first parameter is the center point position (in world space)
        // second parameter is basically setting the size in radius of the square from the center point
        let ball_aabb = Aabb2d::new(
            ball_transform.translation.truncate(),
            ball.size / 2.0,
        );

        for (collider_transform, collider) in &collider_query {
            // Create an AABB for the collider
            let collider_aabb = Aabb2d::new(
                collider_transform.translation.truncate(),
                collider.size / 2.0,
            );

            // Check if they overlap
            if ball_aabb.intersects(&collider_aabb) {
                // Calculate the collision depth to figure out which side it hit
                let min_dist = ball_aabb.min.max(collider_aabb.min);
                let max_dist = ball_aabb.max.min(collider_aabb.max);
                let overlap = max_dist - min_dist;

                // Bounce along the axis with the shallowest penetration depth
                if overlap.x < overlap.y {
                    // Hit from left or right side
                    ball_velocity.x *= -1.0;

                    // Push the ball out of the collision to prevent sticking
                    if ball_transform.translation.x < collider_transform.translation.x {
                        ball_transform.translation.x -= overlap.x;
                    } else {
                        ball_transform.translation.x += overlap.x;
                    }
                } else {
                    // Hit from top or bottom side
                    ball_velocity.y *= -1.0;

                    // Push the ball out of the collision to prevent sticking
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