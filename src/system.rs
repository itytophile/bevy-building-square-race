use crate::{
    component, BUILDING_WIDTH_RANGE, FASTFALL_FORCE, GRAVITY, HEIGHT_OFFSET_RANGE, HORIZON,
    JUMP_FORCE, LANDING_TOLERANCE, LOWER_LIMIT, SCROLL_SPEED, TIME_STEP, UPPER_LIMIT,
};
use bevy::{
    app::AppExit,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

pub fn building_translation(mut query_building: Query<&mut Transform, With<component::Building>>) {
    for mut transform in query_building.iter_mut() {
        transform.translation.x -= SCROLL_SPEED * TIME_STEP as f32;
    }
}

pub fn reset_building_oob(mut query_building: Query<&mut Transform, With<component::Building>>) {
    for mut transform in query_building
        .iter_mut()
        .filter(|transform| transform.translation.x < LOWER_LIMIT)
    {
        transform.translation.x = UPPER_LIMIT;
        transform.translation.y =
            HORIZON + (1. - 2. * rand::thread_rng().gen::<f32>()) * HEIGHT_OFFSET_RANGE;
        transform.scale.x = BUILDING_WIDTH_RANGE.0
            + rand::thread_rng().gen::<f32>() * (BUILDING_WIDTH_RANGE.1 - BUILDING_WIDTH_RANGE.0);
    }
}

pub fn gravity(mut query_square: Query<&mut component::Velocity>) {
    let mut velocity = query_square.single_mut();

    velocity.0 += GRAVITY * TIME_STEP as f32
}

pub fn apply_velocity(mut query_square: Query<(&mut Transform, &component::Velocity)>) {
    let (mut transform, velocity) = query_square.single_mut();
    transform.translation.y += velocity.0 * TIME_STEP as f32
}

pub fn collision_detection(
    query_square: Query<&Transform, With<component::Velocity>>,
    query_building: Query<&Transform, With<component::Building>>,
) -> Option<(Collision, Transform)> {
    let &Transform {
        translation: translation_square,
        scale: scale_square,
        ..
    } = query_square.single();

    for &transform in query_building.iter() {
        let Transform {
            translation, scale, ..
        } = transform;
        // We prioritize the true position before using
        // the landing tolerance
        if let Some(collision) = collide(
            translation_square,
            scale_square.truncate(),
            translation,
            scale.truncate(),
        ) {
            return Some((collision, transform));
        } else {
            // we copy the translation to not affect the next iterations
            let mut translation_square = translation_square;
            translation_square.y -= LANDING_TOLERANCE;
            if let Some(Collision::Top) = collide(
                translation_square,
                scale_square.truncate(),
                translation,
                scale.truncate(),
            ) {
                return Some((Collision::Top, transform));
            }
        }
    }

    None
}

pub fn square_landing(
    In(collision_info): In<Option<(Collision, Transform)>>,
    mut query_square: Query<(
        &mut Transform,
        &mut component::Velocity,
        &mut component::IsOnFloor,
    )>,
) -> Option<(Collision, Transform)> {
    let (mut transform_square, mut velocity_square, mut is_on_floor) = query_square.single_mut();

    // if falling
    if velocity_square.0 <= 0. {
        if let Some((
            ref collision,
            Transform {
                translation, scale, ..
            },
        )) = collision_info
        {
            if let Collision::Top = collision {
                is_on_floor.0 = true;
                velocity_square.0 = 0.;
                transform_square.translation.y =
                    transform_square.scale.y / 2. + scale.y / 2. + translation.y
            }
        } else {
            // if no collision and falling then not on floor
            is_on_floor.0 = false
        }
    }

    collision_info
}

pub fn loose_condition(
    In(collision_info): In<Option<(Collision, Transform)>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Some((Collision::Left, ..)) = collision_info {
        app_exit_event_writer.send(AppExit)
    }
}

pub fn jump_or_fastfall_on_mouse_click(
    mouse_button: Res<Input<MouseButton>>,
    mut query_square: Query<(&mut component::Velocity, &mut component::IsOnFloor)>,
) {
    let (mut velocity, mut is_on_floor) = query_square.single_mut();

    if mouse_button.just_pressed(MouseButton::Left) {
        if is_on_floor.0 {
            velocity.0 = JUMP_FORCE;
            is_on_floor.0 = false;
        } else if velocity.0 > FASTFALL_FORCE {
            velocity.0 = FASTFALL_FORCE
        }
    }
}
