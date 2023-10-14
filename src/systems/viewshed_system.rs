use bevy_ecs::prelude::*;
use crate::components::*;
use crate::map::Map;
use crate::raycast::Raycast;

pub fn compute_viewshed(mut query: Query<(&Position, &mut Viewshed, Option<&PlayerControlled>)>, mut map: ResMut<Map>) {
	// TODO: Use changed position to reduce recompute.
	for (pos, mut vs, pc) in query.iter_mut() {
		if &vs.last_computed == pos {
			continue;
		}

		if pc.is_some() {
			map.visible_tiles.clear();
		}

		// TODO: this is four-way symmetric and it would be nice to boost the speed.
		vs.visible_tiles.clear();
		let left = pos.x.saturating_sub(vs.range);
		let right = pos.x + vs.range;
		let top = pos.y.saturating_sub(vs.range);
		let bottom = pos.y + vs.range;
		for i in 0..vs.range {
			// Mark all points from the center to the boundary visible.

		}
	}
}


