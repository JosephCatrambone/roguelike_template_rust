use anyhow::Result;
use crossterm::{
	QueueableCommand, ExecutableCommand,
	cursor,
	style::{Color, Colors, Print, ResetColor, SetColors, Attribute, SetAttribute},
	event::{poll, read, Event, KeyCode},
	terminal,
};
use roguelike_template_rust::*;
use std::default::Default;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;


pub const TERMINAL_WIDTH: u32 = 160;
pub const TERMINAL_HEIGHT: u32 = 80;
pub const MAP_VIEW_WIDTH: u32 = 80;
pub const MAP_VIEW_HEIGHT: u32 = 40;


fn main() -> Result<()> {
	let out = Arc::new(Mutex::new(stdout()));
	let mut game = GameState::new();
	game.set_camera_viewport_size(MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT);

	terminal::enable_raw_mode()?;
	out.lock().unwrap().execute(terminal::EnterAlternateScreen)?;
	out.lock().unwrap().execute(cursor::Hide)?;
	out.lock().unwrap().queue(terminal::Clear(terminal::ClearType::All))?;

	loop {
		// Input handling:
		if poll(Duration::from_millis(10))? {
			match read()? {
                Event::FocusGained => {},
                Event::FocusLost => {},
				Event::Key(key_event) => {
					match key_event.code {
						KeyCode::Enter => {},
						KeyCode::F(_num) => {},
						KeyCode::Esc => {
							break;
						},
						KeyCode::Char(c) => {
							game.handle_key_down(c);
							game.handle_key_up(c);
						},
						_ => {}
					}
				},
				Event::Mouse(_event) => {},
				Event::Paste(_data) => {},
				Event::Resize(_width, _height) => {},
			}
		}

		// Update:
		game.update();

		// Check for display changes:
		{
			let out = Arc::clone(&out);
			game.with_rendered_map_data(|data|{
				let mut out = out.lock().unwrap();
				for y in 0..data.get_height() {
					for x in 0..data.get_width() {
						let d = &data.get_tile(x, y);
						out
							.queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
							//.queue(style::PrintStyledContent(&String::from(char::from_u32(d.code_point).unwrap_or('?'))))?;
							//.queue(SetBackgroundColor())?.queue(SetForegroundColor())?
							.queue(SetColors(Colors::new(
								Color::Rgb { r: d.fg_color.r, g: d.fg_color.g, b: d.fg_color.b },
								Color::Rgb { r: d.bg_color.r, g: d.bg_color.g, b: d.bg_color.b },
							))).unwrap()
							.queue(Print(char::from_u32(d.code_point).unwrap_or('?'))).unwrap();
					}
				}
			});
		}
		out.lock().unwrap().queue(ResetColor)?;
		out.lock().unwrap().queue(SetAttribute(Attribute::Reset))?;
		out.lock().unwrap().flush()?;
	}
	out.lock().unwrap().execute(terminal::Clear(terminal::ClearType::All))?;
	out.lock().unwrap().execute(cursor::Show)?;
	out.lock().unwrap().execute(terminal::LeaveAlternateScreen)?;
	terminal::disable_raw_mode().unwrap();
	Ok(())
}

