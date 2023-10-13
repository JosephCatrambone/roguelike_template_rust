use crate::components::*;
use crate::map::Map;
use legion::*;

#[system(for_each)]
pub fn movement(pos: &mut Position, vel: &Velocity, #[resource] map: &Map) {
	let old_x = pos.x;
	let old_y = pos.y;
	pos.x = pos.x.saturating_add_signed(vel.x); // * time.elapsed_seconds;
	pos.y = pos.y.saturating_add_signed(vel.y);
	if !map.tile_open(pos.x, pos.y) {
		pos.x = old_x;
		pos.y = old_y;
	}
}