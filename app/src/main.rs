#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;

fn main() {
	dioxus::logger::initialize_default();
	dioxus::LaunchBuilder::new().launch(|| {
		rsx! {
			ToastProvider { App {} }
		}
	});
}

#[component]
fn App() -> Element {
	rsx! {
		main { class: "max-w-7xl mx-auto p-4 font-sans text-gray-800" }
	}
}
