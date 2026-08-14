use bevy::prelude::*;
use avian2d::prelude::*;
use crate::GameLayer;
use crate::walls::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS};

pub const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
pub const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const PADDLE_HITBOX_PADDING: f32 = 48.0; // for QOL purposes when dragging

#[derive(Component)]
pub struct Paddle;

#[derive(Resource, Default)]
pub struct DragState {
    pub entity: Option<Entity>,
}

pub fn spawn_paddle(
    commands: &mut Commands,
    x: f32,
) {
    commands.spawn((
        Sprite {
            color: PADDLE_COLOR,
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        Transform::from_translation(vec3(x, PADDLE_START_Y,  0.)),
        RigidBody::Kinematic,
        Paddle,
        Collider::rectangle(PADDLE_SIZE.x, PADDLE_SIZE.y),
        CollisionLayers::new(GameLayer::Paddle, [GameLayer::Ball]),
    ));
}

pub fn handle_paddle_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    paddles: Query<(Entity, &Transform), With<Paddle>>,
    mut drag_state: ResMut<DragState>,
) {
    if !mouse_button.pressed(MouseButton::Left) {
        drag_state.entity = None;
    }

    if mouse_button.pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else { return };
        let Some(cursor_pos) = window.cursor_position() else { return };
        let Ok((camera, camera_transform)) = camera_query.single() else { return };
        let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

        for (entity, transform) in &paddles {
            let half = PADDLE_SIZE * 0.5 + Vec2::splat(PADDLE_HITBOX_PADDING);
            let min = transform.translation.truncate() - half;
            let max = transform.translation.truncate() + half;

            if world_pos.x >= min.x && world_pos.x <= max.x && world_pos.y >= min.y && world_pos.y <= max.y {
                info!("DEBUG: moving paddle {:?}", entity);
                drag_state.entity = Some(entity);
                break;
            }
        }
    }
}

pub fn move_paddle(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    drag_state: Res<DragState>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let Some(dragged_entity) = drag_state.entity else { return };
    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    let Ok(mut transform) = query.get_mut(dragged_entity) else { return };

    let mut new_x = world_pos.x;
    new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    transform.translation.x = new_x;
}
