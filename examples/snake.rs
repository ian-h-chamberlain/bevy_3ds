//! A simple Snake clone to show off the basic features of `bevy_3ds`, without
//! rendering anything using graphics APIs.

// TODO: maybe snake is not the best option but :shrug:

use bevy::app::AppExit;
use bevy::input::gamepad::GamepadButtonChangedEvent;
use bevy::log;
use bevy::prelude::*;

use bevy_3ds::input::GAMEPAD;

fn main() {
    ctru::use_panic_handler();

    let mut app = App::new();

    app
        // Add default bevy_3ds plugins
        .add_plugins(
            bevy_3ds::DefaultPlugins
                // Configure logging to debug level
                .set(log::LogPlugin {
                    level: log::Level::DEBUG,
                    ..default()
                }),
        )
        // Startup systems
        .insert_resource(MoveTimer(Timer::from_seconds(0.75, TimerMode::Repeating)))
        .add_startup_system(spawn_player)
        // Normal runtime systems
        .add_system(handle_inputs)
        .add_system(move_player.after(handle_inputs))
        // 🚀
        .run();
}

#[derive(Resource)]
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
    commands.spawn((Player, Position { x: 0, y: 0 }, Direction::East));
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
    mut gamepad_event: EventReader<GamepadButtonChangedEvent>,
    mut direction: Query<&mut Direction, With<Player>>,
    mut exit_event: EventWriter<AppExit>,
) {
    use bevy::input::gamepad::GamepadButtonType::{DPadDown, DPadLeft, DPadRight, DPadUp, Start};

    let mut direction = direction.single_mut();
    for &GamepadButtonChangedEvent {
        button_type,
        value,
        gamepad,
    } in gamepad_event.iter()
    {
        assert_eq!(gamepad, GAMEPAD);

        if value > 0.5 {
            match button_type {
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
