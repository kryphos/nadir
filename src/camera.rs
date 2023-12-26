use bevy::{ prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Camera;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera_system);
    }
}

fn create_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera,
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 2.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
