//! Core functionality for running a Bevy app on the 3DS. This plugin initializes
//! some basic 3DS functionality and handles the main application loop, including
//! [`AppExit`].

use bevy::app::AppExit;
use bevy::prelude::*;
use ctru::services::Apt;
use ctru::Gfx;

// TODO: are these stages needed? Might reconsider stages especially with stageless PR incoming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum Stage {
    Apt,
    Gfx,
}

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let apt_handle = Apt::init().expect("failed to init APT");
        let gfx_handle = Gfx::init().expect("unable to init GFX");

        app.insert_non_send_resource(gfx_handle)
            .insert_resource(apt_handle)
            // Check APT and exit system before everything else
            .add_stage_before(
                CoreStage::PreUpdate,
                Stage::Apt,
                SystemStage::single(exit_system),
            )
            // run gfx flush after all other stages
            .add_stage_after(CoreStage::Last, Stage::Gfx, SystemStage::single(flush_gfx));
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
