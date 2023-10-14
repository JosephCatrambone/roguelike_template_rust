// Split out into a lib because we want to run unit tests.

mod action;
mod camera;
mod color;
mod components;
mod input;
mod map;
mod raycast;
mod systems;

use std::collections::HashSet;
use crate::input::*;
use crate::components::*;
use legion::*;
use legion::systems::*;
use std::sync::{Arc, Mutex};
use glam::UVec2;
use crate::action::Action;
use crate::camera::Camera;
use crate::color::RGB8;


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
	// Map is in resources.
	// Input state is also in resources.
}

impl GameState {
	pub fn new() -> Self {
		// Set up keymap:
		let mut keymap = InputState::new();
		keymap.bind_key('w', Action::MoveUp);
		keymap.bind_key('a', Action::MoveLeft);
		keymap.bind_key('s', Action::MoveDown);
		keymap.bind_key('d', Action::MoveRight);

		// Set the run order for our systems:
		let mut schedule = Schedule::builder()
			//.add_system(update_positions_system())
			.add_system(systems::movement::step_try_move_system())
			.add_system(systems::movement::player_movement_system())
			.add_system(systems::viewshed_system::compute_viewshed_system())
			.add_system(systems::map_rendering::render_map_system())
			.build();

		// Insert all the resources:
		let mut resources = Resources::default();
		resources.insert::<map::Map>(map::Map::new_random(600, 500, None));
		resources.insert::<GameMode>(GameMode::Paused);
		resources.insert::<InputState>(keymap);
		resources.insert::<camera::Camera>(camera::Camera::new(300, 200, 80, 60));
		resources.insert::<Vec<systems::map_rendering::RenderedMapTile>>(vec![]);

		// Setup world:
		let mut world = World::default();

		// TODO: We are inserting the player.  Hack-ish.
		let _player = world.push((
			Position(UVec2::new(10, 10)),
			Player {},
			PlayerControlled {},
			BlocksTile {},
			Renderable { codepoint: '@', fg_color: RGB8::new(0, 255, 128), bg_color: RGB8::new(0, 0, 0) },
		));

		GameState {
			ecs_world: world,
			ecs_scheduler: schedule,
			ecs_resources: resources,
		}
	}

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(u32, u32, &Vec<systems::map_rendering::RenderedMapTile>) -> ()) {
		let cam = self.ecs_resources.get::<Camera>().unwrap();
		let map_data = self.ecs_resources.get::<Vec<systems::map_rendering::RenderedMapTile>>().unwrap();
		if map_data.len() != (cam.width*cam.height) as usize {
			println!("Skipping render so map data can be recomputed with new camera frustum.");
		} else {
			render_fn(cam.width, cam.height, &map_data);
		}
	}

	pub fn update(&mut self) {
		// Update systems:
		self.ecs_scheduler.execute(&mut self.ecs_world, &mut self.ecs_resources);
	}

	pub fn input(&mut self, keys_down: &HashSet<char>) {
		let mut inputs = self.ecs_resources.get_mut::<InputState>().expect("Input state was lost!");
		inputs.update_from_keys(keys_down);
	}

	/*
	fn handle_input(&mut self) {
		let mut maybe_player_entities = vec![];
		{
			let mut player_controlled = <(Entity, &PlayerControlled)>::query();
			for (entity, _player) in player_controlled.iter(&self.ecs_world) {
				maybe_player_entities.push(entity.clone());
			}
		}

		// Movement?
		let mut dx = 0;
		let mut dy = 0;
		if self.input_state.is_action_just_pressed(input::KeyAction("move_up".to_string())) {
			dy -= 1;
		}
		if self.input_state.is_action_just_pressed(input::KeyAction("move_down".to_string())) {
		}

		for entity in maybe_player_entities {
			if let Some(mut entry) = self.ecs_world.entry(entity) {
				if let Ok(trymove) = entry.get_component_mut::<TryMove>() {
					trymove.dx = dx;
					trymove.dy = dy;
				} else {
					let trymove = TryMove {
						dx,
						dy,
						bonk: false,
					};
					entry.add_component(trymove);
				};
			}
		}
	}
	*/
}
