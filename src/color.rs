use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct RGB8 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl RGB8 {
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b }
	}
}

pub const BLACK: RGB8 = RGB8 {
	r: 0, g: 0, b: 0
};

pub const WHITE: RGB8 = RGB8 {
	r: 255, g: 255, b: 255
};

pub const DARK_GREY: RGB8 = RGB8 {
	r: 10, g: 10, b: 10
};
