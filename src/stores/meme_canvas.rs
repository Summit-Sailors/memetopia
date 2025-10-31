use dioxus::prelude::*;
use web_sys::HtmlImageElement;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;

use crate::stores::text_box::HandleType;
use crate::stores::text_box::TextBox;
use crate::utils::get_meme_canvas;
use crate::utils::get_meme_canvas_ctx;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionMode {
	None,
	Dragging { index: usize, offset: (f64, f64) },
	Resizing { index: usize, handle: HandleType, start_pos: (f64, f64) },
	Rotating { index: usize, start_angle: f64 },
}

#[derive(Clone, PartialEq, Debug, Store)]
pub struct MemeCanvas {
	pub main_img_url: String,
	pub width: u32,
	pub height: u32,
	pub text_boxes: Vec<TextBox>,
	pub selected_index: Option<usize>,
	pub mode: InteractionMode,
}

impl MemeCanvas {
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
			mode: InteractionMode::None,
		}
	}
}

#[store(pub)]
impl<Lens> Store<MemeCanvas, Lens> {
	fn get_text_box_at_position(&self, x: f64, y: f64) -> Option<usize> {
		self
			.text_boxes()
			.iter()
			.enumerate()
			.rev()
			.find_map(|(index, text_box)| if text_box().contains_point(x, y) { Some(index) } else { None })
	}

	fn select_text_box(&mut self, index: Option<usize>) {
		let MemeCanvasStoreTransposed { mut text_boxes, mut selected_index, .. } = self.transpose();
		for mut text_box in text_boxes.iter() {
			text_box.write().is_selected = false;
		}
		if let Some(idx) = index
			&& let Some(mut text_box) = text_boxes.get_mut(idx)
		{
			text_box.is_selected = true;
		}
		*selected_index.write() = index;
	}

	fn add_text_box(&mut self) {
		let MemeCanvasStoreTransposed { width, height, mut text_boxes, .. } = self.transpose();
		text_boxes.push(TextBox::new("new text".to_owned(), width() as f64 / 2.0, height() as f64 / 2.0, 48));
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

pub fn use_meme_canvas() -> Store<MemeCanvas> {
	let meme_canvas_store = use_store(|| MemeCanvas::new(500, 500));
	use_effect(move || meme_canvas_store.render_canvas());
	meme_canvas_store
}
