// Split out into a lib because we want to run unit tests.

mod action;
mod camera;
mod color;
mod components;
mod gamelog;
mod input;
mod map;
mod raycast;
mod rect_tools;
mod systems;

use std::collections::HashSet;
use crate::input::*;
use crate::components::*;
use bevy_ecs::prelude::*;
use crate::action::Action;
use crate::color::RGB8;


#[derive(Copy, Clone, Resource, PartialEq, Hash)]
pub enum GameMode {
    Paused,
	AwaitingPlayerAction,
	AwaitingPlayerInventoryInput, // TODO: Perhaps we can do something fancier later.
	WorldTick,
}

pub struct GameState {
	ecs_world: World,
	ecs_scheduler: Schedule,
	// Map is in resources.
	input_state: InputState,
}

impl GameState {
	pub fn new() -> Self {
		// Set up keymap:
		let mut keymap = InputState::new();
		keymap.bind_key('w', Action::Move(0, -1));
		keymap.bind_key('a', Action::Move(-1, 0));
		keymap.bind_key('s', Action::Move(0, 1));
		keymap.bind_key('d', Action::Move(1, 0));

		// Setup world:
		let mut world = World::default();

		// Insert all the resources:
		world.insert_resource::<map::Map>(map::Map::new_random(600, 500, None));
		world.insert_resource::<GameMode>(GameMode::Paused);
		world.insert_resource::<gamelog::GameLog>(gamelog::GameLog::default());
		world.insert_resource::<camera::Camera>(camera::Camera::new(300, 200, 80, 60));
		world.insert_resource::<systems::RenderedMap>(systems::RenderedMap::default());

		// Set the run order for our systems:
		let mut schedule = Schedule::default();
		schedule.add_systems((
				systems::step_try_move,
				systems::compute_viewshed,
				systems::render_map,
				systems::camera_follow
			) //.run_if(step_world),
		);

		// TODO: We are inserting the player.  Hack-ish.
		let _player = world.spawn((
			Position { x: 10, y: 10 },
			Player {},
			PlayerControlled {},
			BlocksTile {},
			Viewshed { visible_tiles: vec![], range: 30, last_computed: Position { x: 0, y: 0 } },
			Renderable { codepoint: '@' as u32, fg_color: RGB8::new(0, 255, 128), bg_color: RGB8::new(0, 0, 0) },
		));

		GameState {
			ecs_world: world,
			ecs_scheduler: schedule,
			input_state: keymap
		}
	}

	// Sets the view frustum of the data block for map rendering and the camera frustum.
	pub fn set_camera_viewport_size(&mut self, width: u32, height: u32) {
		{
			let mut camera = self.ecs_world.get_resource_mut::<camera::Camera>().expect("Couldn't get camera ref.");
			camera.width = width;
			camera.height = height;
		}
		{
			let mut map_data = self.ecs_world.get_resource_mut::<systems::RenderedMap>().expect("Couldn't get map data ref.");
			map_data.reallocate(width, height);
		}
	}

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(&systems::RenderedMap) -> ()) {
		let map_data = self.ecs_world.get_resource::<systems::RenderedMap>().unwrap();
		render_fn(&map_data);
	}

	pub fn update(&mut self) {
		let current_game_mode = {
			self.ecs_world.get_resource::<GameMode>().expect("GameMode resource detached!? This can never happen.").clone()
		};
		let next_game_mode = match current_game_mode {
			GameMode::Paused => {
				GameMode::AwaitingPlayerAction
			},
			GameMode::AwaitingPlayerAction => {
				self.handle_player_action()
			},
			GameMode::AwaitingPlayerInventoryInput => {
				GameMode::Paused // TODO
			},
			GameMode::WorldTick => {
				self.ecs_scheduler.run(&mut self.ecs_world); // We have to step the world so that the inputs will be registered.
				GameMode::AwaitingPlayerAction
			}
		};
		//self.ecs_world.get_resource::<GameMode>().expect("GameMode resource detached!? This can never happen.").as_ref()
		*(self.ecs_world.get_resource_mut::<GameMode>().expect("Failed to get game mode!?").as_mut()) = next_game_mode;
	}

	pub fn save(&self) {
	}

	pub fn load(&mut self) {
	}

	// Thin wrappers:
	pub fn handle_key_down(&mut self, key: char) {
		self.input_state.handle_key_down(key);
	}

	pub fn handle_key_up(&mut self, key: char) {
		self.input_state.handle_key_up(key);
	}

	pub fn handle_player_action(&mut self) -> GameMode {
		let actions = self.input_state.pop_actions();
		if actions.is_empty() {
			return GameMode::AwaitingPlayerAction;
		}
		for a in actions {
			match a {
				Action::Move(dx, dy) => {
					for ((e, pos, maybe_trymove, _pc)) in self.ecs_world.query::<((Entity, &Position, Option<&mut TryMove>, &PlayerControlled))>().iter(&self.ecs_world) {
						if let Some(mv) = maybe_trymove {
							// Reuse component.
							//mv.dx = dx;
							//mv.dy = dy;
							// TODO: Start here.  Immutable ref?  What?
						}
					}
				},
				Action::Macro(ops) => {

				},
			}
		}
		return GameMode::WorldTick;
	}
}
