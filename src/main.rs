use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(
            Startup,
            (setup_graphics, setup_physics, print_ball_altitude),
        )
        .add_systems(Update, move_platform)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Debug)]
pub struct MovingPlatform {
    direction: i32,
    min_x: f32,
    max_x: f32,
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)),
    ));

    // Create a moving platform
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(200.0, 25.0),
        TransformBundle::from(Transform::from_xyz(-200.0, 100.0, 0.0)),
        MovingPlatform {
            direction: 1,
            min_x: -400.0,
            max_x: -200.0,
        },
    ));

    // Create a moving platform
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(200.0, 25.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        MovingPlatform {
            direction: 1,
            min_x: -200.0,
            max_x: 200.0,
        },
    ));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn move_platform(mut query: Query<(&mut Transform, &mut MovingPlatform)>) {
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
