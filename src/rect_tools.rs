
enum CurrentSide {
	None,
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
}

impl RectangleBoundsIterator {
	fn new_from_center(x: u32, y: u32, half_width: u32, half_height: u32) -> Self {
		Self {
			left: x.saturating_sub(half_width),
			top: y.saturating_sub(half_height),
			right: x + half_width,
			bottom: y + half_height,

			current_side: CurrentSide::Right,
			current_x: 0,
			current_y: 0,
		}
	}
}

impl Iterator for RectangleBoundsIterator {
	type Item = (u32, u32);

	fn next(&mut self) -> Option<Self::Item> {
		// TODO: There is definitely a more elegant way to do this.
		let next_tuple = (self.current_x, self.current_y);

		match self.current_side {
			CurrentSide::Right => { self.current_x = self.right; self.current_y += 1; },
			CurrentSide::Top => { self.current_x += 1; self.current_y = self.top; },
			CurrentSide::Left => { self.current_x = self.left; self.current_y += 1; },
			CurrentSide::Bottom => { self.current_x += 1; self.current_y = self.bottom; },
			CurrentSide::None => { return None; }
		}

		if self.current_x > self.right || self.current_y >= self.bottom { // Less than or equal on y so we don't duplicate points.
			self.current_side = match self.current_side {
				CurrentSide::Right => CurrentSide::Top,
				CurrentSide::Top => CurrentSide::Left,
				CurrentSide::Left => CurrentSide::Bottom,
				CurrentSide::Bottom => CurrentSide::None,
				CurrentSide::None => CurrentSide::None
			};
			self.current_x = 0;
			self.current_y = 0;
		}

		Some(next_tuple)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_sanity() {
		let mut rect_iter = RectangleBoundsIterator::new_from_center(5, 5, 3, 1);
		let points = rect_iter.into_iter().collect::<Vec<(u32, u32)>>();
		assert_eq!(points, vec![
			(8, 4), (8, 5), (8, 6), // Right
			(2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), // Top
			// JC: Nope, stopping here.  This is dumb.  New approach.
		]);
	}
}