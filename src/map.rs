use legion::Entity;

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
		let x = idx % self.width;
		let y = idx / self.width;
		(x as u32, y as u32)
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

	pub fn tile_open(&self, x: u32, y: u32) -> bool {
		let idx = self.xy_idx(x, y);
		if idx > self.tiles.len() {
			return false;
		}
		return self.tiles[idx] == TileTypes::Empty;
	}
}