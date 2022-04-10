use std::time::Duration;

use bevy_3ds::GfxAndConsole;
use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;

mod bevy_3ds;

fn main() {
    ctru::init();

    App::new()
        // Base Bevy plugins
        .add_plugin(CorePlugin)
        .add_plugin(ScheduleRunnerPlugin)
        .add_plugin(bevy_3ds::DefaultPlugin)
        // Global resources
        .insert_resource(ScheduleRunnerSettings {
            run_mode: bevy_app::RunMode::Loop {
                // TODO: this... doesn't seem to work right.
                wait: Some(Duration::from_millis(10_000)),
            },
        })
        // setup people entities/components
        .add_startup_system(add_people)
        // normal runtime stages
        .add_event::<String>()
        .add_system(greet_people)
        .add_system(hello_world)
        .add_system(printer)
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
