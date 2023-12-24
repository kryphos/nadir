use crate::window::get_window_settings;
use bevy::prelude::*;

pub(crate) mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_settings()))
        .run();
}
