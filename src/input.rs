use bevy::app::{App, CoreStage, Plugin};
use bevy::ecs::{
    event::EventWriter,
    system::{IntoExclusiveSystem, Res},
};
use bevy::input::{
    gamepad::{Gamepad, GamepadEventRaw},
    InputPlugin as BevyInputPlugin, InputSystem,
};
use bevy::prelude::{ExclusiveSystemDescriptorCoercion, GamepadEventType, StartupStage};
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
            .add_plugin(BevyInputPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, startup_system)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                update_system.exclusive_system().before(InputSystem),
            );
    }
}

fn startup_system(mut events: EventWriter<GamepadEventRaw>) {
    events.send(GamepadEventRaw {
        gamepad: GAMEPAD,
        event_type: GamepadEventType::Connected,
    });
}

fn update_system(hid: Res<Hid>, mut events: EventWriter<GamepadEventRaw>) {
    hid.scan_input();
    button::update_state(&*hid, &mut events);
}
