use bevy::app::AppExit;
use bevy::prelude::*;
use ctru::services::soc::Soc;
use ctru::services::Apt;
use ctru::Gfx;

pub mod input;

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
        let soc_handle = Soc::init().expect("failed to init SOC");
        let gfx_handle = Gfx::init().expect("unable to init gfx");

        app.insert_non_send_resource(gfx_handle)
            .insert_resource(apt_handle)
            .insert_resource(soc_handle)
            .add_startup_system(debug_output_to_3dslink)
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

// TODO: maybe #[cfg] this as a debug feature or something. Or just a plugin
fn debug_output_to_3dslink(mut soc: ResMut<Soc>) {
    soc.redirect_to_3dslink(true, true)
        .expect("unable to debug output to 3dslink");
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
