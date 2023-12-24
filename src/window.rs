use bevy::{
    prelude::*,
    window::{EnabledButtons, PresentMode, WindowResolution},
};

pub(crate) fn get_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1080., 720.),
            title: String::from("nadir"),
            resizable: false,
            enabled_buttons: EnabledButtons {
                maximize: false,
                ..default()
            },
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }
}
