use crate::*;
use bevy::prelude::*;
use camera::PlayerCameraBundle;
use player::Player;
use shooter::Shooter;

pub fn plugin(app: &mut App) {
    app.register_type::<GameStates>();

    app.insert_state(GameStates::InGame);

    app.add_systems(OnEnter(GameStates::InGame), ingame_setup);
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Hash, States, Reflect)]
pub enum GameStates {
    Menu,
    #[default]
    InGame,
}

fn ingame_setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut player_camera = PlayerCameraBundle::new();
    player_camera.camera.projection.scale = 5.0;
    cmds.spawn(player_camera);

    let player = player::PlayerBundle::new(&mut meshes, &mut materials);
    cmds.spawn(player);

    let mut asteroid = asteroids::AsteroidBundle::new(&mut meshes, &mut materials);
    asteroid.mesh.transform.translation = Vec3::new(750.0, 450.0, 0.0);
    cmds.spawn(asteroid);
}
