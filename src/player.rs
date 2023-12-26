use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player_system);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {}

fn spawn_player_system(mut commands: Commands) {
    commands.spawn(Player);
}
