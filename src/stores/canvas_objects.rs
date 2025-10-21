use dioxus::prelude::*;
use gloo::utils::document;
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
}

impl TextBox {
	pub fn new(text: String, pos_x: f64, pos_y: f64) -> Self {
		Self { text, pos_x, pos_y, font_size: 48 }
	}
}

#[store(pub)]
impl<Lens> Store<TextBox, Lens> {
	fn draw_to_canvas(&self, ctx: &CanvasRenderingContext2d) {
		let TextBoxStoreTransposed { text, pos_x, pos_y, font_size } = self.transpose();
		ctx.set_font(&format!("bold {font_size}px Arial"));
		ctx.set_fill_style_str("white");
		ctx.set_stroke_style_str("black");
		ctx.set_line_width(3.0);
		ctx.set_text_align("center");
		ctx.stroke_text(&text(), pos_x(), pos_y()).ok();
		ctx.fill_text(&text(), pos_x(), pos_y()).ok();
	}
}

#[derive(Clone, PartialEq, Debug, Store)]
pub struct CanvasObjects {
	pub main_img_url: String,
	pub width: u32,
	pub height: u32,
	pub text_boxes: Vec<TextBox>,
}

impl CanvasObjects {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			main_img_url: "https://i.imgflip.com/4/30b1gx.jpg".to_owned(),
			width,
			height,
			text_boxes: vec![
				TextBox::new("top text".to_owned(), (3_f64 / 4_f64) * width as f64, (1_f64 / 4_f64) * height as f64),
				TextBox::new("bottom text".to_owned(), (3_f64 / 4_f64) * width as f64, (3_f64 / 4_f64) * height as f64),
			],
		}
	}
}

#[store(pub)]
impl<Lens> Store<CanvasObjects, Lens> {
	fn get_text_box_at_position(&self, x: f64, y: f64) -> Option<usize> {
		for (index, text_box) in self.text_boxes().iter().enumerate() {
			let tb = text_box.read();
			let text_width = tb.text.len() as f64 * tb.font_size as f64 * 0.6;
			let text_height = tb.font_size as f64;
			if x >= tb.pos_x && x <= tb.pos_x + text_width && y >= tb.pos_y - text_height && y <= tb.pos_y {
				return Some(index);
			}
		}
		None
	}
	fn add_text_box(&mut self) {
		let CanvasObjectsStoreTransposed { width, height, mut text_boxes, .. } = self.transpose();
		text_boxes.push(TextBox::new("new text".to_owned(), width() as f64 / 2.0, height() as f64 / 2.0));
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
										for text_box in text_boxes.iter() {
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
