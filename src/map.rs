use legion::{Entity};
use rand;
use rand::RngCore;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileTypes {
	Empty,
	Wall,
}

// Could be an entity?
pub struct Room {
}


pub struct Map {
	pub width: u32,
	pub height: u32,
	pub tiles: Vec<TileTypes>,
	pub tile_contents: Vec<Vec<Entity>>,
	pub visible_tiles: Vec<bool>, // Visible to player.
	pub revealed_tiles: Vec<bool>,
	pub rooms: Vec<Room>,
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

	pub fn tile_open(&self, x: u32, y: u32) -> bool {
		let idx = self.xy_idx(x, y);
		if idx > self.tiles.len() {
			return false;
		}
		return self.tiles[idx] == TileTypes::Empty;
	}
}

