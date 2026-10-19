#[cfg(target_os = "horizon")]
fn main() {
    use bevy::prelude::*;
    bevy_horizon::init_switch_logging("sr2-nx");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_horizon::HorizonPlugin)
        .add_plugins(sr2_gameplay::PrototypePlugin)
        .run();
}
#[cfg(not(target_os = "horizon"))]
fn main() {
    eprintln!(
        "SR2-NX: host diagnostic only. Build for Horizon using tools/build-switch.sh; no native hardware validation yet."
    );
    std::process::exit(1);
}
