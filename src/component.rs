use bevy::prelude::Component;

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct IsOnFloor(pub bool);
