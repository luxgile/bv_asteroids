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

fn menu_setup(
    mut cmds: Commands,
    r_assets: Res<AssetServer>,
    mut r_effects: ResMut<Assets<EffectAsset>>,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    let writer = ExprWriter::new();

    let init_pos = SetPositionCircleModifier {
        axis: writer.lit(Vec3::Z).expr(),
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(100.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        axis: writer.lit(Vec3::Z).expr(),
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.lit(300.0)
            + writer.rand(ValueType::Scalar(ScalarType::Float)) * writer.lit(300.0))
        .expr(),
    };

    let size = SetSizeModifier {
        size: CpuValue::Single(Vec2::ONE * 20.0),
    };

    let lifetime = writer.lit(0.2).expr(); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let mut spawner = Spawner::once(25.0.into(), false);
    let effect = EffectAsset::new(vec![32768], spawner, writer.finish())
        .with_name("MyEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier { gradient })
        .render(size);

    cmds.spawn((
        Name::new("Play Button"),
        StateScoped(GameStates::Menu),
        ParticleEffectBundle {
            effect: ParticleEffect::new(r_effects.add(effect)),
            transform: Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::cuboid(120.0, 120.0),
    ))
    .observe(on_play_pressed)
    .with_children(|e| {
        e.spawn(SpriteBundle {
            texture: r_assets.load("ui/play.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        });

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
