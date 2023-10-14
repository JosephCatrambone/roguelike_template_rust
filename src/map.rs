use bevy_ecs::prelude::{Resource, Entity};
use rand;
use rand::RngCore;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileTypes {
	Empty,
	Wall,
}

// Could be an entity?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
}


#[derive(Resource, Serialize, Deserialize)]
pub struct Map {
	width: u32,
	height: u32,
	tiles: Vec<TileTypes>,
	tile_contents: Vec<Vec<Entity>>,
	visible_tiles: Vec<bool>, // Visible to player.
	revealed_tiles: Vec<bool>,
	rooms: Vec<Room>,
}

impl Map {
	pub fn xy_idx(&self, x: u32, y: u32) -> usize {
		(x as usize) + (y * self.width) as usize
	}

	pub fn idx_xy(&self, idx: usize) -> (u32, u32) {
		let x = idx as u32 % self.width;
		let y = idx as u32 / self.width;
		(x, y)
	}

	pub fn new() -> Self {
		Self {
			width: 0,
			height: 0,
			tiles: vec![],
			tile_contents: vec![],
			visible_tiles: vec![],
			revealed_tiles: vec![],
			rooms: vec![],
		}
	}

	pub fn new_empty(width: u32, height: u32) -> Self {
		let mut map = Self {
			width,
			height,
			tiles: vec![],
			tile_contents: vec![],
			visible_tiles: vec![],
			revealed_tiles: vec![],
			rooms: vec![],
		};

		for _y in 0..height {
			for _x in 0..width {
				map.tiles.push(TileTypes::Empty);
				map.tile_contents.push(vec![]);
				map.visible_tiles.push(false);
				map.revealed_tiles.push(false);
			}
		}

		map
	}

	pub fn new_random(width: u32, height: u32, rng: Option<rand::rngs::ThreadRng>) -> Self {
		let mut rng = if let Some(r) = rng { r } else { rand::thread_rng() };

		let mut map = Self::new_empty(width, height);
		for y in 0..height {
			for x in 0..width {
				if rng.next_u32() % 10 == 0 {
					let idx = map.xy_idx(x, y);
					map.tiles[idx] = TileTypes::Wall;
				}
			}
		}

		map
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}

	pub fn is_visible(&self, x: u32, y: u32) -> bool {
		let idx = self.xy_idx(x, y);
		self.visible_tiles[idx]
	}

	pub fn is_revealed(&self, x: u32, y: u32) -> bool {
		let idx = self.xy_idx(x, y);
		self.revealed_tiles[idx]
	}

	pub fn get_tile_type(&self, x: u32, y: u32) -> TileTypes {
		let idx = self.xy_idx(x, y);
		self.tiles[idx]
	}

	pub fn tile_open(&self, x: u32, y: u32) -> bool {
		let idx = self.xy_idx(x, y);
		if idx > self.tiles.len() {
			return false;
		}
		return self.tiles[idx] == TileTypes::Empty;
	}

	pub fn clear_revealed(&mut self) {
		for t in self.revealed_tiles.iter_mut() {
			*t = false;
		}
	}

	pub fn clear_visible(&mut self) {
		for t in self.visible_tiles.iter_mut() {
			*t = false;
		}
	}

	// Also sets revealed.
	pub fn set_visible_and_revealed(&mut self, x: u32, y: u32) {
		let idx = self.xy_idx(x, y);
		self.visible_tiles[idx] = true;
		self.revealed_tiles[idx] = true;
	}

	pub fn unset_visible(&mut self, x: u32, y: u32) {
		let idx = self.xy_idx(x, y);
		self.visible_tiles[idx] = false;
	}
}

