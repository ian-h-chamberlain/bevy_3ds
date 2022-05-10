//! A plugin to enable console logging back from the 3DS back to the development device.
//! Once initialized, the usual [`bevy::log`] macros can be used to instrument the app.

use bevy::app::prelude::*;
use bevy::ecs::prelude::*;
use ctru::services::soc::Soc;

#[derive(Default)]
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.world
            .get_resource_or_insert_with(|| Soc::init().expect("failed to init SOC"));
        app.add_startup_system(log_to_3dslink);
    }
}

fn log_to_3dslink(mut soc: ResMut<Soc>) {
    // TODO: should this ignore failures? Or perhaps configurable behavior?
    soc.redirect_to_3dslink(true, true)
        .expect("unable to debug output to 3dslink");
}
