use gloo::utils::document;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlAnchorElement, HtmlCanvasElement};

pub const MEME_CANVAS_ID: &str = "meme-canvas-id";

pub fn get_meme_canvas() -> HtmlCanvasElement {
	document().get_element_by_id(MEME_CANVAS_ID).and_then(|elem| elem.dyn_into::<HtmlCanvasElement>().ok()).expect("cant get meme canvas")
}

pub fn get_meme_canvas_ctx() -> CanvasRenderingContext2d {
	get_meme_canvas().get_context("2d").expect("canvas context").expect("canvas context").dyn_into::<CanvasRenderingContext2d>().expect("canvas context")
}

pub fn download_canvas_as_image() {
	let canvas = get_meme_canvas();
	let data_url = canvas.to_data_url().expect("data_url");
	let dom = web_sys::window().unwrap().document().expect("no document");
	let body = dom.body().expect("");
	let anchor = dom.create_element("a").expect("").dyn_into::<HtmlAnchorElement>().expect("");
	anchor.set_href(&data_url);
	anchor.set_download("meme.png");
	body.append_child(&anchor).ok();
	anchor.click();
	body.remove_child(&anchor).ok();
}
