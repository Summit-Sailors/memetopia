use app::application::App;
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
