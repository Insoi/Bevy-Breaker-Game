use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Resource, Default)]
pub struct HitStop(pub Timer);

pub fn apply_hit_stop(
    time: Res<Time>,
    mut hit_stop: ResMut<HitStop>,
    mut physics_time: ResMut<Time<Physics>>,
) {
    hit_stop.0.tick(time.delta()); // <-- this was missing entirely — the timer never advances without it

    if !hit_stop.0.is_finished() {
        physics_time.pause();
    } else {
        physics_time.unpause();
    }
}