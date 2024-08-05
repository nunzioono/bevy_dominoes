use bevy::{app::PluginGroupBuilder, prelude::*, render::RenderPlugin};
use bevy_kira_audio::AudioPlugin;

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Dominoes".into(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }
}

fn render_plugin() -> RenderPlugin {
    bevy::render::RenderPlugin {
        render_creation: bevy::render::settings::RenderCreation::Automatic(
            bevy::render::settings::WgpuSettings {
                power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                ..default()
            }
        ),
        ..default()
    }
}

fn asset_plugin() -> AssetPlugin {
    AssetPlugin {
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    }
}

pub fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set (
        window_plugin()
    ).set (
        render_plugin()
    ).set(
        asset_plugin()
    )
    .add(AudioPlugin)
}