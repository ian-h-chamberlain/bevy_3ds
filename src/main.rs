use std::time::Duration;

use bevy_app::{App, AppExit, CoreStage, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;
use ctru::console::Console;
use ctru::services::hid::{Hid, KeyPad};
use ctru::services::Apt;
use ctru::Gfx;
use owning_ref::OwningHandle;

type GfxAndConsole<'a> = OwningHandle<Box<Gfx>, Box<Console<'a>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum Stage {
    Apt,
    Gfx,
}

fn main() {
    ctru::init();

    let gfx = Gfx::init().expect("failed to init gfx");
    let gfx_console: GfxAndConsole = OwningHandle::new_with_fn(Box::new(gfx), |gfx| {
        let gfx = unsafe { &*gfx };
        Box::new(Console::init(gfx.bottom_screen.borrow_mut()))
    });
    let apt = Apt::init().expect("failed to init APT");
    let hid = Hid::init().expect("failed to init HID");

    App::new()
        // Base Bevy plugins
        .add_plugin(CorePlugin)
        .add_plugin(ScheduleRunnerPlugin)
        // Global resources
        .insert_non_send_resource(gfx_console)
        .insert_resource(apt)
        .insert_resource(hid)
        .insert_resource(ScheduleRunnerSettings {
            run_mode: bevy_app::RunMode::Loop {
                // TODO: this... doesn't seem to work right.
                wait: Some(Duration::from_millis(10_000)),
            },
        })
        // setup people entities/components
        .add_startup_system(add_people)
        // Check APT and exit system before everything else
        .add_stage_before(CoreStage::First, Stage::Apt, SystemStage::single_threaded())
        .add_system_to_stage(Stage::Apt, exit_system)
        // normal runtime stages
        .add_event::<String>()
        .add_system(greet_people)
        .add_system(hello_world)
        .add_system(printer)
        // run gfx flush after all other stages
        .add_stage_after(CoreStage::Last, Stage::Gfx, SystemStage::single_threaded())
        .add_system_to_stage(Stage::Gfx, flush_gfx)
        // 🚀
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn hello_world(mut events: EventWriter<String>) {
    events.send("Hello world!".to_string());
}

fn greet_people(query: Query<&Name, With<Person>>, mut events: EventWriter<String>) {
    for name in query.iter() {
        events.send(format!("hello {}!", name.0));
    }
}

/// It's important that all functions using println obtain a reference to the console,
/// otherwise it seems to result in deadlock from multiple threads trying to grab a reference
/// to stdout? It's not clear exactly what's going on here, but it works with this.
fn printer(_: NonSend<GfxAndConsole>, mut events: EventReader<String>) {
    for evt in events.iter() {
        println!("{}", evt);
    }
}

fn exit_system(apt: NonSend<Apt>, input: Res<Hid>, mut exit: EventWriter<AppExit>) {
    if !apt.main_loop() {
        exit.send(AppExit);
        return;
    }

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
