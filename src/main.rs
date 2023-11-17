pub mod components;
mod systems;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use components::*;
use systems::*;

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

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(20.0),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)),
    ));

    // Create a moving platform
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 400.0, y: 50.0 })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_xyz(-200.0, 100.0, 0.0),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(200.0, 25.0),
        MovingPlatform {
            direction: 1,
            min_x: -400.0,
            max_x: -200.0,
        },
    ));

    // Create a moving platform
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 400.0, y: 50.0 })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(200.0, 25.0),
        MovingPlatform {
            direction: 1,
            min_x: -200.0,
            max_x: 200.0,
        },
    ));
}
