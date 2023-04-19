use maud::{html, Markup, Render, DOCTYPE};

pub struct BaseTemplate {
	pub title: String,
	pub content: Markup,
}

impl Render for BaseTemplate {
	fn render(&self) -> Markup {
		html! {
			(DOCTYPE)
			html {
				head {
					title { "admin | " (self.title) }
				}
				body {
					h1 { (self.title) }
					(self.content)
				}
			}
		}
	}
}
