use std::sync::{Arc, Mutex};
use crate::components::*;
use crate::map::Map;
use crate::input::InputState;
use legion::*;
use legion::world::SubWorld;


#[system(for_each)]
pub fn step_try_move(pos: &mut Position, vel: &mut TryMove, #[resource] map: &Map) {
	let old_x = pos.x;
	let old_y = pos.y;
	pos.x = pos.x.saturating_add_signed(vel.dx);
	pos.y = pos.y.saturating_add_signed(vel.dy);
	vel.bonk = false;
	if !map.tile_open(pos.x, pos.y) {
		pos.x = old_x;
		pos.y = old_y;
		vel.bonk = true;
	}
	vel.dx = 0;
	vel.dy = 0;

	/*
	let mut query = <(Entity, Read<Pos>, Write<Rot>)>::query();

	let mut count = 0;
	for (entity, pos, rot) in query.iter_mut(&mut world) {
		assert_eq!(expected.get(&entity).unwrap().0, *pos);
		assert_eq!(expected.get(&entity).unwrap().1, *rot);
		count += 1;

		rot.0 = 0.0;
	}
	*/
}

#[system]
pub fn player_movement(world: &mut SubWorld, query: &mut Query<(Entity, &PlayerControlled)>, #[resource] input_state: &Arc<Mutex<InputState>>) {
	// TODO: It would be nice if we had a way of iterating over entities with Optional<TryMove>, getting it back as Some or None so we can add it if it's missing.
	for (e, _pc) in query.iter(world) {
		if let Ok(c) = &mut world.entry_mut(*e) {

		}
	}
}