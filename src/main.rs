mod component;
mod system;

use bevy::{core::FixedTimestep, prelude::*};
use bevy_networking_turbulence::NetworkingPlugin;

const NETWORK_PERIOD: f64 = 2.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_state(AppState::InGame)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(system::setup)
        .add_startup_system(system::connect_to_server)
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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(NETWORK_PERIOD))
                .with_system(system::send_packets)
                .with_system(system::handle_packets),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
