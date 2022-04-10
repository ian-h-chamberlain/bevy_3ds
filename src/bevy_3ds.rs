use bevy_app::{App, AppExit};
use bevy_app::{CoreStage, Plugin};
use bevy_ecs::prelude::*;

use ctru::console::Console;
use ctru::services::hid::{Hid, KeyPad};
use ctru::services::Apt;
use ctru::Gfx;
use owning_ref::OwningHandle;

// TODO: Erase type on the Gfx? Client code might want to use it somehow, I guess?
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
            Box::new(Console::init(gfx.bottom_screen.borrow_mut()))
        });
        let apt_handle = Apt::init().expect("failed to init APT");
        let hid = Hid::init().expect("failed to init HID");

        app.insert_non_send_resource(gfx_console)
            .insert_resource(apt_handle)
            .insert_resource(hid)
            // Check APT and exit system before everything else
            .add_stage_before(CoreStage::First, Stage::Apt, SystemStage::single_threaded())
            .add_system_to_stage(Stage::Apt, exit_system)
            // run gfx flush after all other stages
            .add_stage_after(CoreStage::Last, Stage::Gfx, SystemStage::single_threaded())
            .add_system_to_stage(Stage::Gfx, flush_gfx);
    }
}

fn exit_system(apt: NonSend<Apt>, input: Res<Hid>, mut exit: EventWriter<AppExit>) {
    if !apt.main_loop() {
        exit.send(AppExit);
        return;
    }

    // TODO customization of exit behavior. Maybe the second half of this
    // system isn't really reasonable to have as part of the plugin

    input.scan_input();
    let keys = input.keys_down();
    if keys.contains(KeyPad::KEY_SELECT) {
        exit.send(AppExit);
    }
}

fn flush_gfx(gfx_console: NonSend<GfxAndConsole>) {
    let gfx = gfx_console.as_owner();

    gfx.flush_buffers();
    gfx.swap_buffers();
    gfx.wait_for_vblank();
}
