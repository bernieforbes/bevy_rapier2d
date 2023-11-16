use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct MovingPlatform {
    pub direction: i32,
    pub min_x: f32,
    pub max_x: f32,
}

pub fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

pub fn move_platform(mut query: Query<(&mut Transform, &mut MovingPlatform)>) {
    for (mut transform, mut moving_platform) in query.iter_mut() {
        if moving_platform.direction != 1 && moving_platform.direction != -1 {
            moving_platform.direction = 1;
        }

        transform.translation.x += 2.0 * moving_platform.direction as f32;

        if transform.translation.x < moving_platform.min_x
            || transform.translation.x > moving_platform.max_x
        {
            moving_platform.direction *= -1;
        }
    }
}
