use dioxus::prelude::*;

use crate::components::ui::{
    cardContent::CardContent,
    disclaimer::Disclaimer,
    homehero::Hero
};

#[component]
pub fn Home() -> Element {

  let message = "Rust 🦀 is a modern systems language focused on safety, speed, and concurrency, offering low-level control without a garbage collector. Its unique ownership model enforces memory safety, preventing bugs like null pointer dereferencing and data races at compile time. Rust is ideal for performance-critical tasks like OS development, game engines 🎮, and web servers 🌐, combining C++-like control with safety and concurrency features, making it popular among developers.".to_string();
  let whyMessage = "I want to learn new programming languages to stay updated with technology and improve my knowledge and skills in programming. I'm exploring the possibilities of using Rust to build Front-End applications, aiming to develop new skills and create opportunities for myself to step into a world I've never known before. I may not be an expert in this, but I will try my best and share what I know as much as possible.".to_string();

  rsx! {
    div {
      style: "--dimension: 100px; --color: #7741725e;",
      class: "home-container",
      Hero {},
      Disclaimer {},
      CardContent {
        title: "What about Rust?".to_string(),
        content: "{message}",
        image: "/images/rust-to-wasm.webp".to_string(),
        order: "",
        "wachiii"
      },
      CardContent {
        title: "Why is Rust?".to_string(),
        content: "{whyMessage}",
        image: "https://images.squarespace-cdn.com/content/v1/634c5dc8c297a25f068e0d65/ea2754af-4c79-494c-a7b0-aaf5f78ff039/Learn+Rust+in+30+Minutes.jpg?format=2500w".to_string(),
        order: "order-b".to_string(),
      },
      CardContent {
        title: "Which Dioxus framework!".to_string(),
        content: "I'm using a framework called `Dioxus` to develop this website because Dioxus is a portable, high-performance, and user-friendly framework for building cross-platform user interfaces 🌐, such as websites, desktops 🖥️, mobile apps 📱, and more. It offers a familiar environment, similar to ReactJS ⚛️, but is written in Rust.".to_string(),
        image: "https://dioxuslabs.com/static/multiplatform-light.svg".to_string(),
        order: "".to_string(),
      },
      div {
        class: "action-container",
        "HELLO RUST"
        // h3 {
        //   "My action plan for this"
        // },
        // ol {
        //   style: "margin: 1rem;",
        //   li {
        //     "Build personal full-stack website with Rust. ✅"
        //   }
        //   li {
        //     "Deploy full-stack website to firebase. 🔎"
        //   }
        //   li {
        //     "Create markdown editor for this website. 🚧"
        //   }
        //   li {
        //     "Create a blog system for this website. 🚧"
        //   },
        //   li {
        //     "Share some interests feature of this website. 🚧"
        //   }
        // },
        // "hello"
      },
    }
  }
}
