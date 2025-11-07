use std::sync::LazyLock;

use crate::stores::meme_canvas::use_meme_canvas;
use crate::stores::text_box::TextBox;
use crate::utils::MEME_CANVAS_ID;
use crate::{
	stores::meme_canvas::{MemeCanvasStoreExt, MemeCanvasStoreImplExt},
	utils::download_canvas_as_image,
};
use dioxus::prelude::*;

const DEFAULT_WIDTH: u32 = 500;
const DEFAULT_HEIGHT: u32 = 500;
const DEFAULT_IMG_URL: &str = "https://i.imgflip.com/4/30b1gx.jpg";
static DEFAULT_TEXT_BOXES: LazyLock<Vec<TextBox>> = LazyLock::new(|| {
	vec![
		TextBox::new("top text".to_owned(), 0.75 * DEFAULT_WIDTH as f64, 0.25 * DEFAULT_HEIGHT as f64, 48, "Arial".to_owned(), "bold".to_owned()),
		TextBox::new("bottom text".to_owned(), 0.75 * DEFAULT_WIDTH as f64, 0.75 * DEFAULT_HEIGHT as f64, 48, "Arial".to_owned(), "bold".to_owned()),
	]
});

#[component]
pub fn Generator() -> Element {
	let mut meme_canvas_store = use_meme_canvas(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_IMG_URL.to_owned(), DEFAULT_TEXT_BOXES.clone());
	let mut main_img_url = meme_canvas_store.main_img_url();

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
            onmousedown: move |e| meme_canvas_store.mouse_down(e),
            onmousemove: move |e| meme_canvas_store.mouse_move(e),
            onmouseup: move |e| meme_canvas_store.mouse_up(e),
            onmouseleave: move |e| meme_canvas_store.mouse_leave(e),
          }
        }
        div { class: "w-full lg:w-80 rounded-xl shadow-lg p-6",
          div { class: "space-y-4",
            div { class: "space-y-2",
              input {
                r#type: "url",
                value: "{main_img_url}",
                oninput: move |evt| main_img_url.set(evt.value()),
                placeholder: "Enter image URL...",
                class: "w-full px-4 py-3 text-base border-2 rounded-lg focus:outline-none transition-all duration-200",
              }
            }
            hr { class: "border-gray-300" }
            for (index , mut text_box) in meme_canvas_store.text_boxes().iter().enumerate() {
              div { class: "border rounded-lg p-3 space-y-2",
                input {
                  r#type: "text",
                  value: "{text_box().text}",
                  oninput: move |evt| text_box.write().text = evt.value(),
                  class: "w-full px-4 py-3 text-base border-2 rounded-lg focus:outline-none transition-all duration-200",
                }
                button {
                  onclick: move |_| meme_canvas_store.remove_text_box(index),
                  class: "px-2 py-1 bg-red-500 rounded text-xs hover:bg-red-600 transition-colors duration-200",
                  "Remove"
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
