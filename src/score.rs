use bevy::prelude::*;

use crate::common::OnDeath;

pub fn plugin(app: &mut App) {
    app.insert_resource(PlayerScore(0));
    app.register_type::<PlayerScore>();
    app.register_type::<Score>();

    app.observe(apply_score_reward);
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerScore(pub u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Score(pub u32);

fn apply_score_reward(
    e_death: Trigger<OnDeath>,
    q_score: Query<&Score>,
    mut r_score: ResMut<PlayerScore>,
) {
    if let Ok(score) = q_score.get(e_death.entity()) {
        r_score.0 += score.0;
        println!("New score: {:?}", r_score.0);
    }
}
