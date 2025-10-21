use dioxus::prelude::*;

use crate::{
	stores::canvas_objects::{
		CanvasObjects, CanvasObjectsStoreExt, CanvasObjectsStoreImplExt, MEME_CANVAS_ID, get_canvas_mouse_pos,
	},
	utils::download_canvas_as_image,
};

#[component]
pub fn Generator() -> Element {
	let mut canvas_objects = use_store(|| CanvasObjects::new(500, 500));
	let mut dragging = use_signal(|| false);
	let mut drag_index = use_signal(|| None::<usize>);
	let mut drag_offset = use_signal(|| (0.0, 0.0)); // Offset from mouse to text box origin

	use_effect(move || canvas_objects.render_canvas());
	rsx! {
		div { class: "max-w-6xl mx-auto p-6 min-h-screen",
			div { class: "mb-8",
				h1 { class: "text-3xl font-bold text-center mb-2", "Meme Generator" }
				p { class: "text-center", "Create your own memes with custom text" }
			}
			div { class: "flex flex-col lg:flex-row gap-8 items-start",
				// Canvas section
				div { class: "flex justify-center",
					canvas {
						id: MEME_CANVAS_ID,
						width: 500,
						height: 500,
						onmousedown: move |e| {
								let (x, y) = get_canvas_mouse_pos(e.downcast::<web_sys::MouseEvent>().unwrap());
								if let Some(index) = canvas_objects.get_text_box_at_position(x, y) {
										let tb = canvas_objects.text_boxes().get(index).unwrap();
										let text_box = tb.read();
										let offset_x = x - text_box.pos_x;
										let offset_y = y - text_box.pos_y;
										*dragging.write() = true;
										*drag_index.write() = Some(index);
										*drag_offset.write() = (offset_x, offset_y);
										e.prevent_default();
								}
						},
						onmousemove: move |e| {
								if dragging() && let Some(index) = drag_index() {
										let (mouse_x, mouse_y) = get_canvas_mouse_pos(
												e.downcast::<web_sys::MouseEvent>().unwrap(),
										);
										let (offset_x, offset_y) = drag_offset();
										if let Some(mut text_box) = canvas_objects.text_boxes().get_mut(index) {
												text_box.pos_x = mouse_x - offset_x;
												text_box.pos_y = mouse_y - offset_y;
												text_box.pos_x = text_box.pos_x.clamp(0.0, 500.0);
												text_box.pos_y = text_box
														.pos_y
														.max(text_box.font_size as f64)
														.min(500.0);
										}
										canvas_objects.render_canvas();
								}
						},
						onmouseup: move |_| {
								*dragging.write() = false;
								*drag_index.write() = None;
								*drag_offset.write() = (0.0, 0.0);
						},
						onmouseleave: move |_| {
								*dragging.write() = false;
								*drag_index.write() = None;
								*drag_offset.write() = (0.0, 0.0);
						},
					}
				}
				// Controls section
				div { class: "w-full lg:w-80 rounded-xl shadow-lg p-6",
					h2 { class: "text-xl font-semibold mb-6 text-center", "Customize Text" }
					div { class: "space-y-4",
						div { class: "space-y-2",
							label { class: "block text-sm font-medium", "Meme Template Url:" }
							input {
								r#type: "url",
								value: "{canvas_objects.main_img_url()}",
								oninput: move |evt| {
										*canvas_objects.main_img_url().write() = evt.value();
								},
								placeholder: "Enter image URL...",
								class: "w-full px-4 py-3 text-base border-2 rounded-lg focus:outline-none transition-all duration-200",
							}
						}
						hr { class: "border-gray-300" }
						for (index , mut text_box) in canvas_objects.text_boxes().iter().enumerate() {
							div { class: "border rounded-lg p-3 space-y-2",
								div { class: "flex justify-between items-center",
									span { class: "text-sm font-medium text-gray-700",
										"Text Box {index + 1}"
									}
									if canvas_objects.text_boxes().len() > 1 {
										button {
											onclick: move |_| canvas_objects.remove_text_box(index),
											class: "px-2 py-1 bg-red-500 rounded text-xs hover:bg-red-600 transition-colors duration-200",
											"Remove"
										}
									}
								}
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
						}
						button {
							onclick: move |_| canvas_objects.add_text_box(),
							class: "px-3 py-1 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors duration-200 text-sm font-medium",
							"Add Text"
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
