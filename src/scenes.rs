use std::time::Duration;

use crate::{hittable_button::SpawnHittableButtonExt, prelude::*};

pub fn plugin(app: &mut App) {
    app.register_type::<GameStates>();

    app.insert_state(GameStates::Menu);

    app.add_systems(Startup, game_setup);
    app.add_systems(OnEnter(GameStates::Match), match_setup);
    app.add_systems(OnEnter(GameStates::Menu), menu_setup);

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

fn menu_setup(
    mut cmds: Commands,
    r_assets: Res<AssetServer>,
    mut r_effects: ResMut<Assets<EffectAsset>>,
) {
    let play_button = cmds.spawn_hittable_button(
        &r_assets,
        &mut r_effects,
        r_assets.load("ui/play.png"),
        "Play!".into(),
    );
    cmds.entity(play_button)
        .insert(Name::new("Play!"))
        .observe(on_play_pressed);

    let exit_button = cmds.spawn_hittable_button(
        &r_assets,
        &mut r_effects,
        r_assets.load("ui/exit.png"),
        "Exit".into(),
    );
    cmds.entity(exit_button)
        .insert(Name::new("Play!"))
        .observe(on_exit_pressed);
}

fn on_player_death(
    e_player_dead: Trigger<OnPlayerDeath>,
    mut r_state: ResMut<NextState<GameStates>>,
) {
    r_state.set(GameStates::Menu);
}

fn on_play_pressed(
    e_hit: Trigger<OnHit>,
    mut cmds: Commands,
    mut q_vfx: Query<&mut EffectSpawner>,
    mut r_state: ResMut<NextState<GameStates>>,
) {
    r_state.set(GameStates::Match);

    // TODO: Add VFXs
    if let Ok(mut vfx) = q_vfx.get_mut(e_hit.entity()) {
        vfx.reset();
    }

    cmds.entity(e_hit.entity())
        .despawn_descendants()
        .insert(Lifetime::new(1.0));
}

fn on_exit_pressed(e_hit: Trigger<OnHit>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    app_exit_events.send(AppExit::Success);
}
