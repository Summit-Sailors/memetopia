use dioxus::prelude::*;
use gloo::utils::document;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlAnchorElement, HtmlCanvasElement, HtmlImageElement};

const MEME_CANVAS_ID: &str = "meme-canvas-id";

fn get_meme_canvas() -> Option<HtmlCanvasElement> {
	document().get_element_by_id(MEME_CANVAS_ID).and_then(|elem| elem.dyn_into::<HtmlCanvasElement>().ok())
}

fn draw_text_to_canvas_ctx(ctx: &CanvasRenderingContext2d, text: String, x: f64, y: f64) {
	ctx.set_font("bold 48px Arial");
	ctx.set_fill_style_str("white");
	ctx.set_stroke_style_str("black");
	ctx.set_line_width(3.0);
	ctx.set_text_align("center");
	ctx.stroke_text(&text, x, y).ok();
	ctx.fill_text(&text, x, y).ok();
}

#[derive(Clone, PartialEq, Debug, Store)]
struct TextBox {
	text: String,
	pos_x: f64,
	pos_y: f64,
}

impl TextBox {
	fn new(text: String, pos_x: f64, pos_y: f64) -> Self {
		Self { text, pos_x, pos_y }
	}
}

#[derive(Clone, PartialEq, Debug, Store)]
struct CanvasObjects {
	main_img_url: String,
	width: u32,
	height: u32,
	text_boxes: Vec<TextBox>,
}

impl CanvasObjects {
	fn new(width: u32, height: u32) -> Self {
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

fn render_canvas(canvas_objects: Store<CanvasObjects>) {
	let CanvasObjectsStoreTransposed { main_img_url, text_boxes, .. } = canvas_objects.transpose();
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
									for text_box in &text_boxes {
										draw_text_to_canvas_ctx(&ctx, text_box.text.clone(), text_box.pos_x, text_box.pos_y);
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

fn download_canvas_as_image() {
	if let Some(canvas) = get_meme_canvas() {
		match canvas.to_data_url() {
			Ok(data_url) => {
				if let Ok(document) = web_sys::window().unwrap().document().ok_or("no document") {
					match document.create_element("a") {
						Ok(anchor_elem) => {
							if let Ok(anchor) = anchor_elem.dyn_into::<HtmlAnchorElement>() {
								anchor.set_href(&data_url);
								anchor.set_download("meme.png");
								if let Some(body) = document.body() {
									body.append_child(&anchor).ok();
									anchor.click();
									body.remove_child(&anchor).ok();
								}
							}
						},
						Err(e) => {
							error!("Failed to create anchor element: {e:#?}");
						},
					}
				}
			},
			Err(e) => {
				error!("Failed to convert canvas to data URL: {e:#?}");
			},
		}
	} else {
		error!("Canvas not found for download");
	}
}

#[component]
pub fn Generator() -> Element {
	let canvas_objects = use_store(|| CanvasObjects::new(500, 500));

	use_effect(move || render_canvas(canvas_objects));
	rsx! {
		div { class: "max-w-6xl mx-auto p-6 min-h-screen",
			div { class: "mb-8",
				h1 { class: "text-3xl font-bold text-center mb-2", "Meme Generator" }
				p { class: "text-center", "Create your own memes with custom text" }
			}
			div { class: "flex flex-col lg:flex-row gap-8 items-start",
				// Canvas section
				div { class: "flex justify-center",
					canvas { id: MEME_CANVAS_ID, width: 500, height: 500 }
				}
				// Controls section
				div { class: "w-full lg:w-80 rounded-xl shadow-lg p-6",
					h2 { class: "text-xl font-semibold mb-6 text-center", "Customize Text" }
					div { class: "space-y-4",
						for mut text_box in canvas_objects.text_boxes().iter() {
							input {
								r#type: "text",
								value: "{text_box.read().text}",
								oninput: move |evt| {
										let mut w = text_box.write();
										w.text = evt.value();
								},
								class: "w-full px-4 py-3 text-base border-2 rounded-lg focus:outline-none transition-all duration-200",
							}
						}
						button {
							onclick: |_| download_canvas_as_image(),
							class: "w-full cursor-pointer font-semibold py-3 px-4 rounded-lg transition-colors duration-200 shadow-md hover:shadow-lg",
							"Download"
						}
					}
				}
			}
		}
	}
}
