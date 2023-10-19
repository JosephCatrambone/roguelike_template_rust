use bevy_ecs::prelude::*;
use crate::action::Action;
use crate::components::*;
use crate::components::ai::*;
use crate::map::Map;
use rand::{RngCore, thread_rng};

pub fn npc_thinking(mut commands: Commands, mut query: Query<(Entity, &mut NPC, &Initiative)>, map: Res<Map>) {
	let mut rng = thread_rng(); // I do not like this.

	for (e, mut npc_ai, initiative) in query.iter_mut() {
		// Queue the action when we're about to move:
		if initiative.current == 1 && npc_ai.selected_action.is_some() {
			let chosen_action= npc_ai.selected_action.take().unwrap();
			match chosen_action {
				Action::MoveRight => {
					commands.entity(e).insert(TryMove { dx: 1, dy: 0 });
				}
				Action::MoveUp => {
					commands.entity(e).insert(TryMove { dx: 0, dy: -1 });
				}
				Action::MoveLeft => {
					commands.entity(e).insert(TryMove { dx: -1, dy: 0 });
				}
				Action::MoveDown => {
					commands.entity(e).insert(TryMove { dx: 0, dy: 1 });
				}
				Action::Macro(_) => {}
			}
		} else if initiative.current > 1 && npc_ai.selected_action.is_none() {
			// It's not our turn AND we have some turns before the next action.
			npc_ai.selected_action = match rng.next_u32() % 5 {
				0 => Some(Action::MoveUp),
				1 => Some(Action::MoveRight),
				2 => Some(Action::MoveLeft),
				3 => Some(Action::MoveDown),
				_ => None,
			};
		} else {
			// We may be in the initial state.  Our initiative is zero AND we have no action.

		}
	}
}