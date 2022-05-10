#![doc = include_str!("../README.md")]

use bevy::app::PluginGroup;

pub mod core;
pub mod input;
pub mod log;

/// A default set of plugins to get an app up and running. This also includes
/// most (but not all) of the same plugins in [`bevy::DefaultPlugins`] for ease
/// of use.
#[derive(Default)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            // Add log plugins early so we can see what's going on
            .add(log::LogPlugin)
            .add(bevy::log::LogPlugin)
            // Default bevy plugins
            .add(bevy::core::CorePlugin)
            .add(bevy::transform::TransformPlugin)
            .add(bevy::hierarchy::HierarchyPlugin)
            .add(bevy::diagnostic::DiagnosticsPlugin)
            .add(bevy::input::InputPlugin)
            // Since we don't have winit, we need the basic schedule runner
            .add(bevy::app::ScheduleRunnerPlugin)
            // Default bevy_3ds plugins
            .add(core::CorePlugin)
            .add(input::InputPlugin);

        // TODO: feature-dependent plugins like render, gltf, audio, etc.
    }
}
