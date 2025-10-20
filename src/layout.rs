use {
	crate::router::Route,
	dioxus::prelude::*,
	dioxus_free_icons::{Icon, icons::bs_icons::BsHouse},
};

#[component]
pub fn Layout() -> Element {
	rsx! {
		div { class: "flex flex-col min-h-screen w-full relative bg-black text-white",
			header { class: "w-full sticky top-0 z-10  right-0 left-0 flex justify-end gap-4 items-center bg-black/80 backdrop-blur-md px-10 py-2 border-b border-b-white/20",
				Link { to: Route::Home {}, class: "mr-auto",
					Icon { icon: BsHouse, width: 24, height: 24 }
				}
			}
			div { class: "flex-1 pt-4", Outlet::<Route> {} }
		}
	}
}
