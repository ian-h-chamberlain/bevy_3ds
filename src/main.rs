use std::sync::Once;
use std::time::Duration;

use bevy::app::AppExit;
use bevy::log;
use bevy::prelude::*;

use bevy_3ds::graphics::{ConsoleId, Graphics};
use bevy_3ds::input::GAMEPAD;
use ctru::console::Console;
// TODO: rename once move to real crate
use fourd3d3d3 as bevy_3ds;

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
        .add_startup_system(init_consoles)
        // normal runtime stages
        .add_system(handle_inputs)
        .add_system(move_player.after(handle_inputs))
        .add_system_to_stage(
            CoreStage::PostUpdate,
            draw_player.exclusive_system().at_end(),
        )
        // 🚀
        .run();
}

#[derive(Component)]
struct MainConsole;

#[derive(Component)]
struct DrawConsole;

fn init_consoles(mut commands: Commands, mut gfx: NonSendMut<Graphics>) {
    let draw_console = gfx.add_console_with(|gfx| {
        let bottom_screen = gfx.bottom_screen.borrow_mut();
        Console::non_flushing(bottom_screen)
    });

    commands.spawn().insert(DrawConsole).insert(draw_console);

    let main_console = gfx.add_console_with(|gfx| {
        let mut top_screen = gfx.top_screen.borrow_mut();
        top_screen.set_wide_mode(true);
        Console::non_flushing(top_screen)
    });

    log::info!("initialized draw + main consoles");

    commands.spawn().insert(MainConsole).insert(main_console);
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
        match *evt {
            GamepadEvent(GAMEPAD, ButtonChanged(DPadUp, v)) if v > 0.5 => {
                *direction = Direction::North;
            }
            GamepadEvent(GAMEPAD, ButtonChanged(DPadDown, v)) if v > 0.5 => {
                *direction = Direction::South;
            }
            GamepadEvent(GAMEPAD, ButtonChanged(DPadLeft, v)) if v > 0.5 => {
                *direction = Direction::West;
            }
            GamepadEvent(GAMEPAD, ButtonChanged(DPadRight, v)) if v > 0.5 => {
                *direction = Direction::East;
            }
            GamepadEvent(GAMEPAD, ButtonChanged(Start, value)) if value > 0.5 => {
                exit_event.send(AppExit);
            }
            _ => continue,
        }

        log::info!("Player direction updated to {direction:?}");
    }
}

fn draw_player(
    mut graphics: NonSendMut<Graphics>,
    player: Query<(&Position, &Direction), With<Player>>,
    draw_console: Query<&ConsoleId, With<DrawConsole>>,
    main_console: Query<&ConsoleId, With<MainConsole>>,
) {
    let (position, direction) = player.single();

    graphics.with_console(*draw_console.single(), |console| {
        console.select();
        // TODO we can't use clear() because it forces a gfx flush
        // console.clear();
    });

    println!("\x1b[0;0H");
    for _ in 0..position.y {
        println!("{:>39}", "");
    }

    let char = match direction {
        Direction::North => '^',
        Direction::South => 'v',
        Direction::East => '>',
        Direction::West => '<',
    };

    print!("{char:>0$}", position.x);

    graphics.with_console(*main_console.single(), |console| {
        console.select();
    });
}
