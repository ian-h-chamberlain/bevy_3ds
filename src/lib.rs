// TODO: separate crate, suitable for public consumption. Could be a git dep
// in the meantime...

use std::sync::{Arc, Mutex};

use bevy::app::AppExit;
use bevy::prelude::*;

use ctru::console::Console;
use ctru::services::Apt;
use ctru::Gfx;

pub mod graphics;
pub mod input;

use graphics::Graphics;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum Stage {
    Apt,
    Gfx,
}

#[derive(Default)]
pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, app: &mut App) {
        let apt_handle = Apt::init().expect("failed to init APT");

        app.init_non_send_resource::<Graphics>()
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

fn flush_gfx(graphics: NonSend<Graphics>) {
    let gfx = graphics.gfx();

    gfx.flush_buffers();
    gfx.swap_buffers();
    gfx.wait_for_vblank();
}
