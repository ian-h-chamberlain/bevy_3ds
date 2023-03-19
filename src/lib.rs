#![doc = include_str!("../README.md")]

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod core;
pub mod input;
pub mod log;

/// A default set of plugins to get an app up and running. This also includes
/// most (but not all) of the same plugins in [`bevy::DefaultPlugins`] for ease
/// of use.
#[derive(Default)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            // Add log plugin early so we can see what's going on
            .add(log::LogPlugin);

        group = group
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin::default())
            .add(bevy::core::FrameCountPlugin::default())
            .add(bevy::time::TimePlugin::default())
            .add(bevy::transform::TransformPlugin::default())
            .add(bevy::hierarchy::HierarchyPlugin::default())
            .add(bevy::diagnostic::DiagnosticsPlugin::default())
            .add(bevy::input::InputPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::a11y::AccessibilityPlugin);

        // Default bevy_3ds plugins
        group = group.add(core::CorePlugin).add(input::InputPlugin);

        // TODO: feature-dependent plugins like render, gltf, audio, etc.

        group
    }
}
