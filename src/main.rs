mod component;
mod system;

use bevy::{core::FixedTimestep, prelude::*};

const GAP: f32 = 300.;
const FACTOR: f32 = 5.;
const UPPER_LIMIT: f32 = GAP * FACTOR;
const LOWER_LIMIT: f32 = -UPPER_LIMIT;
const BUILDING_WIDTH_RANGE: (f32, f32) = (60., 170.);
const HEIGHT_OFFSET_RANGE: f32 = 50.;
const BUILDING_BASE_HEIGHT: f32 = 1000.;
const HORIZON: f32 = -600.;
const SCROLL_SPEED: f32 = 400.;
const SQUARE_SIZE: f32 = 30.;
const GRAVITY: f32 = -1500.;
const TIME_STEP: f64 = 1. / 240.;
const JUMP_FORCE: f32 = 600.;
const FASTFALL_FORCE: f32 = -400.;
const LANDING_TOLERANCE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(system::building_translation)
                .with_system(system::reset_building_oob)
                .with_system(system::gravity)
                .with_system(system::apply_velocity)
                .with_system(
                    system::collision_detection
                        .chain(system::square_landing)
                        .chain(system::loose_condition),
                )
                .with_system(system::jump_or_fastfall_on_mouse_click),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let mut start = UPPER_LIMIT - GAP;

    while start >= LOWER_LIMIT {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(start, HORIZON + HEIGHT_OFFSET_RANGE, 0.),
                    // +1. to prevent collision on the building side
                    scale: Vec3::new(GAP + 1., BUILDING_BASE_HEIGHT, 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(component::Building);

        start -= GAP;
    }

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    0.,
                    HORIZON + HEIGHT_OFFSET_RANGE + BUILDING_BASE_HEIGHT / 2. + SQUARE_SIZE / 2.,
                    0.,
                ),
                scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(component::Velocity(0.))
        .insert(component::IsOnFloor(false));
}
