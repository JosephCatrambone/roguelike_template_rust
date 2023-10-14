use std::collections::{HashMap, HashSet};
use num;
use num_derive::FromPrimitive;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyAction(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct InputState {
	keymap: HashMap<char, KeyAction>,
	key_lookup: HashMap<KeyAction, char>,
	keys_pressed: HashSet<char>,
	keys_just_pressed: HashSet<char>,
	keys_just_released: HashSet<char>,
}

impl InputState {
	pub fn new() -> Self {
		InputState {
			keymap: HashMap::new(),
			key_lookup: HashMap::new(),
			keys_pressed: HashSet::new(),
			keys_just_pressed: HashSet::new(),
			keys_just_released: HashSet::new(),
		}
	}

	pub fn is_key_pressed(&self, key: char) -> bool {
		self.keys_pressed.contains(&key)
	}

	pub fn is_action_pressed(&self, action: KeyAction) -> bool {
		if let Some(key) = self.key_lookup.get(&action) {
			self.is_key_pressed(*key)
		} else {
			false
		}
	}

	pub fn is_key_just_pressed(&self, key: char) -> bool {
		self.keys_just_pressed.contains(&key)
	}

	pub fn is_action_just_pressed(&self, action: KeyAction) -> bool {
		if let Some(key) = self.key_lookup.get(&action) {
			self.is_key_just_pressed(*key)
		} else {
			eprintln!("Unrecognized keyaction: {:?}", &action);
			false
		}
	}

	pub fn is_key_just_released(&self, key: char) -> bool {
		self.keys_just_released.contains(&key)
	}

	pub fn is_action_just_released(&self, action: KeyAction) -> bool {
		if let Some(key) = self.key_lookup.get(&action) {
			self.is_key_just_released(*key)
		} else {
			eprintln!("Unrecognized keyaction: {:?}", &action);
			false
		}
	}

	pub fn reset_press_states(&mut self) {
		self.keys_just_pressed.clear();
		self.keys_just_released.clear();
	}

	pub fn bind_key(&mut self, key: char, action: KeyAction) {
		self.keymap.remove(&key);
		self.key_lookup.remove(&action);
		self.keymap.insert(key, action.clone());
		self.key_lookup.insert(action, key);
	}

	pub fn update_from_keys(&mut self, new_keys_down: &HashSet<char>) {
		// Clear the old keys just pressed and released.
		self.keys_just_pressed.clear();
		self.keys_just_released.clear();
		// Update pressed keys:
		for k in new_keys_down.iter() {
			if !self.keys_pressed.contains(k) {
				self.keys_just_pressed.insert(*k);
				self.keys_pressed.insert(*k);
			}
		}
		// Update released keys:
		for k in self.keys_pressed.iter() {
			if !new_keys_down.contains(k) {
				self.keys_just_released.insert(*k);
			}
		}
		// And now clear the released.
		for k in self.keys_just_released.iter() {
			self.keys_pressed.remove(k);
		}
	}
}


// Char implements Into<u32> but FromPrimitive only allows isize.
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, FromPrimitive)]
pub enum ISizeKeyCode {
	Right = 'R' as isize,
	Left = 'L' as isize,
	Down = 'D' as isize,
	Up = 'U' as isize,

	Space = ' ' as isize,
	Apostrophe = '\'' as isize,
	Comma = ',' as isize,
	Minus = '-' as isize,
	Period = '.' as isize,
	Slash = '/' as isize,
	Key0 = '0' as isize,
	Key1 = '1' as isize,
	Key2 = '2' as isize,
	Key3 = '3' as isize,
	Key4 = '4' as isize,
	Key5 = '5' as isize,
	Key6 = '6' as isize,
	Key7 = '7' as isize,
	Key8 = '8' as isize,
	Key9 = '9' as isize,
	Semicolon = ';' as isize,
	Equal = '=' as isize,
	A = 'a' as isize,
	B = 'b' as isize,
	C = 'c' as isize,
	D = 'd' as isize,
	E = 'e' as isize,
	F = 'f' as isize,
	G = 'g' as isize,
	H = 'h' as isize,
	I = 'i' as isize,
	J = 'j' as isize,
	K = 'k' as isize,
	L = 'l' as isize,
	M = 'm' as isize,
	N = 'n' as isize,
	O = 'o' as isize,
	P = 'p' as isize,
	Q = 'q' as isize,
	R = 'r' as isize,
	S = 's' as isize,
	T = 't' as isize,
	U = 'u' as isize,
	V = 'v' as isize,
	W = 'w' as isize,
	X = 'x' as isize,
	Y = 'y' as isize,
	Z = 'z' as isize,
	LeftBracket = '[' as isize,
	Backslash = '\\' as isize,
	RightBracket = ']' as isize,
	GraveAccent = '`' as isize,
	Enter = '\n' as isize,
	Tab = '\t' as isize,
}