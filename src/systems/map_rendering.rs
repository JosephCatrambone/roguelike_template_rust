use crate::camera::Camera;
use crate::components::*;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::color::RGB8;
use crate::map::{Map, TileTypes};


#[derive(Default, Resource, Serialize, Deserialize)]
pub struct RenderedMap {
	width: u32,
	height: u32,
	#[serde(skip)] // Do we want to try a special handler for this?
	tiles: Vec<RenderedMapTile>,
}

impl RenderedMap {
	pub fn reallocate(&mut self, new_width: u32, new_height: u32) {
		self.tiles.clear();
		for _ in 0..(new_width*new_height) {
			self.tiles.push(RenderedMapTile::new(' ' as u32, RGB8::new(0, 0, 0), RGB8::new(0, 0, 0)));
		}
		self.width = new_width;
		self.height = new_height;
	}

	pub fn clear(&mut self) {
		self.tiles.iter_mut().for_each(|t| {
			t.code_point = ' ' as u32;
			t.fg_color = RGB8 { r: 0, g: 0, b: 0 };
			t.bg_color = t.fg_color; // TODO: Make sure this isn't messing with coloration.
		});
	}

	pub fn get_tile(&self, x: u32, y: u32) -> &RenderedMapTile {
		let render_idx = (x + y*self.width) as usize;
		self.tiles.get(render_idx).unwrap()
	}

	pub fn get_tile_mut(&mut self, x: u32, y: u32) -> &mut RenderedMapTile {
		let render_idx = (x + y*self.width) as usize;
		self.tiles.get_mut(render_idx).unwrap()
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}
}

pub struct RenderedMapTile {
	pub code_point: u32,
	pub fg_color: RGB8,
	pub bg_color: RGB8,
}

impl RenderedMapTile {
	pub fn new(c: u32, fg: RGB8, bg: RGB8) -> Self {
		RenderedMapTile {
			code_point: c,
			fg_color: fg,
			bg_color: bg,
		}
	}
}

pub fn render_map(query: Query<(&Position, &Renderable)>, map: Res<Map>, camera: Res<Camera>, mut rendered_map_data: ResMut<RenderedMap>) {
	let (left, top, right, bottom) = camera.get_frustum();

	if camera.width != rendered_map_data.width || camera.height != rendered_map_data.height {
		rendered_map_data.reallocate(camera.width, camera.height);
	}
	rendered_map_data.clear();

	for y in top..bottom {
		for x in left..right {
			if x > map.get_width() || y > map.get_height() { continue; }
			let render_t: &mut RenderedMapTile = rendered_map_data.get_tile_mut(x - left, y - top);

			let visible = map.is_visible(x, y);
			let revealed = map.is_revealed(x, y);

			if visible || revealed {
				//if map.tile_open(x, y) { // We can render the tile and check what's in here OR we could iterate all the entities separately.
				render_t.code_point = match map.get_tile_type(x, y) {
					TileTypes::Wall => '#' as u32,
					TileTypes::Empty => '.' as u32,
				};
				if visible {
					render_t.fg_color.r = 255;
					render_t.fg_color.g = 255;
					render_t.fg_color.b = 255;
				} else { // Only revealed.
					render_t.fg_color.r = 128;
					render_t.fg_color.g = 128;
					render_t.fg_color.b = 128;
				}
			}
		}
	}

	// The tilemap has all the entities in a given slot.  We could/should reuse that?
	// TODO: Option<Hidden>
	for (pos, render) in query.iter() {
		let x = pos.x;
		let y = pos.y;
		if x >= left && x < right && y >= top && y < bottom {
			let render_t: &mut RenderedMapTile = rendered_map_data.get_tile_mut(x - left, y - top);
			render_t.code_point = render.codepoint;
			render_t.fg_color = render.fg_color;
			render_t.bg_color = render.bg_color;
		}
	}
}