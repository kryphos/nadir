use crate::window::get_window_settings;
use bevy::prelude::*;
use logger::get_logger_settings;

pub(crate) mod logger;
pub(crate) mod window;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(get_window_settings())
                .set(get_logger_settings()),
        )
        .run();
}
