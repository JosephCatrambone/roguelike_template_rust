// Split out into a lib because we want to run unit tests.

mod color;
mod components;
mod input;
mod map;
mod raycast;
mod systems;

use crate::input::*;
use crate::components::*;
use legion::*;
use legion::systems::*;


enum GameMode {
    Paused,
    AwaitPlayerInput,
    BlockingModalCharacter,
    BlockingModelString,
    WorldTick,
}

pub struct GameState {
	ecs_world: World,
	ecs_scheduler: Schedule,
	ecs_resources: Resources,
	
	pub input_state: InputState,

	map: map::Map,
}

impl GameState {
	pub fn new() -> Self {
		let mut keymap = InputState::new();
		keymap.bind_key('w', ACTION_MOVE_UP);
		keymap.bind_key('a', input::KeyAction("move_left".to_string()));
		keymap.bind_key('s', input::KeyAction("move_down".to_string()));
		keymap.bind_key('d', input::KeyAction("move_right".to_string()));

		let mut schedule = Schedule::builder()
			//.add_system(update_positions_system())
			.add_system(systems::movement::step_try_move_system())
			.add_system(systems::viewshed_system::compute_viewshed_system())
			.build();

		let mut resources = Resources::default();
		resources.insert::<map::Map>(map::Map::new());

		GameState {
			ecs_world: World::default(),
			ecs_scheduler: schedule,
			ecs_resources: resources,
			input_state: keymap,
			map: map::Map::new(),
		}
	}

	pub fn update(&mut self) {
		// Handle input:
		self.handle_input();

		// Update systems:
		self.ecs_scheduler.execute(&mut self.ecs_world, &mut self.ecs_resources);
	}

	fn handle_input(&mut self) {
		// First, get the player.
		let mut player_controlled = <(Entity, &PlayerControlled)>::query();
		for (entity, player) in player_controlled.iter(&self.ecs_world) {
			if let Some(mut entry) = self.ecs_world.entry(*entity) {
				//entry.add_component(12f32);
				if self.input_state.is_action_just_pressed(input::ACTION_MOVE_UP) {
				}
			}
		}
	}
}
