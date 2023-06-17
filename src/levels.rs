use crate::GameState;
use bevy::prelude::*;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(load_level.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn load_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("meshes/lvl_1.glb"),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Capsule::default().into()),
        material: materials.add(Color::BLUE.into()),
        ..default()
    });
}
