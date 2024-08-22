use std::time::Duration;

use crate::prelude::*;

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

fn menu_setup(mut cmds: Commands, r_assets: Res<AssetServer>) {
    cmds.spawn((
        Name::new("Play Button"),
        StateScoped(GameStates::Menu),
        SpriteBundle {
            texture: r_assets.load("ui/play.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::cuboid(120.0, 120.0),
    ))
    .observe(on_play_pressed)
    .with_children(|e| {
        e.spawn(SpriteBundle {
            texture: r_assets.load("ui/button_outline.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(120.0, 120.0)),
                ..default()
            },
            ..default()
        });

        e.spawn(Text2dBundle {
            text: Text::from_section(
                "Play!",
                TextStyle {
                    font_size: 80.0,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(0.0, -140.0, 0.0),
            ..default()
        });
    });
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
    mut r_state: ResMut<NextState<GameStates>>,
) {
    r_state.set(GameStates::Match);

    // TODO: Add VFXs

    cmds.entity(e_hit.entity()).despawn_recursive();
}
