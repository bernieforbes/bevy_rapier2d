use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MovingPlatform {
    pub direction: i32,
    pub min_x: f32,
    pub max_x: f32,
}

#[derive(Component, Debug)]
pub struct Ball;
