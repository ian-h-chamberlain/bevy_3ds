//! A plugin to enable console logging back from the 3DS back to the development device.
//! Once initialized, the usual [`bevy::log`] macros can be used to instrument the app.

use bevy::app::prelude::*;
use bevy::ecs::prelude::*;
use ctru::prelude::{Console, Gfx};
use ctru::services::soc::Soc;

#[derive(Default)]
pub struct LogPlugin;

#[derive(Resource)]
struct SocketLogger(Soc);

impl Default for SocketLogger {
    fn default() -> Self {
        let soc = Soc::init().expect("failed to init SOC");
        Self(soc)
    }
}

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SocketLogger>()
            .add_startup_system(log_to_3dslink);
    }
}

fn log_to_3dslink(mut soc: ResMut<SocketLogger>) {
    // TODO: should this ignore failures? Or perhaps configurable behavior?
    soc.0
        .redirect_to_3dslink(true, true)
        .expect("unable to debug output to 3dslink");
}
