use bevy_ecs::prelude::*;
use crate::components::*;

pub fn random_ai_movement(mut commands: Commands, mut query: Query<(Entity, &mut Position, Option<&mut TryMove>, &mut Initiative), (With<TurnActive>, With<AIRandomMovement>)>) {
	for (e, pos, maybe_trymove, initiative) in query.iter_mut() {
		if let Some(tm) = maybe_trymove {
			if tm.bonk {
				
			}
		}
	}
}