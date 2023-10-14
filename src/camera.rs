use bevy_ecs::prelude::Resource;

#[derive(Clone, Debug, Default, Resource)]
pub struct Camera {
	pub center_x: u32,
	pub center_y: u32,
	pub width: u32,
	pub height: u32,
}

impl Camera {
	pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
		Camera {
			center_x: x,
			center_y: y,
			width: w,
			height: h,
		}
	}

	pub fn get_frustum(&self) -> (u32, u32, u32, u32) {
		// Left, Top, Right, Bottom
		let half_width = self.width / 2;
		let half_height = self.height / 2;
		(
			self.center_x.saturating_sub(half_width),
			self.center_y.saturating_sub(half_height),
			self.center_x.saturating_add(half_width),
			self.center_y.saturating_add(half_height),
		)
	}

	pub fn teleport_to(&mut self, x: u32, y: u32) {
		self.center_x = x;
		self.center_y = y;
	}
}
