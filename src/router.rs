use {
	crate::{
		layout::Layout,
		pages::{generator::Generator, home::Home},
	},
	dioxus::prelude::*,
};

#[derive(Clone, strum::EnumIter, Eq, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
	#[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/generator")]
    Generator {},
}
