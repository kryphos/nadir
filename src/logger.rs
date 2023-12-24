use bevy::log::{Level, LogPlugin};

pub(crate) fn get_logger_settings() -> LogPlugin {
    LogPlugin {
        level: Level::INFO,
        filter: String::from("wgpu=warn,naga=warn"),
    }
}
