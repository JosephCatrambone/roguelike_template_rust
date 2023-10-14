use crate::camera::Camera;
use crate::components::*;
use bevy_ecs::prelude::*;
use crate::color::RGB8;
use crate::map::{Map, TileTypes};


#[derive(Default, Resource)]
pub struct RenderedMap {
	pub width: u32,
	pub height: u32,
	pub tiles: Vec<RenderedMapTile>,
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
	let f_width = right - left;
	let f_height = bottom - top;

	// Clear OR pre-fill:
	if rendered_map_data.tiles.len() != (f_width*f_height) as usize || rendered_map_data.width != f_width || rendered_map_data.height != f_height {
		// There was a camera resize.  Reallocate.
		rendered_map_data.tiles.clear();
		for _ in 0..(f_width*f_height) {
			rendered_map_data.tiles.push(RenderedMapTile::new(' ' as u32, RGB8::new(0, 0, 0), RGB8::new(0, 0, 0)));
		}
		rendered_map_data.width = f_width;
		rendered_map_data.height = f_height;
	}
	rendered_map_data.tiles.iter_mut().for_each(|t| {
		t.code_point = ' ' as u32;
		t.fg_color = RGB8 { r: 0, g: 0, b: 0 };
		t.bg_color = t.fg_color; // TODO: Make sure this isn't messing with coloration.
	});

	for y in top..bottom {
		for x in left..right {
			if x > map.width || y > map.height { continue; }
			let map_idx = map.xy_idx(x, y);
			let render_idx = (x-left) + ((y-top)*f_width);
			let render_t: &mut RenderedMapTile = rendered_map_data.tiles.get_mut(render_idx as usize).unwrap();

			//if map.tile_open(x, y) { // We can render the tile and check what's in here OR we could iterate all the entities separately.
			render_t.code_point = match map.tiles[map_idx] {
				TileTypes::Wall => '#' as u32,
				TileTypes::Empty => '.' as u32,
			};
			render_t.fg_color.r = 255;
			render_t.fg_color.g = 255;
			render_t.fg_color.b = 255;
		}
	}

	// The tilemap has all the entities in a given slot.  We could/should reuse that?

}