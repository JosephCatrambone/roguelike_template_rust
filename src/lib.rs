// Split out into a lib because we want to run unit tests.

mod action;
mod camera;
mod color;
mod components;
mod input;
mod map;
mod raycast;
mod rect_tools;
mod systems;

use std::collections::HashSet;
use crate::input::*;
use crate::components::*;
use bevy_ecs::prelude::*;
use std::sync::{Arc, Mutex};
use glam::UVec2;
use crate::action::Action;
use crate::camera::Camera;
use crate::color::RGB8;


#[derive(Resource)]
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

		// Setup world:
		let mut world = World::default();

		// Insert all the resources:
		world.insert_resource::<map::Map>(map::Map::new_random(600, 500, None));
		world.insert_resource::<GameMode>(GameMode::Paused);
		world.insert_resource::<InputState>(keymap);
		world.insert_resource::<camera::Camera>(camera::Camera::new(300, 200, 80, 60));
		world.insert_resource::<systems::map_rendering::RenderedMap>(systems::map_rendering::RenderedMap::default());

		// Set the run order for our systems:
		let mut schedule = Schedule::default();
		schedule.add_systems(systems::movement::step_try_move);
		schedule.add_systems(systems::movement::player_movement);
		schedule.add_systems(systems::viewshed_system::compute_viewshed);
		schedule.add_systems(systems::map_rendering::render_map);
		schedule.add_systems(systems::camera_follow::camera_follow);

		// TODO: We are inserting the player.  Hack-ish.
		let _player = world.spawn((
			Position(UVec2::new(10, 10)),
			Player {},
			PlayerControlled {},
			BlocksTile {},
			Renderable { codepoint: '@' as u32, fg_color: RGB8::new(0, 255, 128), bg_color: RGB8::new(0, 0, 0) },
		));

		GameState {
			ecs_world: world,
			ecs_scheduler: schedule,
		}
	}

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(u32, u32, &Vec<systems::map_rendering::RenderedMapTile>) -> ()) {
		let map_data = self.ecs_world.get_resource::<systems::map_rendering::RenderedMap>().unwrap();
		render_fn(map_data.width, map_data.height, &map_data.tiles);
	}

	pub fn update(&mut self) {
		// Update systems:
		self.ecs_scheduler.run(&mut self.ecs_world);
	}

	pub fn input(&mut self, keys_down: &HashSet<char>) {
		let mut inputs = self.ecs_world.get_resource_mut::<InputState>().expect("Input state was lost!");
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
