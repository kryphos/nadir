use crate::window::get_window_settings;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_rapier3d::prelude::*;
use logger::get_logger_settings;

pub(crate) mod camera;
pub(crate) mod components;
pub(crate) mod logger;
pub(crate) mod player;
pub(crate) mod window;
pub(crate) mod world;

struct GamePluginGroup;
impl PluginGroup for GamePluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(world::WorldPlugin)
            .add(camera::CameraPlugin)
            .add(player::PlayerPlugin)
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(get_window_settings())
                .set(get_logger_settings()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(GamePluginGroup)
        .run();
}
