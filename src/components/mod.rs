
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
	x: u32,
	y: u32,
}


#[derive(Clone, Copy, Debug, PartialEq)]
struct Sprite {
	fg_code: u32,
	bg_code: u32,
}


#[derive(Debug)]
pub struct Volume {
}