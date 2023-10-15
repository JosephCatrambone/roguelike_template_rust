use bevy_ecs::prelude::*;
use crate::components::*;
use crate::color::RGB8;


fn make_npc(commands: &mut Commands, x: u32, y: u32) {
	commands.spawn((
		Position { x, y },
		BlocksTile {},
		Viewshed::new(30),
		Renderable { codepoint: 'A' as u32, fg_color: RGB8::new(150, 205, 128), bg_color: RGB8::new(0, 0, 0) }
	));
}