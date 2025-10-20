use {crate::router::Route, dioxus::prelude::*};

static TAILWIND: Asset = asset!("/assets/tailwind.css", AssetOptions::css());
// static FAVICON: Asset = asset!("/assets/favicon.ico", AssetOptions::image().with_avif());

pub fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: TAILWIND }
		// document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
		document::Link {
			rel: "stylesheet",
			href: "https://fonts.googleapis.com/css2?family=Poppins:ital,wght@0,400;0,500;0,600;0,700;1,400;1,500;1,600;1,700&display=swap",
		}
		Router::<Route> {}
	}
}
