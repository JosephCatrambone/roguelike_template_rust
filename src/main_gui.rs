use hroguelike::*;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use std::collections::HashSet;
use std::default::Default;

pub const TERMINAL_WIDTH: u32 = 160;
pub const TERMINAL_HEIGHT: u32 = 80;
pub const MAP_VIEW_WIDTH: u32 = 80;
pub const MAP_VIEW_HEIGHT: u32 = 40;

/*
+-------+-------+
|.......|.......|
|.......|.......|
+-------+-------+
|...............|
|...............|
+---------------+
*/


#[derive(Debug, Clone)]
struct FontRenderingProperties {
	optimal_font_size: f32,
	character_width: f32,
	scanline_height: f32,
	horizontal_offset: f32,
}


#[macroquad::main("Roguelike")]
async fn main() {
	let mut game = GameState::new();
	//game.set_camera_viewport_size(MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT);
	/*
	let font = load_ttf_font("./examples/DancingScriptRegular.ttf")
		.await
		.unwrap();
	*/

	let mut previous_screen_size = screen_size();
	let mut font_settings = find_optimal_font_settings(TERMINAL_WIDTH, TERMINAL_HEIGHT, None);
	dbg!("{:?}", &font_settings);

	loop {
		clear_background(BLACK);
		//let mut display = game.map.render_map(0, 0, TERMINAL_WIDTH, TERMINAL_HEIGHT);
		game.with_rendered_map_data(|data|{
			for y in 0..data.get_height() {
				for x in 0..data.get_width() {
					let d = &data.get_tile(x, y);
					let fx = (x as f32*font_settings.character_width)+font_settings.horizontal_offset;
					let fy = (1.0 + y as f32)*font_settings.scanline_height;
					let fs = font_settings.optimal_font_size;
					let fc = Color::from_rgba(d.fg_color.r, d.fg_color.g, d.fg_color.b, 255);
					let fbg = Color::from_rgba(d.bg_color.r, d.bg_color.g, d.bg_color.b, 255);
					if fbg != BLACK {
						draw_rectangle(fx, fy, font_settings.character_width, font_settings.scanline_height, fbg);
					}
					draw_text(&String::from(char::from_u32(d.code_point).unwrap_or('?')), fx, fy, fs,fc);
				}
			}
		});


		// Check to see if we need to resize everything:
		let new_screen_size = screen_size();
		if new_screen_size != previous_screen_size {
			font_settings = find_optimal_font_settings(TERMINAL_WIDTH, TERMINAL_HEIGHT, None);
			previous_screen_size = new_screen_size;
		}

		//game.input_state.update_from_keys()
		update_inputs(&mut game);
		game.update();

		next_frame().await
	}
}

fn find_optimal_font_settings(terminal_width: u32, terminal_height: u32, font: Option<&Font>) -> FontRenderingProperties {
	let mut font_size = 8;
	let mut previous_font_size = font_size;
	let mut previous_previous_size = font_size;
	let mut character_size = TextDimensions { width: 0f32, height: 0f32, offset_y: 0f32 };
	let mut rendered_width = 0f32; // We're going to use this later.
	let mut rendered_height = 0f32;
	let (screen_width, screen_height) = screen_size();
	loop {
		character_size = measure_text(&"#", font, font_size, 1.0);
		rendered_width = character_size.width * terminal_width as f32;
		rendered_height = character_size.height * terminal_height as f32;

		previous_previous_size = previous_font_size;
		previous_font_size = font_size;

		if rendered_width >= screen_width || rendered_height >= screen_height {
			// Our chosen font is too big.
			font_size -= 1;
		} else if rendered_width < screen_width || rendered_height < screen_height {
			font_size += 1;
		} else {
			break;
		}

		// Avoid looping between two good options.
		if font_size == previous_previous_size || font_size == previous_font_size {
			break;
		}
	}

	FontRenderingProperties {
		optimal_font_size: font_size as f32,
		character_width: character_size.width,
		scanline_height: character_size.height,
		horizontal_offset: (screen_width - rendered_width) * 0.5
	}
}

fn update_inputs(game_state: &mut GameState) {
	// TODO: This is a leaky hack.
	let mut new_keys = HashSet::new();
	while let Some(c) = get_char_pressed() {
		new_keys.insert(c);
	}
	game_state.input(&new_keys);
}
