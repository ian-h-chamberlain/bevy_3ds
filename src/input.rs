//! Input handling for the 3DS. The device is treated as a single gamepad and
//! sends native Bevy gamepad events for use with Bevy's [`InputSystem`].

use bevy::input::gamepad::GamepadEventRaw;
use bevy::prelude::*;
use ctru::services::Hid;

/// There is only one "gamepad" on the 3DS, so its ID is always zero.
pub const GAMEPAD: Gamepad = Gamepad { id: 0 };

mod button;

#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        let hid = Hid::init().expect("failed to init HID");

        app.insert_resource(hid)
            .add_startup_system_to_stage(StartupStage::PreStartup, gamepad_startup_system)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                gamepad_update_system.before(bevy::input::InputSystem),
            );
    }
}

fn gamepad_startup_system(mut events: EventWriter<GamepadEventRaw>) {
    events.send(GamepadEventRaw::new(GAMEPAD, GamepadEventType::Connected));
}

fn gamepad_update_system(hid: Res<Hid>, mut events: EventWriter<GamepadEventRaw>) {
    hid.scan_input();
    button::update_state(&*hid, &mut events);
}
