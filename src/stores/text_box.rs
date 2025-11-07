use dioxus::prelude::*;

use std::iter::FromIterator;
use web_sys::CanvasRenderingContext2d;
use web_sys::wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Store)]
pub struct TextBoxStyle {
	pub size: u32,
	pub family: String,
	pub effect: String,
}

#[derive(Clone, PartialEq, Debug, Store)]
pub struct TextBox {
	pub text: String,
	pub x: f64,
	pub y: f64,
	pub rotation: f64,
	pub scale_x: f64,
	pub scale_y: f64,
	pub style: TextBoxStyle,
	pub is_selected: bool,
}

const HANDLE_SIZE: f64 = 8.0;

#[derive(Debug, Clone, PartialEq)]
pub struct TextBounds {
	pub left: f64,
	pub top: f64,
	pub right: f64,
	pub bottom: f64,
}

impl TextBox {
	pub fn new(text: String, x: f64, y: f64, font_size: u32, font_family: String, font_effect: String) -> Self {
		Self {
			text,
			x,
			y,
			style: TextBoxStyle { size: font_size, effect: font_effect, family: font_family },
			rotation: 0.0,
			scale_x: 1.0,
			scale_y: 1.0,
			is_selected: false,
		}
	}

	pub fn get_bounds(&self, use_scaling: bool) -> TextBounds {
		let text_width = self.text.len() as f64 * self.style.size as f64 * 0.6 * if use_scaling { self.scale_x } else { 1.0 };
		let text_height = self.style.size as f64 * if use_scaling { self.scale_y } else { 1.0 };
		TextBounds { left: self.x - text_width / 2.0, top: self.y - text_height / 2.0, right: self.x + text_width / 2.0, bottom: self.y + text_height / 2.0 }
	}

	pub fn draw_to_canvas(&self, ctx: &CanvasRenderingContext2d) {
		ctx.save();

		ctx.translate(self.x, self.y).ok();
		ctx.rotate(self.rotation).ok();
		ctx.scale(self.scale_x, self.scale_y).ok();

		ctx.set_font(&format!("{} {}px {}", self.style.effect, self.style.size, self.style.family));
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
		let TextBounds { left, top, right, bottom } = self.get_bounds(true);

		ctx.save();
		ctx.translate(self.x, self.y).ok();
		ctx.rotate(self.rotation).ok();
		ctx.translate(-self.x, -self.y).ok();

		ctx.set_stroke_style_str("#0066ff");
		ctx.set_line_width(1.0);
		ctx.set_line_dash(&js_sys::Array::from_iter([JsValue::from_f64(5.0), JsValue::from_f64(5.0)])).ok();

		ctx.stroke_rect(left, top, right - left, bottom - top);

		ctx.set_line_dash(&js_sys::Array::new()).ok();
		ctx.set_fill_style_str("#0066ff");
		ctx.set_stroke_style_str("#ffffff");
		ctx.set_line_width(2.0);

		for (x, y) in [(left, top), (right, top), (right, bottom), (left, bottom)] {
			ctx.fill_rect(x - HANDLE_SIZE / 2.0, y - HANDLE_SIZE / 2.0, HANDLE_SIZE, HANDLE_SIZE);
			ctx.stroke_rect(x - HANDLE_SIZE / 2.0, y - HANDLE_SIZE / 2.0, HANDLE_SIZE, HANDLE_SIZE);
		}

		let rotation_handle_x = (left + right) / 2.0;
		let rotation_handle_y = top - 20.0;

		ctx.begin_path();
		ctx.arc(rotation_handle_x, rotation_handle_y, HANDLE_SIZE / 2.0, 0.0, 2.0 * std::f64::consts::PI).ok();
		ctx.fill();
		ctx.stroke();

		ctx.begin_path();
		ctx.move_to(rotation_handle_x, rotation_handle_y + HANDLE_SIZE / 2.0);
		ctx.line_to(rotation_handle_x, top);
		ctx.stroke();

		ctx.restore();
	}

	fn canvas_to_local_coords(&self, x: f64, y: f64) -> (f64, f64) {
		let translated_x = x - self.x;
		let translated_y = y - self.y;
		let cos_rot = (-self.rotation).cos();
		let sin_rot = (-self.rotation).sin();
		let local_x = translated_x * cos_rot - translated_y * sin_rot;
		let local_y = translated_x * sin_rot + translated_y * cos_rot;
		(local_x, local_y)
	}

	pub fn contains_point(&self, x: f64, y: f64) -> bool {
		let (local_x, local_y) = self.canvas_to_local_coords(x, y);
		let text_width = self.text.len() as f64 * self.style.size as f64 * 0.6 * self.scale_x;
		let text_height = self.style.size as f64 * self.scale_y;
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
		let text_width = self.text.len() as f64 * self.style.size as f64 * 0.6 * self.scale_x;
		let text_height = self.style.size as f64 * self.scale_y;
		let left = -text_width / 2.0;
		let right = text_width / 2.0;
		let top = -text_height / 2.0;
		let bottom = text_height / 2.0;
		let tolerance = HANDLE_SIZE / 2.0;
		let rotation_handle_x = 0.0;
		let rotation_handle_y = top - 20.0;
		if (local_x - rotation_handle_x).abs() <= tolerance && (local_y - rotation_handle_y).abs() <= tolerance {
			return Some(HandleType::Rotate);
		}
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
pub struct MemeCanvas {
	pub main_img_url: String,
	pub width: u32,
	pub height: u32,
	pub text_boxes: Vec<TextBox>,
	pub selected_index: Option<usize>,
}
