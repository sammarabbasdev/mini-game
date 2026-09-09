use crate::components::{Movable, Velocity};
use crate::resources::{WinSize, BASE_SPEED};
use bevy::prelude::*;

pub fn movable_system(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    let delta_time = time.delta_secs();

    for (entity, velocity, mut transform, movable) in &mut query {
        transform.translation.x += velocity.x * delta_time * BASE_SPEED;
        transform.translation.y += velocity.y * delta_time * BASE_SPEED;

        if movable.auto_despawn {
            let (x, y) = (transform.translation.x, transform.translation.y);
            if y.abs() > win_size.h / 2.0 + 200.0 || x.abs() > win_size.w / 2.0 + 200.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
