use bevy::app::AppExit;
use bevy::log;
use bevy::prelude::*;

use bevy_3ds::input::GAMEPAD;

fn main() {
    ctru::init();

    let mut app = App::new();

    app
        // 3ds stuff needs to be set up first, for gfx, console, etc
        .add_plugin(bevy_3ds::DefaultPlugin)
        // Then logging, so we can see what Bevy is doing
        .insert_resource(log::LogSettings {
            level: log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugin(log::LogPlugin)
        // Base Bevy plugins
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy_3ds::input::InputPlugin)
        // Startup
        .insert_resource(MoveTimer(Timer::from_seconds(0.75, true)))
        .add_startup_system(spawn_player)
        // normal runtime stages
        .add_system(handle_inputs)
        .add_system(move_player.after(handle_inputs))
        // 🚀
        .run();
}

struct MoveTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn clamp_to_window(&mut self) {
        // TODO don't hardcode these values.
        self.x = self.x.clamp(0, 40);
        self.y = self.y.clamp(0, 30);
    }
}

#[derive(Component, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Position { x: 0, y: 0 })
        .insert(Direction::East);
}

fn move_player(
    mut player_pos: Query<(&mut Position, &Direction), With<Player>>,
    mut move_timer: ResMut<MoveTimer>,
    time: Res<Time>,
) {
    move_timer.0.tick(time.delta());
    if move_timer.0.just_finished() {
        let (mut pos, dir) = player_pos.single_mut();

        match dir {
            Direction::North => pos.y -= 1,
            Direction::South => pos.y += 1,
            Direction::East => pos.x += 1,
            Direction::West => pos.x -= 1,
        }

        // TODO this clamp doesn't actually work since the subtracts can overflow
        pos.clamp_to_window();

        log::debug!("Player pos updated to {pos:?}");
    }
}

fn handle_inputs(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut direction: Query<&mut Direction, With<Player>>,
    mut exit_event: EventWriter<AppExit>,
) {
    use bevy::input::gamepad::{
        GamepadButtonType::{DPadDown, DPadLeft, DPadRight, DPadUp, Start},
        GamepadEventType::ButtonChanged,
    };

    let mut direction = direction.single_mut();
    for evt in gamepad_event.iter() {
        if let GamepadEvent {
            gamepad: GAMEPAD,
            event_type: &ButtonChanged(button, value),
        } = evt
        {
            if value > 0.5 {
                match button {
                    DPadUp => {
                        *direction = Direction::North;
                    }
                    DPadDown => {
                        *direction = Direction::South;
                    }
                    DPadLeft => {
                        *direction = Direction::West;
                    }
                    DPadRight => {
                        *direction = Direction::East;
                    }
                    Start => {
                        exit_event.send(AppExit);
                    }
                    _ => continue,
                }
                log::info!("Player direction updated to {direction:?}");
            }
        }
    }
}
