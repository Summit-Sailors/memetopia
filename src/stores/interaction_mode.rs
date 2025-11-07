use crate::stores::text_box::HandleType;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Store)]
pub enum InteractionMode {
	None,
	Dragging { index: usize, offset: (f64, f64) },
	Resizing { index: usize, handle: HandleType, start_pos: (f64, f64) },
	Rotating { index: usize, start_angle: f64 },
}

#[store(pub)]
impl<Lens> Store<InteractionMode, Lens> {
	fn set_dragging(&mut self, index: usize, offset: (f64, f64)) {
		self.set(InteractionMode::Dragging { index, offset });
	}

	fn set_resize(&mut self, index: usize, handle: HandleType, start_pos: (f64, f64)) {
		self.set(InteractionMode::Resizing { index, handle, start_pos });
	}

	fn set_rotating(&mut self, index: usize, start_angle: f64) {
		self.set(InteractionMode::Rotating { index, start_angle });
	}
}
