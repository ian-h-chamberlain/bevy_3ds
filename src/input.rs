//! Input handling for the 3DS. The device is treated as a single gamepad and
//! sends native Bevy gamepad events for use with Bevy's [`InputSystem`].

use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent, GamepadInfo};
use bevy::prelude::*;
use ctru::services::Hid;

/// There is only one "gamepad" on the 3DS, so its ID is always zero.
pub const GAMEPAD: Gamepad = Gamepad { id: 0 };

mod button;

#[derive(Default)]
pub struct InputPlugin;

#[derive(Resource)]
struct GamepadInput(Hid);

impl Default for GamepadInput {
    fn default() -> Self {
        let hid = Hid::init().expect("failed to init HID");
        Self(hid)
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamepadInput>()
            // TODO: reread https://bevyengine.org/news/bevy-0-10/#ecs-schedule-v3
            // and make sure these are scheduled correctly
            .add_startup_system(gamepad_startup_system.before(StartupSet::PreStartup))
            .add_system(gamepad_update_system.before(bevy::input::InputSystem));
    }
}

fn gamepad_startup_system(mut events: EventWriter<GamepadEvent>) {
    events.send(
        GamepadConnectionEvent::new(
            GAMEPAD,
            GamepadConnection::Connected(GamepadInfo {
                name: String::from("3DS Gamepad"),
            }),
        )
        .into(),
    );
}

fn gamepad_update_system(hid: Res<GamepadInput>, mut events: EventWriter<GamepadEvent>) {
    hid.0.scan_input();
    button::update_state(&hid.0, &mut events);
}
