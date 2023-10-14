
enum CurrentSide {
	Left,
	Top,
	Right,
	Bottom,
}

pub struct RectangleBoundsIterator {
	pub left: u32,
	pub top: u32,
	pub right: u32,
	pub bottom: u32,

	current_side: CurrentSide,
	current_x: u32,
	current_y: u32,
	dx: u8,
	dy: u8,
}

impl RectangleBoundsIterator {
	pub fn new_from_center(x: u32, y: u32, half_width: u32, half_height: u32) -> Self {
		let left = x.saturating_sub(half_width);
		let top = y.saturating_sub(half_height);
		let right = x + half_width;
		let bottom = y + half_height;

		Self {
			left,
			top,
			right,
			bottom,

			current_side: CurrentSide::Right,
			current_x: right,
			current_y: top,
			dx: 0,
			dy: 1,
		}
	}

	pub fn new_from_bounds(left: u32, top: u32, right: u32, bottom: u32) -> Self {
		Self {
			left, top, right, bottom,
			current_side: CurrentSide::Right,
			current_x: right,
			current_y: top,
			dx: 0,
			dy: 1,
		}
	}
}

impl Iterator for RectangleBoundsIterator {
	type Item = (u32, u32);

	fn next(&mut self) -> Option<Self::Item> {
		// TODO: There is definitely a more elegant way to do this.
		let next_tuple = (self.current_x, self.current_y);

		if self.dx == 0 && self.dy == 0 {
			// Sentinel values are perhaps an anti-pattern...
			return None;
		}
		self.current_x += self.dx as u32;
		self.current_y += self.dy as u32;

		if self.current_x > self.right || self.current_y > self.bottom { // Less than or equal on y so we don't duplicate points.
			// We finished:
			match self.current_side {
				CurrentSide::Right => { self.current_side = CurrentSide::Top; self.dx = 1; self.dy = 0; self.current_x = self.left; self.current_y = self.top; },
				CurrentSide::Top => { self.current_side = CurrentSide::Left; self.dx = 0; self.dy = 1; self.current_x = self.left; self.current_y = self.top + 1; }, // Don't repeat the 0,0.
				CurrentSide::Left => { self.current_side = CurrentSide::Bottom; self.dx = 1; self.dy = 0; self.current_x = self.left+1; self.current_y = self.bottom; },
				CurrentSide::Bottom => { self.dx = 0; self.dy = 0; },
			};
		}

		Some(next_tuple)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_sanity() {
		let mut rect_iter = RectangleBoundsIterator::new_from_center(2, 2, 1, 1);
		let points = rect_iter.into_iter().collect::<Vec<(u32, u32)>>();
		assert_eq!(points, vec![
			(3, 1), (3, 2), (3, 3), // Right
			(1, 1), (2, 1), (3, 1), // Top
			(1, 2), (1, 3), // Left
			(2, 3), (3, 3), // TODO: BOTTOM REPEATS (3, 3).  Fix later.
		]);
	}
}