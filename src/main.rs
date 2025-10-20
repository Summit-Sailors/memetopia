use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;
use memetopia::application::App;

fn main() {
	dioxus::LaunchBuilder::new().launch(|| {
		rsx! {
			ToastProvider { App {} }
		}
	});
}
