use crate::components::{Card, Hero};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h2 { "Functional Components" }

        Card { title: "Star Wars" }
        Card { title: "Avatar" }
        Card { title: "The Lion King" }
        Card { title: "Card 4" }
        Card { title: "Card 5" }
    }
}
