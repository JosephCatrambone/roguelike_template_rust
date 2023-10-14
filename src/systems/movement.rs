use std::sync::{Arc, Mutex};
use crate::components::*;
use crate::map::Map;
use crate::input::InputState;
use bevy_ecs::prelude::*;


pub fn step_try_move(query: Query<(&mut Position, &mut TryMove)>, map: Res<Map>) {
	/*
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
	*/
}

pub fn player_movement(query: Query<(&PlayerControlled)>, input_state: Res<InputState>) {
	// TODO: It would be nice if we had a way of iterating over entities with Optional<TryMove>, getting it back as Some or None so we can add it if it's missing.

}