use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use hroguelike::*;
use std::default::Default;

const TERMINAL_WIDTH: u32 = 160;
const TERMINAL_HEIGHT: u32 = 80;


#[derive(Debug, Clone)]
struct FontRenderingProperties {
	optimal_font_size: f32,
	scanline_height: f32,
	horizontal_offset: f32,
}


#[macroquad::main("Roguelike")]
async fn main() {
	let mut game = GameState::new();
	/*
	let font = load_ttf_font("./examples/DancingScriptRegular.ttf")
		.await
		.unwrap();
	*/

	//let s: String = v.into_iter().collect();
	// This will preserve a character buffer:
	// let s: String = v.iter().collect();

	let mut previous_screen_size = screen_size();
	let mut text_buffer = String::with_capacity((TERMINAL_WIDTH*TERMINAL_HEIGHT) as usize);
	for idx in 0..(TERMINAL_WIDTH * TERMINAL_HEIGHT) {
		text_buffer.push_str(&((idx % 10)+1).to_string());
	}

	let mut font_settings = find_optimal_font_settings(TERMINAL_WIDTH, TERMINAL_HEIGHT, None);
	dbg!("{:?}", &font_settings);


	loop {
		clear_background(BLACK);
		let mut display = "";
		let mut rest = text_buffer.as_str();
		for y in 0..TERMINAL_HEIGHT {
			(display, rest) = rest.split_at(TERMINAL_WIDTH as usize);
			draw_text(&display, font_settings.horizontal_offset, (1.0 + y as f32)*font_settings.scanline_height, font_settings.optimal_font_size, WHITE);
		}

		//draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
		//draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
		//draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
		//draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

		// Check to see if we need to resize everything:
		let new_screen_size = screen_size();
		if new_screen_size != previous_screen_size {
			font_settings = find_optimal_font_settings(TERMINAL_WIDTH, TERMINAL_HEIGHT, None);
			previous_screen_size = new_screen_size;
		}

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
		scanline_height: character_size.height,
		horizontal_offset: (screen_width - rendered_width) * 0.5
	}
}
