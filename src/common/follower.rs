use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<FollowEntity>();

    app.add_systems(Update, run_follow_entity);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FollowEntity {
    offset: Vec3,
    entity: Entity,
}

fn run_follow_entity(
    mut q_follower: Query<(&mut Transform, &FollowEntity)>,
    mut q_targets: Query<&GlobalTransform>,
) {
    for (mut xform, follower) in q_follower.iter_mut() {
        if let Ok(target_xform) = q_targets.get(follower.entity) {
            xform.translation = target_xform.translation() + follower.offset;
        }
    }
}
