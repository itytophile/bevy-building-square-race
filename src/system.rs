use crate::component;
use bevy::prelude::*;

pub fn building_translation(mut query: Query<&mut Transform, With<component::Building>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= crate::SCROLL_SPEED * crate::TIME_STEP as f32;
    }
}

pub fn reset_building_oob(mut query: Query<&mut Transform, With<component::Building>>) {
    for mut transform in query
        .iter_mut()
        .filter(|transform| transform.translation.x < crate::LOWER_LIMIT)
    {
        transform.translation.x = crate::UPPER_LIMIT;
        transform.translation.y = crate::get_building_y();
        transform.scale.x = crate::get_building_width();
    }
}

pub fn gravity(mut query: Query<&mut component::Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0 += crate::GRAVITY * crate::TIME_STEP as f32
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &component::Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.0 * crate::TIME_STEP as f32
    }
}
