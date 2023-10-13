use legion::*;
use legion::world::SubWorld;

use crate::components::*;
use crate::map::Map;

#[system]
pub fn compute_viewshed(world: &mut SubWorld, query: &mut Query<(&Position, &mut Viewshed)>, #[resource] map: &Map) {
	for (pos, view) in query.iter_mut(world) {
		//println!("{} {}", a, b);
		if !view.dirty {
			continue;
		}

		// Otherwise we need to clear our old viewshed.
	}
}


