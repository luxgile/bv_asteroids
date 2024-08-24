use crate::{
    hittable_button::{HittableButton, SpawnHittableButtonExt},
    prelude::*,
};

pub mod prelude {}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameStates::Menu), setup_main_menu);
    app.add_systems(Update, update_menu_buttons);
}

fn setup_main_menu(
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
        .insert(Name::new("Exit"))
        .observe(on_exit_pressed);
}

fn update_menu_buttons(
    mut cmds: Commands,
    q_player: Query<&GlobalTransform, With<Player>>,
    mut gizmos: Gizmos,
    mut q_buttons: Query<(Entity, &mut Transform, &HittableButton, &mut Velocity)>,
) {
    let player = q_player.single();

    let button_count = q_buttons.iter().len();
    let angle_dt = 360.0 / button_count as f32;
    let force_mult = 100.0;
    let vel_clamp = 1000.0;
    for (i, (e, mut xform, button, mut vel)) in q_buttons.iter_mut().enumerate() {
        let angle = f32::to_radians(angle_dt * i as f32);
        let target = player.translation().xy() + Vec2::from_angle(angle) * button.distance;
        let position_difference = target - xform.translation.xy();
        cmds.entity(e).insert(ExternalForce {
            force: position_difference * force_mult,
            ..default()
        });
        vel.linvel = vel.linvel.clamp_length(0.0, vel_clamp);
        xform.look_to(Vec3::Z, Vec3::Y);
        gizmos.arrow_2d(
            xform.translation.xy(),
            xform.translation.xy() + position_difference * force_mult,
            GREEN,
        );
        gizmos.arrow_2d(xform.translation.xy(), target, RED);
    }
}

fn on_play_pressed(
    e_hit: Trigger<OnHit>,
    mut cmds: Commands,
    mut q_vfx: Query<&mut EffectSpawner>,
    mut r_state: ResMut<NextState<GameStates>>,
    q_buttons: Query<Entity, With<HittableButton>>,
) {
    r_state.set(GameStates::Match);

    // TODO: Add VFXs
    if let Ok(mut vfx) = q_vfx.get_mut(e_hit.entity()) {
        vfx.reset();
    }

    cmds.entity(e_hit.entity())
        .despawn_descendants()
        .insert(Lifetime::new(1.0));

    clear_buttons(cmds, q_buttons);
}

fn on_exit_pressed(
    e_hit: Trigger<OnHit>,
    mut cmds: Commands,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    q_buttons: Query<Entity, With<HittableButton>>,
) {
    app_exit_events.send(AppExit::Success);

    clear_buttons(cmds, q_buttons);
}

fn clear_buttons(mut cmds: Commands, q_buttons: Query<Entity, With<HittableButton>>) {
    for e in q_buttons.iter() {
        cmds.entity(e).despawn_recursive();
    }
}
