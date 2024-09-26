use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn FooterNavigationBar() -> Element {
    rsx! {
        div {
            class: "footer-bar-container",
            div { class: "logo footer" },
            div {
              "This is a complete Personal website. It does exactly built by Rust (🦀). Thank you to visited! 😇"
            },
            ul {
              class: "menu-panel",
              li {
                a {
                  href: Route::Home {}.to_string(),
                  "Home"
                }
              },
              li {
                a {
                  href: Route::Blogs { id: 0 }.to_string(),
                  "Blogs"
                }
              },
              li {
                a {
                  href: Route::Blogs { id: 0 }.to_string(),
                  "Ports"
                }
              },
              li {
                a {
                  href: Route::Blogs { id: 0 }.to_string(),
                  "What am I?"
                }
              },
            }
            div {
              class: "footer-bar-tools",
              div {
                class: "footer-caption-bar",
                "This website base develop by Rust and framwork called Dioxus"
              },
              div {
                class: "footer-logo",
                img {
                  class: "footer-rust-logo",
                  src: "/logos/rust-logo.png",
                  alt: "Rust Logo",
                },
                img {
                  class: "footer-dioxus-logo",
                  src: "/header.svg",
                  alt: "Dioxus Logo",
                }
              }
            }
        }
    }
}
