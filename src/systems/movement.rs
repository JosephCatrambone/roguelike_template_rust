use crate::components::*;
use crate::map::Map;
use bevy_ecs::prelude::*;

pub fn step_try_move(mut commands: Commands, mut query: Query<(Entity, &mut Position, &mut TryMove), With<TurnActive>>, map: Res<Map>) {
	for (e, mut pos, mut vel) in query.iter_mut() {
		let old_x = pos.x;
		let old_y = pos.y;
		if vel.dx == 0 && vel.dy == 0 {
			// If this entity is not moving then don't burn the action on it.
			commands.entity(e)
				.remove::<TryMove>()
				.remove::<MoveFailed>()
				.remove::<MoveSucceeded>();
			continue;
		}
		pos.x = pos.x.saturating_add_signed(vel.dx);
		pos.y = pos.y.saturating_add_signed(vel.dy);
		if !map.tile_open(pos.x, pos.y) {
			pos.x = old_x;
			pos.y = old_y;
			commands.entity(e)
				.insert(MoveFailed)
				.remove::<MoveFailed>();
		}
		vel.dx = 0;
		vel.dy = 0;
		commands.entity(e)
			.remove::<TurnActive>()
			.insert(MoveSucceeded);
	}
}
