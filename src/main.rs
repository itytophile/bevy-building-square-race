mod component;
mod system;

use bevy::prelude::*;

const GAP: f32 = 300.;
const FACTOR: u32 = 5;
const UPPER_LIMIT: f32 = GAP * FACTOR as f32;
const LOWER_LIMIT: f32 = -UPPER_LIMIT;
const BUILDING_WIDTH_RANGE: (f32, f32) = (60., 170.);
const HEIGHT_OFFSET_RANGE: f32 = 50.;
const BUILDING_BASE_HEIGHT: f32 = 1000.;
const HORIZON: f32 = -600.;
const SCROLL_SPEED: f32 = 400.;
const SQUARE_SIZE: f32 = 30.;
const GRAVITY: f32 = -1500.;
const JUMP_FORCE: f32 = 600.;
const FASTFALL_FORCE: f32 = -400.;
const LANDING_TOLERANCE: f32 = 10.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::InGame)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(system::setup)
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(system::start_position_setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(system::building_translation)
                .with_system(system::reset_building_oob)
                .with_system(system::gravity)
                .with_system(system::apply_velocity)
                .with_system(
                    system::collision_detection
                        .chain(system::square_landing)
                        .chain(system::loose_condition),
                )
                .with_system(system::jump_or_fastfall_on_click),
        )
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(system::resume_on_click))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
