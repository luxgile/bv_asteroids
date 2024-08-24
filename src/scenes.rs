use std::time::Duration;


use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<GameStates>();

    app.insert_state(GameStates::Menu);

    app.add_systems(Startup, game_setup);
    app.add_systems(OnEnter(GameStates::Match), match_setup);

    app.observe(on_player_death);
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, States, Reflect)]
pub enum GameStates {
    Menu,
    Match,
}

fn game_setup(mut cmds: Commands) {
    let mut player_camera = PlayerCameraBundle::new();
    player_camera.camera.projection.scale = 5.0;
    cmds.spawn(player_camera);

    cmds.add(SpawnPlayer);
}

fn match_setup(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Spawner"),
        StateScoped(GameStates::Match),
        AsteroidSpawner {
            timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Repeating),
        },
    ));
}

fn on_player_death(
    e_player_dead: Trigger<OnPlayerDeath>,
    mut r_state: ResMut<NextState<GameStates>>,
) {
    r_state.set(GameStates::Menu);
}
