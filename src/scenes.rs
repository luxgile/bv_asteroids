use std::time::Duration;

use crate::*;
use bevy::prelude::*;
use camera::PlayerCameraBundle;
use player::{OnPlayerDeath, SpawnPlayer};
use score::SpawnMoney;
use spawner::AsteroidSpawner;

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

fn ingame_setup(mut cmds: Commands) {
    let mut player_camera = PlayerCameraBundle::new();
    player_camera.camera.projection.scale = 5.0;
    cmds.spawn(player_camera);

    cmds.add(SpawnPlayer);

    cmds.spawn((
        Name::new("Spawner"),
        StateScoped(GameStates::InGame),
        AsteroidSpawner {
            timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Repeating),
        },
    ));

    cmds.add(SpawnMoney {
        money: 50,
        position: Vec2::new(750.0, 500.0),
        radial_force: 100.0..250.0,
    })
}

fn menu_setup(mut cmds: Commands) {
    cmds.spawn((StateScoped(GameStates::Menu)));
}

fn on_player_death(
    e_player_dead: Trigger<OnPlayerDeath>,
    mut r_state: ResMut<NextState<GameStates>>,
) {
    r_state.set(GameStates::Menu);
}

fn on_play_pressed(mut r_state: ResMut<NextState<GameStates>>) {
    r_state.set(GameStates::InGame);
}
