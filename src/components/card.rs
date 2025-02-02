use dioxus::prelude::*;

#[component]
pub fn Card(title: String) -> Element {
    rsx! {
        h2 { "{title}" }
    }
}
