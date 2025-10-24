use dioxus::prelude::*;
use gloo::utils::document;
use std::iter::FromIterator;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub const MEME_CANVAS_ID: &str = "meme-canvas-id";

pub fn get_meme_canvas() -> Option<HtmlCanvasElement> {
	document().get_element_by_id(MEME_CANVAS_ID).and_then(|elem| elem.dyn_into::<HtmlCanvasElement>().ok())
}

pub fn get_canvas_mouse_pos(event: &web_sys::MouseEvent) -> (f64, f64) {
	if let Some(canvas) = get_meme_canvas() {
		let rect = canvas.get_bounding_client_rect();
		let x = event.client_x() as f64 - rect.left();
		let y = event.client_y() as f64 - rect.top();
		(x, y)
	} else {
		(0.0, 0.0)
	}
}

#[derive(Clone, PartialEq, Debug, Store)]
pub struct TextBox {
	pub text: String,
	pub pos_x: f64,
	pub pos_y: f64,
	pub font_size: u32,
	pub rotation: f64, // rotation in radians
	pub scale_x: f64,
	pub scale_y: f64,
	pub is_selected: bool,
}

const HANDLE_SIZE: f64 = 8.0;

impl TextBox {
	pub fn new(text: String, pos_x: f64, pos_y: f64, font_size: u32) -> Self {
		// pos_y += font_size as f64 / 2_f64;
		Self { text, pos_x, pos_y, font_size, rotation: 0.0, scale_x: 1.0, scale_y: 1.0, is_selected: false }
	}

	pub fn get_text_bounds(&self, use_scaling: bool) -> (f64, f64, f64, f64) {
		let text_width =
			self.text.len() as f64 * self.font_size as f64 * 0.6 * if use_scaling { self.scale_x } else { 1_f64 };
		let text_height = self.font_size as f64 * if use_scaling { self.scale_y } else { 1_f64 };
		let left = self.pos_x - text_width / 2.0;
		let top = self.pos_y - text_height / 2.0;
		let right = self.pos_x + text_width / 2.0;
		let bottom = self.pos_y + text_height / 2.0;
		(left, top, right, bottom)
	}

	fn draw_to_canvas(&self, ctx: &CanvasRenderingContext2d) {
		ctx.save();

		// Apply transformations
		ctx.translate(self.pos_x, self.pos_y).ok();
		ctx.rotate(self.rotation).ok();
		ctx.scale(self.scale_x, self.scale_y).ok();

		// Draw text
		ctx.set_font(&format!("bold {}px Arial", self.font_size));
		ctx.set_fill_style_str("white");
		ctx.set_stroke_style_str("black");
		ctx.set_line_width(3.0);
		ctx.set_text_align("center");
		ctx.set_text_baseline("middle");
		ctx.stroke_text(&self.text, 0.0, 0.0).ok();
		ctx.fill_text(&self.text, 0.0, 0.0).ok();

		ctx.restore();

		if self.is_selected {
			self.draw_selection_handles(ctx);
		}
	}

	fn draw_selection_handles(&self, ctx: &CanvasRenderingContext2d) {
		let (left, top, right, bottom) = self.get_text_bounds(true);

		ctx.save();
		ctx.translate(self.pos_x, self.pos_y).ok();
		ctx.rotate(self.rotation).ok();
		ctx.translate(-self.pos_x, -self.pos_y).ok();

		// Draw dotted rectangle
		ctx.set_stroke_style_str("#0066ff");
		ctx.set_line_width(1.0);
		ctx.set_line_dash(&js_sys::Array::from_iter([JsValue::from_f64(5.0), JsValue::from_f64(5.0)])).ok();

		ctx.stroke_rect(left, top, right - left, bottom - top);

		// Reset line dash for handles
		ctx.set_line_dash(&js_sys::Array::new()).ok();
		ctx.set_fill_style_str("#0066ff");
		ctx.set_stroke_style_str("#ffffff");
		ctx.set_line_width(2.0);

		// Corner handles (resize)
		let handles = [
			(left, top),     // top-left
			(right, top),    // top-right
			(right, bottom), // bottom-right
			(left, bottom),  // bottom-left
		];

		for (x, y) in handles {
			ctx.fill_rect(x - HANDLE_SIZE / 2.0, y - HANDLE_SIZE / 2.0, HANDLE_SIZE, HANDLE_SIZE);
			ctx.stroke_rect(x - HANDLE_SIZE / 2.0, y - HANDLE_SIZE / 2.0, HANDLE_SIZE, HANDLE_SIZE);
		}

		// Rotation handle (circle above the text box)
		let rotation_handle_x = (left + right) / 2.0;
		let rotation_handle_y = top - 20.0;

		ctx.begin_path();
		ctx.arc(rotation_handle_x, rotation_handle_y, HANDLE_SIZE / 2.0, 0.0, 2.0 * std::f64::consts::PI).ok();
		ctx.fill();
		ctx.stroke();

		// Draw line connecting rotation handle to text box
		ctx.begin_path();
		ctx.move_to(rotation_handle_x, rotation_handle_y + HANDLE_SIZE / 2.0);
		ctx.line_to(rotation_handle_x, top);
		ctx.stroke();

		ctx.restore();
	}

	fn canvas_to_local_coords(&self, x: f64, y: f64) -> (f64, f64) {
		// Translate to origin (center of text box)
		let translated_x = x - self.pos_x;
		let translated_y = y - self.pos_y;

		// Rotate by the same rotation as the text box
		let cos_rot = (-self.rotation).cos();
		let sin_rot = (-self.rotation).sin();

		let local_x = translated_x * cos_rot - translated_y * sin_rot;
		let local_y = translated_x * sin_rot + translated_y * cos_rot;

		(local_x, local_y)
	}

	pub fn contains_point(&self, x: f64, y: f64) -> bool {
		let (local_x, local_y) = self.canvas_to_local_coords(x, y);

		// Get bounds in local coordinates (unrotated, unscaled)
		let text_width = self.text.len() as f64 * self.font_size as f64 * 0.6 * self.scale_x;
		let text_height = self.font_size as f64 * self.scale_y;

		let left = -text_width / 2.0;
		let right = text_width / 2.0;
		let top = -text_height / 2.0;
		let bottom = text_height / 2.0;

		local_x >= left && local_x <= right && local_y >= top && local_y <= bottom
	}

	pub fn get_handle_at_position(&self, x: f64, y: f64) -> Option<HandleType> {
		if !self.is_selected {
			return None;
		}

		let (local_x, local_y) = self.canvas_to_local_coords(x, y);

		// Get bounds in local coordinates (including scaling)
		let text_width = self.text.len() as f64 * self.font_size as f64 * 0.6 * self.scale_x;
		let text_height = self.font_size as f64 * self.scale_y;

		let left = -text_width / 2.0;
		let right = text_width / 2.0;
		let top = -text_height / 2.0;
		let bottom = text_height / 2.0;

		let tolerance = HANDLE_SIZE / 2.0;

		// Check rotation handle (above the text box in local coordinates)
		let rotation_handle_x = 0.0; // Center of text box
		let rotation_handle_y = top - 20.0;

		if (local_x - rotation_handle_x).abs() <= tolerance && (local_y - rotation_handle_y).abs() <= tolerance {
			return Some(HandleType::Rotate);
		}

		// Check corner handles in local coordinates
		let handles = [
			(left, top, HandleType::ResizeTopLeft),
			(right, top, HandleType::ResizeTopRight),
			(right, bottom, HandleType::ResizeBottomRight),
			(left, bottom, HandleType::ResizeBottomLeft),
		];

		for (hx, hy, handle_type) in handles {
			if (local_x - hx).abs() <= tolerance && (local_y - hy).abs() <= tolerance {
				return Some(handle_type);
			}
		}

		None
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandleType {
	ResizeTopLeft,
	ResizeTopRight,
	ResizeBottomRight,
	ResizeBottomLeft,
	Rotate,
}

#[store(pub)]
impl<Lens> Store<TextBox, Lens> {}

#[derive(Clone, PartialEq, Debug, Store)]
pub struct CanvasObjects {
	pub main_img_url: String,
	pub width: u32,
	pub height: u32,
	pub text_boxes: Vec<TextBox>,
	pub selected_index: Option<usize>,
}

impl CanvasObjects {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			main_img_url: "https://i.imgflip.com/4/30b1gx.jpg".to_owned(),
			width,
			height,
			text_boxes: vec![
				TextBox::new("top text".to_owned(), (3_f64 / 4_f64) * width as f64, (1_f64 / 4_f64) * height as f64, 48),
				TextBox::new("bottom text".to_owned(), (3_f64 / 4_f64) * width as f64, (3_f64 / 4_f64) * height as f64, 48),
			],
			selected_index: None,
		}
	}
}

#[store(pub)]
impl<Lens> Store<CanvasObjects, Lens> {
	fn get_text_box_at_position(&self, x: f64, y: f64) -> Option<usize> {
		// Iterate in reverse order to check top-most text boxes first
		for (index, text_box) in self.text_boxes().iter().enumerate().rev() {
			let tb = text_box.read();
			if tb.contains_point(x, y) {
				return Some(index);
			}
		}
		None
	}

	fn select_text_box(&mut self, index: Option<usize>) {
		let CanvasObjectsStoreTransposed { mut text_boxes, mut selected_index, .. } = self.transpose();

		// Deselect all text boxes
		for mut text_box in text_boxes.iter() {
			text_box.write().is_selected = false;
		}

		// Select the specified text box
		if let Some(idx) = index
			&& let Some(mut text_box) = text_boxes.get_mut(idx)
		{
			text_box.is_selected = true;
		}

		*selected_index.write() = index;
	}

	fn add_text_box(&mut self) {
		let CanvasObjectsStoreTransposed { width, height, mut text_boxes, .. } = self.transpose();
		text_boxes.push(TextBox::new("new text".to_owned(), width() as f64 / 2.0, height() as f64 / 2.0, 48));
	}

	fn remove_text_box(&mut self, index: usize) {
		let CanvasObjectsStoreTransposed { mut text_boxes, .. } = self.transpose();
		if text_boxes.len() > 1 && index < text_boxes.len() {
			text_boxes.remove(index);
		}
	}
	fn render_canvas(&self) {
		let CanvasObjectsStoreTransposed { main_img_url, .. } = self.transpose();
		let text_boxes = self.text_boxes();
		let text_boxes = text_boxes();
		let main_img_url = main_img_url();
		match get_meme_canvas() {
			Some(meme_canvas) => match meme_canvas.get_context("2d") {
				Ok(Some(ctx)) => match ctx.dyn_into::<CanvasRenderingContext2d>() {
					Ok(ctx) => match HtmlImageElement::new() {
						Ok(img_elem) => {
							img_elem.set_cross_origin(Some("anonymous"));
							let img_clone = img_elem.clone();
							let onload = Closure::wrap(Box::new(move || {
								let canvas_width = meme_canvas.width() as f64;
								let canvas_height = meme_canvas.height() as f64;
								ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
								match ctx.draw_image_with_html_image_element_and_dw_and_dh(
									&img_clone,
									0.0,
									0.0,
									canvas_width,
									canvas_height,
								) {
									Ok(()) => {
										for text_box in text_boxes.clone() {
											text_box.draw_to_canvas(&ctx);
										}
									},
									Err(e) => {
										error!("{e:#?}");
									},
								}
							}) as Box<dyn Fn()>);
							img_elem.set_onload(Some(onload.as_ref().unchecked_ref()));
							onload.forget();
							img_elem.set_src(&main_img_url);
						},
						Err(e) => {
							error!("{e:#?}");
						},
					},
					Err(e) => {
						error!("{e:#?}");
					},
				},
				Ok(None) => {
					error!("none ctx");
				},
				Err(e) => {
					error!("{e:#?}");
				},
			},
			None => {
				error!("no meme canvas :/");
			},
		}
	}
}
