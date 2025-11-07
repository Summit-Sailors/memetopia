use crate::stores::interaction_mode::InteractionMode;
use crate::stores::interaction_mode::InteractionModeStoreImplExt;
use crate::stores::text_box::HandleType;
use crate::stores::text_box::TextBounds;
use crate::stores::text_box::TextBox;
use crate::utils::get_meme_canvas;
use crate::utils::get_meme_canvas_ctx;
use dioxus::html::geometry::euclid::Point2D;
use dioxus::prelude::*;
use web_sys::HtmlImageElement;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq, Debug, Store)]
pub struct MemeCanvas {
	pub main_img_url: String,
	pub width: u32,
	pub height: u32,
	pub text_boxes: Vec<TextBox>,
	pub selected_index: Option<usize>,
	pub interaction_mode: InteractionMode,
}

impl MemeCanvas {
	pub fn new(width: u32, height: u32, main_img_url: String, text_boxes: Vec<TextBox>) -> Self {
		Self { main_img_url, width, height, text_boxes, selected_index: None, interaction_mode: InteractionMode::None }
	}
}

#[store(pub)]
impl<Lens> Store<MemeCanvas, Lens> {
	fn select_handle(&mut self, x: f64, y: f64) -> Option<HandleType> {
		if let Some(selected_idx) = self.selected_index()()
			&& let Some(text_box) = self.text_boxes()().get(selected_idx)
			&& let Some(handle_type) = text_box.get_handle_at_position(x, y)
		{
			match handle_type {
				HandleType::Rotate => {
					let start_angle = (y - text_box.y).atan2(x - text_box.x) - text_box.rotation;
					self.interaction_mode().set_rotating(selected_idx, start_angle);
				},
				_ => {
					self.interaction_mode().set_resize(selected_idx, handle_type, (x, y));
				},
			}
			return Some(handle_type);
		}
		None
	}

	fn get_textbox_position(&self, index: usize) -> (f64, f64) {
		let tb = self.text_boxes().get(index).expect("no tc at index")();
		(tb.x, tb.y)
	}

	fn maybe_select_text_box(&mut self, x: f64, y: f64) {
		if let Some(index) = self.get_text_box_at_position(x, y) {
			self.select_text_box(Some(index));
			let (tb_x, tb_y) = self.get_textbox_position(index);
			self.interaction_mode().set(InteractionMode::Dragging { index, offset: (x - tb_x, y - tb_y) });
		} else {
			self.select_text_box(None);
			self.interaction_mode().set(InteractionMode::None);
		}
	}

	fn mouse_down(&mut self, e: Event<MouseData>) {
		e.prevent_default();
		e.stop_propagation();
		let Point2D { x, y, .. } = e.element_coordinates();
		if self.select_handle(x, y).is_none() {
			self.maybe_select_text_box(x, y);
		}
	}
	fn get_text_box_at_position(&self, x: f64, y: f64) -> Option<usize> {
		self.text_boxes().iter().enumerate().rev().find_map(|(index, text_box)| if text_box().contains_point(x, y) { Some(index) } else { None })
	}

	fn mouse_up(&mut self, e: Event<MouseData>) {
		e.prevent_default();
		e.stop_propagation();
		self.interaction_mode().set(InteractionMode::None);
	}

	fn mouse_leave(&mut self, e: Event<MouseData>) {
		e.prevent_default();
		e.stop_propagation();
		self.interaction_mode().set(InteractionMode::None);
	}

	fn mouse_move(&mut self, e: Event<MouseData>) {
		e.prevent_default();
		e.stop_propagation();
		let Point2D { x: mouse_x, y: mouse_y, .. } = e.element_coordinates();
		match self.interaction_mode()() {
			InteractionMode::Dragging { index, offset } => {
				let (offset_x, offset_y) = offset;
				if let Some(mut text_box) = self.text_boxes().get_mut(index) {
					text_box.x = (mouse_x - offset_x).clamp(0.0, 500.0);
					text_box.y = text_box.y.max(text_box.style.size as f64).min(500.0);
					text_box.y = (mouse_y - offset_y).max(text_box.style.size as f64).min(500.0);
				}
				self.render_canvas();
			},
			InteractionMode::Rotating { index, start_angle } => {
				if let Some(mut text_box) = self.text_boxes().get_mut(index) {
					let current_angle = (mouse_y - text_box.y).atan2(mouse_x - text_box.x);
					text_box.rotation = current_angle - start_angle;
				}
				self.render_canvas();
			},
			InteractionMode::Resizing { index, handle, start_pos } => {
				if let Some(mut text_box) = self.text_boxes().get_mut(index) {
					let (start_x, start_y) = start_pos;
					let dx = mouse_x - start_x;
					let dy = mouse_y - start_y;
					let TextBounds { left, top, right, bottom } = text_box.get_text_bounds(false);
					match handle {
						HandleType::ResizeBottomRight => {
							text_box.scale_x = ((right - left) + 2.0 * dx) / (right - left);
							text_box.scale_y = ((bottom - top) + 2.0 * dy) / (bottom - top);
						},
						HandleType::ResizeTopLeft => {
							text_box.scale_x = ((right - left) - 2.0 * dx) / (right - left);
							text_box.scale_y = ((bottom - top) - 2.0 * dy) / (bottom - top);
						},
						_ => {},
					}
				}
				self.render_canvas();
			},
			InteractionMode::None => {},
		}
	}

	fn unselect_text_boxes(&mut self) {
		for mut text_box in self.text_boxes().iter() {
			text_box.write().is_selected = false;
		}
	}

	fn select_text_box(&mut self, index: Option<usize>) {
		let MemeCanvasStoreTransposed { mut text_boxes, mut selected_index, .. } = self.transpose();
		self.unselect_text_boxes();
		if let Some(idx) = index
			&& let Some(mut text_box) = text_boxes.get_mut(idx)
		{
			text_box.is_selected = true;
		}
		*selected_index.write() = index;
	}

	fn add_text_box(&mut self) {
		let MemeCanvasStoreTransposed { width, height, mut text_boxes, .. } = self.transpose();
		text_boxes.push(TextBox::new("new text".to_owned(), width() as f64 / 2.0, height() as f64 / 2.0, 48, "Ariel".to_owned(), "bold".to_owned()));
	}

	fn remove_text_box(&mut self, index: usize) {
		let mut text_boxes = self.text_boxes();
		if text_boxes.len() > 1 && index < text_boxes.len() {
			text_boxes.remove(index);
		}
	}
	fn render_canvas(&self) {
		let MemeCanvasStoreTransposed { main_img_url, .. } = self.transpose();
		let text_boxes = self.text_boxes();
		let text_boxes = text_boxes();
		let main_img_url = main_img_url();
		let meme_canvas = get_meme_canvas();
		let ctx = get_meme_canvas_ctx();
		let img_elem = HtmlImageElement::new().expect("cannot create img elem");
		img_elem.set_cross_origin(Some("anonymous"));
		let onload = Closure::wrap(Box::new({
			to_owned![img_elem];
			move || {
				let canvas_width = meme_canvas.width() as f64;
				let canvas_height = meme_canvas.height() as f64;
				ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
				match ctx.draw_image_with_html_image_element_and_dw_and_dh(&img_elem, 0.0, 0.0, canvas_width, canvas_height) {
					Ok(()) => {
						for text_box in text_boxes.clone() {
							text_box.draw_to_canvas(&ctx);
						}
					},
					Err(e) => {
						error!("{e:#?}");
					},
				}
			}
		}) as Box<dyn Fn()>);
		img_elem.set_onload(Some(onload.as_ref().unchecked_ref()));
		onload.forget();
		img_elem.set_src(&main_img_url);
	}
}

pub fn use_meme_canvas(width: u32, height: u32, main_img_url: String, text_boxes: Vec<TextBox>) -> Store<MemeCanvas> {
	let meme_canvas_store = use_store(|| MemeCanvas::new(width, height, main_img_url, text_boxes));
	use_effect(move || meme_canvas_store.render_canvas());
	meme_canvas_store
}
