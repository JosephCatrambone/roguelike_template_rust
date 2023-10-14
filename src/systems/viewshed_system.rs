use bevy_ecs::prelude::*;
use crate::components::*;
use crate::map::Map;
use crate::rect_tools::RectangleBoundsIterator;
use crate::raycast::Raycast;

pub fn compute_viewshed(mut query: Query<(&Position, &mut Viewshed, Option<&PlayerControlled>)>, mut map: ResMut<Map>) {
	// TODO: Use changed position to reduce recompute.
	for (pos, mut vs, pc) in query.iter_mut() {
		if &vs.last_computed == pos {
			continue;
		}

		// TODO: If there are multiple PCs, we don't know which one will be the latest.
		if pc.is_some() {
			map.clear_visible();
		}

		vs.visible_tiles.clear();
		let mut bounds_iter = RectangleBoundsIterator::new_from_center(pos.x, pos.y, vs.range.div_ceil(2), vs.range.div_ceil(2));
		for (target_x, target_y) in bounds_iter.into_iter() {
			// Mark all points from the center to the boundary visible.
			let mut rc = Raycast::new(pos.x as i32, pos.y as i32, target_x as i32, target_y as i32);
			'inner: for (map_x, map_y) in rc.into_iter() {
				let open = map.tile_open(map_x as u32, map_y as u32);
				// If this tile is not open, we've just hit it.  Add the blocking tile to the visible list, then stop iterating.
				vs.visible_tiles.push(Position { x: map_x as u32, y: map_y as u32 });
				if pc.is_some() {
					// This is a player character, so we can add to the map.
					map.set_visible_and_revealed(map_x as u32, map_y as u32);
				}

				if !open {
					break 'inner;
				}
			}
		}
	}
}


