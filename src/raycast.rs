use glam::{IVec2, UVec2};

pub struct Raycast {
	start_x: i32,
	start_y: i32,

	next_x: i32,
	next_y: i32,

	dx: f32,
	dy: f32,

	step: u32,
	step_count: u32,
}

impl Iterator for Raycast {
	type Item = (i32, i32);

	fn next(&mut self) -> Option<Self::Item> {
		// TODO: Replace with Bresenham's algorithm.
		let ret = Some((self.next_x, self.next_y));
		self.step += 1;

		if self.step > self.step_count {
			return None;
		}

		self.next_x = self.start_x + (self.dx * self.step as f32).round() as i32;
		self.next_y = self.start_y + (self.dy * self.step as f32).round() as i32;

		ret
	}
}

impl Raycast {
	pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
		let dx = x1 - x0;
		let dy = y1 - y0;
		let step_count = dx.abs().max(dy.abs()) as u32;

		Self {
			start_x: x0,
			start_y: y0,

			// Returned from iterator and tracked:
			next_x: x0,
			next_y: y0,

			// Used to compute the next step:
			step: 0,
			step_count: step_count,
			dx: dx as f32 / step_count as f32,
			dy: dy as f32 / step_count as f32,
		}
	}

	pub fn from_ivec(start: IVec2, destination: IVec2) -> Self {
		Self::new(start.x, start.y, destination.x, destination.y)
	}

	pub fn from_uvec(start: UVec2, destination: UVec2) -> Self {
		Self::new(start.x as i32, start.y as i32, destination.x as i32, destination.y as i32)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_horizontal_ray() {
		let rc = Raycast::new(0, 0, 5, 0);
		let steps = rc.into_iter().collect::<Vec<(i32, i32)>>();
		assert_eq!(steps, vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
	}

	#[test]
	fn check_vertical_ray() {
		let rc = Raycast::new(1, 1, 1, 4);
		let steps = rc.into_iter().collect::<Vec<(i32, i32)>>();
		assert_eq!(steps, vec![(1, 1), (1, 2), (1, 3)]);
	}

	#[test]
	fn check_leftdown_ray() {
		let rc = Raycast::new(2, 3, -2, 2);
		let steps = rc.into_iter().collect::<Vec<(i32, i32)>>();
		assert_eq!(steps, vec![(2, 3), (1, 3), (0, 2), (-1, 2)]);
	}
}
