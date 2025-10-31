use crate::stores::meme_canvas::{MemeCanvasStoreTransposed, use_meme_canvas};
use crate::stores::text_box::{TextBounds, TextBoxStoreExt};
use crate::utils::{MEME_CANVAS_ID, get_canvas_mouse_pos};
use crate::{
	stores::{
		meme_canvas::{InteractionMode, MemeCanvasStoreExt, MemeCanvasStoreImplExt},
		text_box::HandleType,
	},
	utils::download_canvas_as_image,
};
use dioxus::prelude::*;

#[component]
pub fn Generator() -> Element {
	let mut meme_canvas_store = use_meme_canvas();
	let MemeCanvasStoreTransposed { text_boxes, mut mode, selected_index, .. } = meme_canvas_store.transpose();

	rsx! {
		div { class: "max-w-6xl mx-auto p-6 min-h-screen",
			div { class: "mb-8",
				h1 { class: "text-3xl font-bold text-center mb-2", "Meme Generator" }
				p { class: "text-center", "Create your own memes with custom text" }
			}
			div { class: "flex flex-col lg:flex-row gap-8 items-start",
				div { class: "flex justify-center",
					canvas {
						id: MEME_CANVAS_ID,
						width: 500,
						height: 500,
						onmousedown: move |e| {
								e.prevent_default();
								let (x, y) = get_canvas_mouse_pos(e.downcast::<web_sys::MouseEvent>().unwrap());
								if let Some(selected_idx) = selected_index()
										&& let Some(text_box) = text_boxes.get(selected_idx)
										&& let Some(handle_type) = text_box.read().get_handle_at_position(x, y)
								{
										match handle_type {
												HandleType::Rotate => {
														let tb = text_box.read();
														let start_angle = (y - tb.y).atan2(x - tb.x) - tb.rotation;
														*mode.write() = InteractionMode::Rotating {
																index: selected_idx,
																start_angle,
														};
												}
												_ => {
														*mode.write() = InteractionMode::Resizing {
																index: selected_idx,
																handle: handle_type,
																start_pos: (x, y),
														};
												}
										}
										return;
								}
								if let Some(index) = meme_canvas_store.get_text_box_at_position(x, y) {
										meme_canvas_store.select_text_box(Some(index));
										let tb = meme_canvas_store.text_boxes().get(index).expect("no tc at index");
										let text_box = tb.read();
										let offset_x = x - text_box.x;
										let offset_y = y - text_box.y;
										*mode.write() = InteractionMode::Dragging {
												index,
												offset: (offset_x, offset_y),
										};
								} else {
										meme_canvas_store.select_text_box(None);
										*mode.write() = InteractionMode::None;
								}
						},
						onmousemove: move |e| {
								let (mouse_x, mouse_y) = get_canvas_mouse_pos(
										e.downcast::<web_sys::MouseEvent>().unwrap(),
								);
								match mode() {
										InteractionMode::Dragging { index, offset } => {
												let (offset_x, offset_y) = offset;
												if let Some(mut text_box) = meme_canvas_store.text_boxes().get_mut(index)
												{
														text_box.x = (mouse_x - offset_x).clamp(0.0, 500.0);
														text_box.y = text_box
																.y
																.max(text_box.style.font_size as f64)
																.min(500.0);
														text_box.y = (mouse_y - offset_y)
																.max(text_box.style.font_size as f64)
																.min(500.0);
												}
												meme_canvas_store.render_canvas();
										}
										InteractionMode::Rotating { index, start_angle } => {
												if let Some(mut text_box) = meme_canvas_store.text_boxes().get_mut(index)
												{
														let current_angle = (mouse_y - text_box.y)
																.atan2(mouse_x - text_box.x);
														text_box.rotation = current_angle - start_angle;
												}
												meme_canvas_store.render_canvas();
										}
										InteractionMode::Resizing { index, handle, start_pos } => {
												if let Some(mut text_box) = meme_canvas_store.text_boxes().get_mut(index)
												{
														let (start_x, start_y) = start_pos;
														let dx = mouse_x - start_x;
														let dy = mouse_y - start_y;
														let TextBounds { left, top, right, bottom } = text_box
																.get_text_bounds(false);
														match handle {
																HandleType::ResizeBottomRight => {
																		text_box.scale_x = ((right - left) + 2_f64 * dx)
																				/ (right - left);
																		text_box.scale_y = ((bottom - top) + 2_f64 * dy)
																				/ (bottom - top);
																}
																HandleType::ResizeTopLeft => {
																		text_box.scale_x = ((right - left) - 2_f64 * dx)
																				/ (right - left);
																		text_box.scale_y = ((bottom - top) - 2_f64 * dy)
																				/ (bottom - top);
																}
																_ => {}
														}
												}
												meme_canvas_store.render_canvas();
										}
										InteractionMode::None => {}
								}
						},
						onmouseup: move |_| {
								*mode.write() = InteractionMode::None;
						},
						onmouseleave: move |_| {
								*mode.write() = InteractionMode::None;
						},
					}
				}
				div { class: "w-full lg:w-80 rounded-xl shadow-lg p-6",
					h2 { class: "text-xl font-semibold mb-6 text-center", "Customize Text" }
					div { class: "space-y-4",
						div { class: "space-y-2",
							label { class: "block text-sm font-medium", "Meme Template Url:" }
							input {
								r#type: "url",
								value: "{meme_canvas_store.main_img_url()}",
								oninput: move |evt| {
										*meme_canvas_store.main_img_url().write() = evt.value();
								},
								placeholder: "Enter image URL...",
								class: "w-full px-4 py-3 text-base border-2 rounded-lg focus:outline-none transition-all duration-200",
							}
						}
						hr { class: "border-gray-300" }
						for (index , mut text_box) in text_boxes.iter().enumerate() {
							div { class: "border rounded-lg p-3 space-y-2",
								div { class: "flex justify-between items-center",
									span { class: "text-sm font-medium text-gray-700",
										"Text Box {index + 1}"
									}
									if meme_canvas_store.text_boxes().len() > 1 {
										button {
											onclick: move |_| meme_canvas_store.remove_text_box(index),
											class: "px-2 py-1 bg-red-500 rounded text-xs hover:bg-red-600 transition-colors duration-200",
											"Remove"
										}
									}
								}
								input {
									r#type: "text",
									value: "{text_box.text().read()}",
									oninput: move |evt| {
											text_box.write().text = evt.value();
									},
									class: "w-full px-4 py-3 text-base border-2 rounded-lg focus:outline-none transition-all duration-200",
								}
							}
						}
						button {
							onclick: move |_| meme_canvas_store.add_text_box(),
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
