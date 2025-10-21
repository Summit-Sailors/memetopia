use dioxus::prelude::*;
use web_sys::HtmlAnchorElement;
use web_sys::wasm_bindgen::JsCast;

use crate::stores::canvas_objects::get_meme_canvas;

pub fn download_canvas_as_image() {
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
