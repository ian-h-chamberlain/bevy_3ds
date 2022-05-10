use bevy::ecs::event::EventWriter;
use bevy::input::gamepad::{GamepadButtonType, GamepadEventRaw, GamepadEventType};
use ctru::services::hid::{Hid, KeyPad};

use super::GAMEPAD;

// We don't get analog button input, so use "binary" values for press/unpress values.
const PRESSED: f32 = 1.0;
const UNPRESSED: f32 = 0.0;

///
pub fn update_state(hid: &Hid, events: &mut EventWriter<GamepadEventRaw>) {
    for_each_key(hid.keys_down(), |key_pad| {
        if let Some(button) = convert_key(key_pad) {
            events.send(GamepadEventRaw {
                gamepad: GAMEPAD,
                event_type: GamepadEventType::ButtonChanged(button, PRESSED),
            });
        }
    });

    for_each_key(hid.keys_held(), |key_pad| {
        if let Some(button) = convert_key(key_pad) {
            events.send(GamepadEventRaw {
                gamepad: GAMEPAD,
                event_type: GamepadEventType::ButtonChanged(button, PRESSED),
            });
        }
    });

    for_each_key(hid.keys_up(), |key_pad| {
        if let Some(button) = convert_key(key_pad) {
            events.send(GamepadEventRaw {
                gamepad: GAMEPAD,
                event_type: GamepadEventType::ButtonChanged(button, UNPRESSED),
            });
        }
    });
}

fn for_each_key<F: FnMut(KeyPad)>(key_pad: KeyPad, mut f: F) {
    // we have to match everything, in lieu of https://github.com/bitflags/bitflags/issues/28
    for key in [
        KeyPad::KEY_A,
        KeyPad::KEY_B,
        // TODO: map dpad as axis if user specified setting. Or maybe both by default?
        KeyPad::KEY_DDOWN,
        KeyPad::KEY_DLEFT,
        KeyPad::KEY_DRIGHT,
        KeyPad::KEY_DUP,
        KeyPad::KEY_L,
        KeyPad::KEY_R,
        KeyPad::KEY_SELECT,
        KeyPad::KEY_START,
        KeyPad::KEY_X,
        KeyPad::KEY_Y,
        KeyPad::KEY_ZL,
        KeyPad::KEY_ZR,
    ] {
        if key_pad.contains(key) {
            f(key);
        }
    }
}

fn convert_key(key: KeyPad) -> Option<GamepadButtonType> {
    match key {
        KeyPad::KEY_A => Some(GamepadButtonType::East),
        KeyPad::KEY_B => Some(GamepadButtonType::South),
        KeyPad::KEY_DDOWN => Some(GamepadButtonType::DPadDown),
        KeyPad::KEY_DLEFT => Some(GamepadButtonType::DPadLeft),
        KeyPad::KEY_DRIGHT => Some(GamepadButtonType::DPadRight),
        KeyPad::KEY_DUP => Some(GamepadButtonType::DPadUp),
        KeyPad::KEY_L => Some(GamepadButtonType::LeftTrigger),
        KeyPad::KEY_R => Some(GamepadButtonType::RightTrigger),
        KeyPad::KEY_SELECT => Some(GamepadButtonType::Select),
        KeyPad::KEY_START => Some(GamepadButtonType::Start),
        KeyPad::KEY_X => Some(GamepadButtonType::North),
        KeyPad::KEY_Y => Some(GamepadButtonType::West),
        KeyPad::KEY_ZL => Some(GamepadButtonType::LeftTrigger2),
        KeyPad::KEY_ZR => Some(GamepadButtonType::RightTrigger2),
        _ => None,
    }
}
