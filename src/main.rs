use std::sync::Once;

use bevy::log;
use bevy::prelude::*;

mod bevy_3ds;

fn main() {
    ctru::init();

    let mut app = App::new();

    app
        // 3ds stuff needs to be set up first, for gfx, console, etc
        .add_plugin(bevy_3ds::DefaultPlugin)
        // Then logging, so we can see what Bevy is doing
        .insert_resource(log::LogSettings {
            level: log::Level::TRACE,
            ..Default::default()
        })
        .add_plugin(log::LogPlugin)
        // Base Bevy plugins
        .add_plugins(MinimalPlugins)
        // Startup
        .add_startup_system(add_people)
        // normal runtime stages
        .add_system(greet_people)
        .add_system(hello_world)
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

fn hello_world(_: Query<&Name, With<Person>>) {
    static HELLO: Once = Once::new();

    HELLO.call_once(|| {
        log::info!("Hello world!");
    });
}

fn greet_people(query: Query<&Name, With<Person>>) {
    static HELLO: Once = Once::new();

    HELLO.call_once(|| {
        for name in query.iter() {
            log::info!("hello {}!", name.0);
        }
    });
}
