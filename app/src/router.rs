use {
	crate::{layout::Layout, pages::home::Home},
	dioxus::prelude::*,
};

#[derive(Clone, PartialEq, strum::EnumIter, Routable)]
#[rustfmt::skip]
pub enum Route {
	#[layout(Layout)]
    #[route("/")]
    Home {},
}
