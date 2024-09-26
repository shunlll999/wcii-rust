use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Blogs(id: i32) -> Element {
    rsx! {
        div {
            style: "--dimension: 100px; --color: #7741725e;",
            class: "blogs-container",
            h1 { "Blogs {id}" }
            Link { to: Route::Home {}, "Go to counter" },
        }
    }
}
