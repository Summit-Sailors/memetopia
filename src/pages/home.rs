use dioxus::prelude::*;
use std::sync::LazyLock;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoadmapStage {
	pub title: &'static str,
	pub description: &'static str,
	pub items: Vec<StageItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageItem {
	pub title: &'static str,
	pub description: &'static str,
}

static ROADMAP_DATA: LazyLock<Vec<RoadmapStage>> = LazyLock::new(|| {
	vec![
		RoadmapStage {
			title: "Meme Generator",
			description: "The starting point of development. Requires using less common apis.",
			items: vec![
				StageItem { title: "Canvas Implementation", description: "Will start with canvas initially as is more well known and lowers inital complexity." },
				StageItem { title: "WGPU Engine Upgrade", description: "Migrate rendering to WGPU for cross-platform and performance." },
			],
		},
		RoadmapStage {
			title: "Meme Site Basics",
			description: "Implement the most common functionality of a meme site.",
			items: vec![
				StageItem { title: "Database", description: "PostgreSQL" },
				StageItem { title: "DB ORM", description: "Diesel" },
				StageItem { title: "User Authentication", description: "JWT/Cookies" },
				StageItem { title: "Blob Storage", description: "Try self hosted Minio, move to cloudflare R2 when bandwidth is too high." },
				StageItem { title: "File Upload API", description: "signed-urls to post, but need to sync with database." },
			],
		},
		// RoadmapStage {
		// 	title: "Meme Analytics Game",
		// 	description: "Use data science on scrapped memes to allow users to bet on percentile window the meme falls in.",
		// 	items: vec![
		// 		StageItem {
		// 			title: "Scrape Memes",
		// 			description: "Gather large collection of memes from the internet along with their engagement metrics.",
		// 		},
		// 		StageItem {
		// 			title: "Data Science",
		// 			description: "Design the calculations to create the numbers for betting and bet returns.",
		// 		},
		// 		StageItem { title: "Implement Game on Site", description: "Add a page for playing the game to the site." },
		// 	],
		// },
		// RoadmapStage {
		// 	title: "Meme Poker",
		// 	description: "Use the betting on a meme's percentile to create a meme texas holdem.",
		// 	items: vec![
		// 		StageItem {
		// 			title: "Showdown Value",
		// 			description: "Defined as the total returns of the best 5 of 7 meme bets in a hand.",
		// 		},
		// 		StageItem {
		// 			title: "Initial System",
		// 			description: "Start by implement standard poker, then layer on the memes.",
		// 		},
		// 	],
		// },
		// RoadmapStage {
		// 	title: "Meme Marketing Tournaments",
		// 	description: "Allow Companies to create tournaments for purposed marketing memes",
		// 	items: vec![
		// 		StageItem {
		// 			title: "Showdown Value",
		// 			description: "Defined as the total returns of the best 5 of 7 meme bets in a hand.",
		// 		},
		// 		StageItem {
		// 			title: "Initial System",
		// 			description: "Start by implement standard poker, then layer on the memes.",
		// 		},
		// 	],
		// },
		// RoadmapStage {
		// 	title: "The truest Meme Coin",
		// 	description: "Design a crypto token based on the memetopia ecosystem",
		// 	items: vec![
		// 		StageItem {
		// 			title: "Creation",
		// 			description: "buy for meme poker or torunament creation. Cannot withdraw to chain ",
		// 		},
		// 		StageItem { title: "Sinks", description: "Rake and tournaments" },
		// 		StageItem { title: "Distribution", description: "poker winnings, profit share of to tournament winners" },
		// 	],
		// },
	]
});

#[component]
pub fn Home() -> Element {
	rsx! {
		div { class: "max-w-4xl mx-auto p-6 md:p-8 bg-slate-900 bg-gradient-to-br from-slate-900 via-zinc-900 to-black min-h-screen font-sans text-slate-200",
			div { class: "text-center mb-12",
				h1 { class: "text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-600 mb-3",
					"Memetopia Roadmap"
				}
				p { class: "text-slate-400 text-lg",
					"Forging the future meme site for the memers by the memers."
				}
			}
			div { class: "space-y-10",
				for (idx , item) in ROADMAP_DATA.iter().enumerate() {
					div {
						key: "{idx}",
						class: "bg-slate-800/50 backdrop-blur-xl rounded-xl border border-white/10 p-6",
						div { class: "border-b border-white/10 pb-4 mb-4",
							h3 { class: "text-2xl font-semibold text-slate-100 mb-1",
								"{item.title}"
							}
							p { class: "text-slate-300 mb-3", "{item.description}" }
						}
						div { class: "space-y-3",
							for (idx , sub_item) in item.items.iter().enumerate() {
								div {
									key: "{idx}",
									class: "flex items-start space-x-3",
									div {
										p { class: "font-medium text-slate-200", "{sub_item.title}" }
										p { class: "text-sm text-slate-400", "{sub_item.description}" }
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
