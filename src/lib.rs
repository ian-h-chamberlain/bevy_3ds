// TODO: separate crate, suitable for public consumption. Could be a git dep
// in the meantime...

use bevy::app::AppExit;
use bevy::prelude::*;

use ctru::console::Console;
use ctru::services::Apt;
use ctru::Gfx;
use owning_ref::OwningHandle;

pub mod gfx;
pub mod input;

pub type GfxAndConsole<'a> = OwningHandle<Box<Gfx>, Box<Console<'a>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum Stage {
    Apt,
    Gfx,
}

#[derive(Default)]
pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, app: &mut App) {
        let gfx = Gfx::init().expect("failed to init gfx");
        let gfx_console: GfxAndConsole = OwningHandle::new_with_fn(Box::new(gfx), |gfx| {
            let gfx = unsafe { &*gfx };
            let mut screen = gfx.top_screen.borrow_mut();
            screen.set_wide_mode(true);
            Box::new(Console::non_flushing(screen))
        });
        let apt_handle = Apt::init().expect("failed to init APT");

        app.insert_non_send_resource(gfx_console)
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

fn flush_gfx(gfx_console: NonSend<GfxAndConsole>) {
    let gfx = gfx_console.as_owner();

    gfx.flush_buffers();
    gfx.swap_buffers();
    gfx.wait_for_vblank();
}
