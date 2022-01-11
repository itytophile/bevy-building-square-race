use crate::{
    component, event, BUILDING_WIDTH_RANGE, FASTFALL_FORCE, GRAVITY, HEIGHT_OFFSET_RANGE, HORIZON,
    JUMP_FORCE, LANDING_TOLERANCE, LOWER_LIMIT, SCROLL_SPEED, TIME_STEP, UPPER_LIMIT,
};
use bevy::{
    app::AppExit,
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

pub fn building_translation(mut query: Query<&mut Transform, With<component::Building>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= SCROLL_SPEED * TIME_STEP as f32;
    }
}

pub fn reset_building_oob(mut query: Query<&mut Transform, With<component::Building>>) {
    for mut transform in query
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

pub fn gravity(mut query: Query<(&mut component::Velocity, &component::IsOnFloor)>) {
    let (mut velocity, is_on_floor) = query.single_mut();

    if !is_on_floor.0 {
        velocity.0 += GRAVITY * TIME_STEP as f32
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &component::Velocity)>) {
    let (mut transform, velocity) = query.single_mut();
    transform.translation.y += velocity.0 * TIME_STEP as f32
}

pub fn collision_detection(
    mut collision_event_writer: EventWriter<event::BuildingCollision>,
    query_square: Query<&Transform, With<component::Velocity>>,
    query_building: Query<&Transform, With<component::Building>>,
) {
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
            collision_event_writer.send(event::BuildingCollision(collision, transform));
            break;
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
                collision_event_writer.send(event::BuildingCollision(Collision::Top, transform));
                break;
            }
        }
    }
}

pub fn square_landing(
    mut collision_event_reader: EventReader<event::BuildingCollision>,
    mut query_square: Query<(
        &mut Transform,
        &mut component::Velocity,
        &mut component::IsOnFloor,
    )>,
) {
    let (mut transform_square, mut velocity_square, mut is_on_floor) = query_square.single_mut();

    if !is_on_floor.0 && velocity_square.0 <= 0. {
        for event::BuildingCollision(
            collision,
            Transform {
                translation, scale, ..
            },
        ) in collision_event_reader.iter()
        {
            if let Collision::Top = collision {
                is_on_floor.0 = true;
                velocity_square.0 = 0.;
                transform_square.translation.y =
                    transform_square.scale.y / 2. + scale.y / 2. + translation.y
            }
        }
    }
}

pub fn loose_condition(
    mut collision_event_reader: EventReader<event::BuildingCollision>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for event::BuildingCollision(collision, ..) in collision_event_reader.iter() {
        if let Collision::Left = collision {
            app_exit_event_writer.send(AppExit)
        }
    }
}

pub fn jump_or_fastfall_on_mouse_click(
    mut click_event_reader: EventReader<MouseButtonInput>,
    mut query_velocity: Query<(&mut component::Velocity, &mut component::IsOnFloor)>,
) {
    let (mut velocity, mut is_on_floor) = query_velocity.single_mut();

    for event in click_event_reader.iter() {
        if let MouseButtonInput {
            state: ElementState::Pressed,
            ..
        } = event
        {
            if is_on_floor.0 {
                velocity.0 = JUMP_FORCE;
                is_on_floor.0 = false;
            } else if velocity.0 > FASTFALL_FORCE {
                velocity.0 = FASTFALL_FORCE
            }
        }
    }
}

/*
Don't work with Res<Input<MouseButton>>

pub fn jump_or_fastfall_on_mouse_click(
    mouse_button: Res<Input<MouseButton>>,
    mut query_velocity: Query<(&mut component::Velocity, &mut component::IsOnFloor)>,
) {
    let (mut velocity, mut is_on_floor) = query_velocity.single_mut();

    if mouse_button.just_pressed(MouseButton::Left) {
        if is_on_floor.0 {
            velocity.0 = JUMP_FORCE;
            is_on_floor.0 = false;
        } else if velocity.0 > FASTFALL_FORCE {
            velocity.0 = FASTFALL_FORCE
        }
    }
}
*/
