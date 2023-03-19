//! Core functionality for running a Bevy app on the 3DS. This plugin initializes
//! some basic 3DS functionality and handles the main application loop, including
//! [`AppExit`].

use bevy::app::AppExit;
use bevy::prelude::*;
use ctru::prelude::*;

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let gfx = Gfx::init().expect("unable to init GFX");
        let apt = Apt::init().expect("failed to init APT");

        app.insert_non_send_resource(gfx)
            .insert_non_send_resource(apt)
            // Check APT and exit system before everything else
            // TODO: reread https://bevyengine.org/news/bevy-0-10/#ecs-schedule-v3
            // and make sure these are scheduled correctly
            .add_system(exit_system.before(CoreSet::PreUpdate))
            // run gfx flush after all other stages
            .add_system(flush_gfx.after(CoreSet::PostUpdate));
    }
}

fn exit_system(apt: NonSend<Apt>, mut exit: EventWriter<AppExit>) {
    if !apt.main_loop() {
        exit.send(AppExit);
    }
}

fn flush_gfx(gfx: NonSend<Gfx>) {
    gfx.flush_buffers();
    gfx.swap_buffers();
    gfx.wait_for_vblank();
}
