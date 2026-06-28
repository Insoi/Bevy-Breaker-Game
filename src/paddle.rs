use bevy::prelude::*;

pub const PADDLE_START_Y: f32 = -150.0;
pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
pub const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
pub const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Movement {
    left_key: KeyCode,
    right_key: KeyCode,
}

pub fn spawn_paddle(
    commands: &mut Commands,
    x: f32,
    left_key: KeyCode,
    right_key: KeyCode,
) {
    commands.spawn((
        Sprite {
            color: PADDLE_COLOR,
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        Transform {
            translation: vec3(x, PADDLE_START_Y,  0.),
            ..default()
        },
        Paddle,
        Movement { left_key, right_key },
    ));
}

pub fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut query: Query<(&Movement, &mut Transform), With<Paddle>>,
) {
    for (movement, mut transform) in &mut query {
        let mut direction: f32 = 0.0;
        if input.pressed(movement.left_key) {
            direction -= 1.0;
        }
        if input.pressed(movement.right_key) {
            direction += 1.0;
        }

        let new_x = direction * PADDLE_SPEED * time.delta_secs();
        transform.translation.x += new_x;
    }
}