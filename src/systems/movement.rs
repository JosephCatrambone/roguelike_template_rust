use crate::components::*;
use crate::map::Map;
use bevy_ecs::prelude::*;
use crate::RunState;


pub fn step_try_move(mut query: Query<(&mut Position, &mut TryMove)>, map: Res<Map>) {
	for (mut pos, mut vel) in query.iter_mut() {
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
	}
}

pub fn player_movement_input(mut commands: Commands, mut query: Query<(Entity, &mut Position, &mut TryMove), With<TurnActive>>, mut run: ResMut<RunState>) {

}
